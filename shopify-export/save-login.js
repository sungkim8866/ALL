// 1회용: 브라우저를 띄워서 직접 Shopify에 로그인하면 세션(쿠키)을 auth/ 에 저장합니다.
// 이후 export-orders.js 는 이 세션을 재사용하므로 매번 로그인/2단계 인증이 필요 없습니다.
//
//   npm run login
//
const fs = require('fs');
const path = require('path');
const { chromium } = require('playwright');
const cfg = require('./config');

(async () => {
  const authPath = path.resolve(__dirname, cfg.authFile);
  fs.mkdirSync(path.dirname(authPath), { recursive: true });

  const browser = await chromium.launch({ headless: false });
  const context = await browser.newContext();
  const page = await context.newPage();

  const ordersUrl = `https://admin.shopify.com/store/${cfg.store}/orders`;
  await page.goto(ordersUrl);

  console.log('\n브라우저에서 Shopify 로그인(이메일/비밀번호/2단계 인증)을 완료해 주세요.');
  console.log('주문(Orders) 목록 화면이 보이면 자동으로 세션을 저장합니다...\n');

  // 주문 목록 페이지가 로드될 때까지(= 로그인 완료) 최대 10분 대기
  await page.waitForURL(
    (u) => u.hostname === 'admin.shopify.com' && u.pathname.includes(`/store/${cfg.store}/orders`),
    { timeout: 10 * 60 * 1000 },
  );
  await page.getByRole('button', { name: /^Export$/ }).waitFor({ timeout: 60_000 });

  await context.storageState({ path: authPath });
  console.log(`세션 저장 완료: ${authPath}`);
  console.log('GitHub Actions 에서 쓰려면 아래 값을 SHOPIFY_STORAGE_STATE_B64 시크릿에 넣으세요:\n');
  console.log(fs.readFileSync(authPath).toString('base64'));

  await browser.close();
})().catch((e) => {
  console.error(e);
  process.exit(1);
});
