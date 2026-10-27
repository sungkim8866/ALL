# 영상 분석 (claude-watch-kr) — 설치. 새 컴퓨터에서 한 번만.
# 이 폴더째로 .claude\skills\ 아래에 복사한 뒤 INSTALL.bat 더블클릭.
# 없는 것만 받고, 있으면 건너뛴다. 관리자 권한은 필요 없다.

$ErrorActionPreference = "Continue"
$BASE = $PSScriptRoot

function Say($m, $c = "Gray") { Write-Host $m -ForegroundColor $c }
function Have($n) { return [bool](Get-Command $n -ErrorAction SilentlyContinue) }

Say "설치 위치: $BASE" Cyan
Say ""

# --- 0. 파일이 다 있는지 ---
foreach ($f in @("watch.ps1", "whisper_run.py", "SKILL.md")) {
  if (-not (Test-Path (Join-Path $BASE $f))) {
    Say "$f 이 없습니다. 폴더를 통째로 복사하셔야 합니다." Red
    Read-Host "엔터"; exit 1
  }
}

# --- 1. ffmpeg ---
$ff = $null
if (Have "ffmpeg") {
  $ff = (Get-Command ffmpeg).Source
} else {
  $r = Get-ChildItem "$env:LOCALAPPDATA\Microsoft\WinGet\Packages" -Recurse -Filter "ffmpeg.exe" -ErrorAction SilentlyContinue | Select-Object -First 1
  if ($r) { $ff = $r.FullName }
}
if (-not $ff) {
  if (-not (Have "winget")) {
    Say "winget 이 없습니다. 스토어에서 '앱 설치 관리자' 를 먼저 설치하세요." Red
    Read-Host "엔터"; exit 1
  }
  Say "1/6 ffmpeg 설치 중 (몇 분)..." Yellow
  winget install --id Gyan.FFmpeg -e --source winget --accept-package-agreements --accept-source-agreements
  $r = Get-ChildItem "$env:LOCALAPPDATA\Microsoft\WinGet\Packages" -Recurse -Filter "ffmpeg.exe" -ErrorAction SilentlyContinue | Select-Object -First 1
  if ($r) { $ff = $r.FullName }
}
if ($ff) { Say ("1/6 ffmpeg OK — " + $ff) Green } else { Say "1/6 ffmpeg 실패. winget install Gyan.FFmpeg 를 직접 돌려보세요." Red }

# --- 2. python ---
# 새 Windows 에는 python.exe 가 '스토어 바로가기' 로만 있다. 이름만 보면 있는 줄 아니 버전이 실제로 나오는지 본다.
function HavePython {
  if (-not (Have "python")) { return $false }
  return ((& python --version 2>&1 | Out-String) -match "Python 3\.")
}
if (-not (HavePython)) {
  Say "2/6 python 설치 중..." Yellow
  winget install --id Python.Python.3.12 -e --source winget --accept-package-agreements --accept-source-agreements
  $env:Path = [Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [Environment]::GetEnvironmentVariable("Path", "User")
}
if (-not (HavePython)) {
  # 깔렸는데 PATH 에 안 들어간 경우 — 설치 폴더를 찾아 사용자 PATH 앞에 넣는다.
  $py = Get-ChildItem "$env:LOCALAPPDATA\Programs\Python\Python3*\python.exe" -ErrorAction SilentlyContinue |
        Sort-Object { [int]($_.Directory.Name -replace '\D', '') } -Descending | Select-Object -First 1
  if ($py) {
    $d = $py.DirectoryName
    [Environment]::SetEnvironmentVariable("Path", "$d;$d\Scripts;" + [Environment]::GetEnvironmentVariable("Path", "User"), "User")
    $env:Path = "$d;$d\Scripts;" + $env:Path
  }
}
if (-not (HavePython)) {
  Say "2/6 python 이 PATH 에 안 잡힙니다. 설정에서 '앱 실행 별칭' 을 검색해 python 두 개를 끄고, 이 창을 닫고 INSTALL.bat 을 다시 실행하세요." Red
  Read-Host "엔터"; exit 1
}
Say ("2/6 python OK — " + ((python --version) 2>&1)) Green

# --- 3. yt-dlp + faster-whisper ---
Say "3/6 yt-dlp / faster-whisper 설치 중 (몇 분)..." Yellow
python -m pip install -q --upgrade pip
python -m pip install -q --upgrade "yt-dlp[default]" faster-whisper
# 유튜브는 JS 런타임(deno)이 없으면 화질 일부가 빠진다. 따로 받아서 실패해도 나머지는 그대로 돈다.
python -m pip install -q --upgrade "yt-dlp[deno]"
$ytv = (python -m yt_dlp --version) 2>&1
Say ("3/6 yt-dlp " + $ytv) Green

# --- 4. whisper 모델 미리 받기 (약 1.5GB) ---
Say "4/6 음성인식 모델 받는 중 — 여기가 제일 오래 걸립니다 (10~20분). 창을 닫지 마세요." Yellow
$pre = Join-Path $env:TEMP "watch-prefetch.py"
Set-Content $pre 'from faster_whisper import WhisperModel; WhisperModel("medium", device="cpu", compute_type="int8"); print("model ok")' -Encoding ASCII
python $pre
Remove-Item $pre -Force -ErrorAction SilentlyContinue
Say "4/6 모델 준비됨 (캐시: %USERPROFILE%\.cache\huggingface)" Green

# --- 5. ps1 은 UTF-8 BOM 으로 (없으면 한글 주석 다음 줄이 조용히 안 돈다) ---
foreach ($f in Get-ChildItem $BASE -Filter *.ps1) {
  $b = [System.IO.File]::ReadAllBytes($f.FullName)
  if ($b.Length -lt 3 -or $b[0] -ne 0xEF -or $b[1] -ne 0xBB -or $b[2] -ne 0xBF) {
    $t = [System.IO.File]::ReadAllText($f.FullName)
    [System.IO.File]::WriteAllText($f.FullName, $t, (New-Object System.Text.UTF8Encoding $true))
    Say ("  BOM 추가: " + $f.Name) DarkGray
  }
}
Say "5/6 스크립트 인코딩 OK" Green

# --- 6. SKILL.md 안의 경로를 이 컴퓨터 것으로 ---
$sk = Join-Path $BASE "SKILL.md"
$txt = [System.IO.File]::ReadAllText($sk)
$new = [regex]::Replace($txt, '-File "[^"]*watch\.ps1"', ('-File "' + (Join-Path $BASE "watch.ps1") + '"'))
if ($new -ne $txt) {
  [System.IO.File]::WriteAllText($sk, $new, (New-Object System.Text.UTF8Encoding $false))
  Say "6/6 SKILL.md 경로 수정됨" Green
} else {
  Say "6/6 SKILL.md 경로 그대로 OK" Green
}

Say ""
Say "끝났습니다. Claude 앱을 껐다 켜면 스킬이 잡힙니다." Green
Say "세션에 영상 링크를 주고 '이거 분석해줘' 라고 하시면 됩니다." Green
Read-Host "엔터"
