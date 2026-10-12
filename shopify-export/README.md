# Formuland Shopify 주문 주간 자동 Export

매주 월요일에 아래 작업을 자동으로 수행합니다.

1. `admin.shopify.com/store/formuland/orders` 접속
2. 검색 조건 적용: `appstle_1st` `ship-protect` + `Date is Last 7 days`
3. **Export** → **50+ orders matching your search** → **CSV for Excel** → **Export orders**
4. 위 export 를 한 번 더 반복 (총 2회, `config.js` 의 `repeat`)

> Shopify 는 주문 수가 적으면 CSV 를 바로 다운로드하고, 많으면 계정 이메일로 CSV 링크를 보냅니다.
> 스크립트는 두 경우 모두 처리하며, 다운로드된 파일은 `downloads/` 에, 각 단계 스크린샷은 `screenshots/` 에 저장됩니다.

## 1. 최초 1회: 로그인 세션 저장

Shopify 로그인에는 2단계 인증이 필요하므로, 한 번만 직접 로그인해 세션(쿠키)을 저장합니다.

```bash
cd shopify-export
npm install
npx playwright install chromium
npm run login
```

브라우저가 열리면 Shopify 에 로그인하세요. 주문 목록이 보이면 자동으로 `auth/shopify-state.json` 에 저장되고,
터미널에 **base64 문자열**이 출력됩니다 (GitHub Actions 용).

## 2. 수동 실행 (내 컴퓨터에서)

```bash
npm run export            # 헤드리스 실행
HEADLESS=false npm run export   # 브라우저 보면서 실행
```

## 3. 매주 월요일 자동 실행 (GitHub Actions)

1. GitHub 리포 → **Settings → Secrets and variables → Actions → New repository secret**
2. 이름 `SHOPIFY_STORAGE_STATE_B64`, 값에 `npm run login` 이 출력한 base64 문자열 붙여넣기
3. `.github/workflows/shopify-weekly-export.yml` 이 매주 **월요일 09:00 (미국 태평양, PDT)** 에 실행됩니다.
   - 시간을 바꾸려면 workflow 파일의 `cron` 수정 (UTC 기준)
   - **Actions** 탭 → 해당 workflow → **Run workflow** 로 즉시 테스트 가능
4. 실행 결과(CSV, 스크린샷)는 Actions 실행 페이지의 **Artifacts** 에서 다운로드. 이메일로 발송된 경우 Shopify 계정 이메일을 확인하세요.

### 내 컴퓨터에서 cron 으로 돌리고 싶다면

```cron
# 매주 월요일 09:00
0 9 * * 1  cd /path/to/ALL/shopify-export && npm run export >> export.log 2>&1
```

## 세션 만료 시

실행 로그에 `Shopify 세션이 만료되었습니다` 가 나오면 `npm run login` 을 다시 실행하고
GitHub Secret 값을 새 base64 문자열로 교체하세요.

## 조건 변경

`config.js` 에서 태그 / 날짜 조건 / 반복 횟수 / 저장된 보기 ID 를 수정합니다.

## 이미 크롬에 로그인되어 있다면 (브라우저 쿠키 가져오기)

`npm run login` 대신, 크롬 확장 **Get cookies.txt LOCALLY** 로 `admin.shopify.com` 페이지에서 쿠키를 내보낸 뒤:

```bash
node import-cookies.js ~/Downloads/admin.shopify.com_cookies.txt
```

출력되는 base64 값을 GitHub Secret `SHOPIFY_STORAGE_STATE_B64` 에 넣으면 됩니다.
