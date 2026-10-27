# 영상 -> 프레임 이미지 + 한국어 대본 + 컷 목록
param(
  [Parameter(Mandatory=$true)][string]$Url,
  [string]$Model     = "medium",   # medium / large-v3
  [int]$MaxFrames    = 36,
  [string]$OutDir    = ""
)
$ErrorActionPreference = "Stop"
$SKILL = $PSScriptRoot
if (-not $OutDir) {
  $OutDir = Join-Path $env:TEMP ("watch-" + (Get-Date -Format "MMdd-HHmmss"))
}
New-Item -ItemType Directory -Force -Path $OutDir | Out-Null
$frames = Join-Path $OutDir "frames"
New-Item -ItemType Directory -Force -Path $frames | Out-Null

function Find-Exe($name) {
  $g = Get-Command $name -ErrorAction SilentlyContinue
  if ($g) { return $g.Source }
  $r = Get-ChildItem "$env:LOCALAPPDATA\Microsoft\WinGet\Packages" -Recurse -Filter "$name.exe" -ErrorAction SilentlyContinue |
       Select-Object -First 1
  if ($r) { return $r.FullName }
  throw "$name 을 찾을 수 없다. winget install Gyan.FFmpeg 부터 해라."
}
$FFMPEG  = Find-Exe "ffmpeg"
$FFPROBE = Find-Exe "ffprobe"

function Say($m) { "[watch] $m" }

# 1. 다운로드
Say "1/4 다운로드"
$mp4 = Join-Path $OutDir "video.mp4"
python -m yt_dlp -f "bv*[height<=720]+ba/b[height<=720]/b" --merge-output-format mp4 `
  --ffmpeg-location (Split-Path $FFMPEG) -o $mp4 --no-playlist --no-warnings --quiet $Url
if (-not (Test-Path $mp4)) { throw "다운로드 실패" }

# 길이
$dur = [double](& $FFPROBE -v error -show_entries format=duration -of csv=p=0 $mp4)
Say ("길이 {0:N1}초" -f $dur)

# 2. 프레임 (균등 샘플링, 파일명에 초를 박는다)
Say "2/4 프레임 추출"
$step = [math]::Max(0.5, [math]::Round($dur / $MaxFrames, 2))
& $FFMPEG -v error -i $mp4 -vf "fps=1/$step,scale=640:-2" -q:v 4 (Join-Path $frames "f_%03d.jpg")
$i = 0
Get-ChildItem $frames -Filter "f_*.jpg" | Sort-Object Name | ForEach-Object {
  $t = [math]::Round($i * $step, 1)
  $n = "{0:d3}_{1}s.jpg" -f $i, $t
  Rename-Item $_.FullName $n
  $i++
}
Say "$i 장"

# 2-2. 후킹 구간(첫 10초)은 1초 간격으로 따로
$hook = Join-Path $OutDir "hook"
New-Item -ItemType Directory -Force -Path $hook | Out-Null
& $FFMPEG -v error -t 10 -i $mp4 -vf "fps=1,scale=640:-2" -q:v 4 (Join-Path $hook "h_%02d.jpg")
$j = 0
Get-ChildItem $hook -Filter "h_*.jpg" | Sort-Object Name | ForEach-Object {
  Rename-Item $_.FullName ("hook_{0}s.jpg" -f $j); $j++
}
Say "후킹 $j 장"

# 3. 컷(장면 전환) 시각
Say "3/4 컷 감지"
$cutRaw = & $FFMPEG -v error -i $mp4 -vf "select='gt(scene,0.30)',metadata=print:file=-" -an -f null - 2>&1
$cuts = @()
foreach ($l in $cutRaw) { if ($l -match "pts_time:([0-9.]+)") { $cuts += [math]::Round([double]$Matches[1],1) } }
Say ($cuts.Count.ToString() + " 컷")

# 4. 음성 -> 한국어 대본
Say "4/4 음성 인식 ($Model) - 처음 한 번은 모델 받느라 오래 걸린다"
$wav = Join-Path $OutDir "audio.wav"
& $FFMPEG -v error -y -i $mp4 -ac 1 -ar 16000 -vn $wav
$txt = Join-Path $OutDir "transcript.txt"
python (Join-Path $SKILL "whisper_run.py") $wav $txt $Model

# 요약 파일
$sum = Join-Path $OutDir "INFO.txt"
$lines = @(
  "URL       $Url",
  ("길이      {0:N1}초" -f $dur),
  ("프레임    {0} 장 / {1}초 간격" -f $i, $step),
  ("컷 시각   " + ($cuts -join ", ")),
  "프레임    $frames",
  "후킹      $hook",
  "대본      $txt"
)
Set-Content $sum ($lines -join "`r`n") -Encoding utf8
$lines | ForEach-Object { $_ }
Say "완료 -> $OutDir"
