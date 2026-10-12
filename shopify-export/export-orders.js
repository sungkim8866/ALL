// Formuland Shopify 주문 export 자동화
//
//   조건: appstle_1st  ship-protect  Date is Last 7 days  (config.js)
//   동작: Export → "50+ orders matching your search" → Export orders  (config.repeat 회 반복)
//
//   npm run export
//
const fs = require('fs');
const path = require('path');
const { chromium } = require('playwright');
const cfg = require('./config');

const HEADLESS = process.env.HEADLESS !== 'false';

function stamp() {
  return new Date().toISOString().replace(/[:.]/g, '-');
}

function buildOrdersUrl() {
  const params = new URLSearchParams({
    savedViewId: cfg.savedViewId,
    query: cfg.query,
    order: cfg.order,
    selectedColumns: cfg.selectedColumns.join(','),
  });
  return `https://admin.shopify.com/store/${cfg.store}/orders?${params.toString()}`;
}

async function loadAuthState() {
  const authPath = path.resolve(__dirname, cfg.authFile);
  if (process.env.SHOPIFY_STORAGE_STATE_B64) {
    fs.mkdirSync(path.dirname(authPath), { recursive: true });
    fs.writeFileSync(authPath, Buffer.from(process.env.SHOPIFY_STORAGE_STATE_B64, 'base64'));
  }
  if (!fs.existsSync(authPath)) {
    throw new Error(
      `로그인 세션 파일이 없습니다: ${authPath}\n` +
        '먼저 `npm run login` 으로 Shopify에 로그인해 세션을 저장하거나, ' +
        'SHOPIFY_STORAGE_STATE_B64 환경변수를 설정하세요.',
    );
  }
  return authPath;
}

async function snap(page, name) {
  const dir = path.resolve(__dirname, cfg.screenshotDir);
  fs.mkdirSync(dir, { recursive: true });
  const file = path.join(dir, `${stamp()}-${name}.png`);
  await page.screenshot({ path: file, fullPage: false }).catch(() => {});
  return file;
}

async function assertLoggedIn(page) {
  const url = page.url();
  if (/accounts\.shopify\.com|\/login|\/auth/.test(url)) {
    await snap(page, 'login-required');
    throw new Error(
      `Shopify 세션이 만료되었습니다 (현재 URL: ${url}).\n` +
        '`npm run login` 을 다시 실행해 세션을 갱신하세요.',
    );
  }
}

// Export 모달을 열고 "50+ orders matching your search" + CSV for Excel 선택 후 Export orders 클릭
async function runExport(page, round, downloadDir) {
  console.log(`\n[${round}] Export 버튼 클릭`);
  const exportBtn = page.getByRole('button', { name: /^Export$/ });
  await exportBtn.waitFor({ state: 'visible', timeout: 60_000 });
  await exportBtn.click();

  const dialog = page.getByRole('dialog').filter({ hasText: /Export orders/i });
  await dialog.waitFor({ state: 'visible', timeout: 30_000 });

  // Export 범위: "50+ orders matching your search" (숫자는 주문 수에 따라 달라지므로 정규식으로 매칭)
  const scope = dialog.getByLabel(cfg.exportScope);
  await scope.waitFor({ state: 'visible', timeout: 15_000 });
  await scope.check();
  console.log(`[${round}] 선택: ${await dialog.getByText(cfg.exportScope).first().innerText()}`);

  // Export as: CSV for Excel
  await dialog.getByLabel(cfg.exportFormat).check();

  await snap(page, `round${round}-modal`);

  // Shopify 는 주문 수가 적으면 바로 다운로드, 많으면 이메일로 CSV 링크를 보냅니다.
  // 두 경우 모두 처리: 다운로드 이벤트를 기다리되, 안 오면 "이메일 전송" 안내를 확인.
  const downloadPromise = page.waitForEvent('download', { timeout: 90_000 }).catch(() => null);

  await dialog.getByRole('button', { name: /^Export orders$/ }).click();
  console.log(`[${round}] Export orders 클릭`);

  const download = await downloadPromise;
  if (download) {
    fs.mkdirSync(downloadDir, { recursive: true });
    const suggested = download.suggestedFilename() || 'orders_export.csv';
    const target = path.join(downloadDir, `${stamp()}-round${round}-${suggested}`);
    await download.saveAs(target);
    console.log(`[${round}] 다운로드 완료: ${target}`);
    return { round, mode: 'download', file: target };
  }

  // 다운로드가 없으면 이메일로 전송된 것 — 안내 토스트/문구가 있는지 기록
  const emailed = await page
    .getByText(/email|이메일/i)
    .first()
    .innerText({ timeout: 5_000 })
    .catch(() => '');
  console.log(`[${round}] 직접 다운로드 없음 — Shopify 가 CSV 를 이메일로 발송했습니다. ${emailed}`);
  await snap(page, `round${round}-after-export`);
  return { round, mode: 'email', note: emailed };
}

(async () => {
  const authPath = await loadAuthState();
  const downloadDir = path.resolve(__dirname, cfg.downloadDir);
  fs.mkdirSync(downloadDir, { recursive: true });

  const browser = await chromium.launch({ headless: HEADLESS });
  const context = await browser.newContext({
    storageState: authPath,
    acceptDownloads: true,
    viewport: { width: 1600, height: 1000 },
  });
  const page = await context.newPage();

  const url = buildOrdersUrl();
  console.log(`주문 목록 열기:\n${url}`);
  await page.goto(url, { waitUntil: 'domcontentloaded', timeout: 90_000 });
  await page.waitForLoadState('networkidle', { timeout: 60_000 }).catch(() => {});
  await assertLoggedIn(page);

  // 검색/필터가 적용되었는지 확인 (검색창에 태그 + "Last 7 days" 가 보여야 함)
  await page.getByRole('button', { name: /^Export$/ }).waitFor({ timeout: 60_000 });
  const bodyText = await page.locator('body').innerText();
  const hasTags = bodyText.includes('appstle_1st') && bodyText.includes('ship-protect');
  const hasDate = /Last 7 days/i.test(bodyText);
  console.log(`필터 확인 → 태그: ${hasTags ? 'OK' : '없음'}, 날짜(Last 7 days): ${hasDate ? 'OK' : '없음'}`);
  await snap(page, 'filtered-orders');
  if (!hasTags || !hasDate) {
    throw new Error('검색 조건(appstle_1st ship-protect / Last 7 days)이 화면에 적용되지 않았습니다. 스크린샷을 확인하세요.');
  }

  const results = [];
  for (let round = 1; round <= cfg.repeat; round++) {
    results.push(await runExport(page, round, downloadDir));
    // 모달이 닫힐 때까지 잠시 대기 후 다음 반복
    await page.getByRole('dialog').waitFor({ state: 'hidden', timeout: 30_000 }).catch(() => {});
    await page.waitForTimeout(3_000);
  }

  // 세션이 갱신되었을 수 있으므로 저장
  await context.storageState({ path: authPath });
  await browser.close();

  console.log('\n===== 결과 =====');
  for (const r of results) {
    console.log(r.mode === 'download' ? `${r.round}회차: ${r.file}` : `${r.round}회차: 이메일 발송 ${r.note || ''}`);
  }
  fs.writeFileSync(path.join(downloadDir, 'last-run.json'), JSON.stringify({ at: new Date().toISOString(), results }, null, 2));
})().catch(async (e) => {
  console.error('\n실패:', e.message || e);
  process.exit(1);
});
