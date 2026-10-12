// 브라우저에서 내보낸 쿠키(Netscape cookies.txt 또는 JSON 배열)를 Playwright 세션 파일로 변환합니다.
// 크롬 확장 "Get cookies.txt LOCALLY" 등으로 admin.shopify.com / accounts.shopify.com 쿠키를 내보낸 뒤:
//
//   node import-cookies.js ~/Downloads/admin.shopify.com_cookies.txt
//
const fs = require('fs');
const path = require('path');
const cfg = require('./config');

const src = process.argv[2];
if (!src) {
  console.error('사용법: node import-cookies.js <cookies.txt 또는 cookies.json>');
  process.exit(1);
}
const raw = fs.readFileSync(src, 'utf8');
let cookies = [];

if (raw.trim().startsWith('[')) {
  cookies = JSON.parse(raw).map((c) => ({
    name: c.name,
    value: c.value,
    domain: c.domain,
    path: c.path || '/',
    expires: typeof c.expirationDate === 'number' ? Math.floor(c.expirationDate) : c.expires ?? -1,
    httpOnly: !!c.httpOnly,
    secure: !!c.secure,
    sameSite: ['Strict', 'Lax', 'None'].includes(c.sameSite) ? c.sameSite : 'Lax',
  }));
} else {
  for (const line of raw.split('\n')) {
    if (!line.trim() || line.startsWith('#')) continue;
    const [domain, , p, secure, expires, name, value] = line.split('\t');
    if (!name) continue;
    cookies.push({
      name,
      value: (value || '').trim(),
      domain,
      path: p || '/',
      expires: Number(expires) || -1,
      httpOnly: line.startsWith('#HttpOnly_'),
      secure: secure === 'TRUE',
      sameSite: 'Lax',
    });
  }
}

cookies = cookies.filter((c) => /shopify\.com$/.test(c.domain.replace(/^\./, '')) || /shopify/.test(c.domain));
if (!cookies.length) {
  console.error('shopify.com 쿠키를 찾지 못했습니다.');
  process.exit(1);
}

const authPath = path.resolve(__dirname, cfg.authFile);
fs.mkdirSync(path.dirname(authPath), { recursive: true });
fs.writeFileSync(authPath, JSON.stringify({ cookies, origins: [] }, null, 2));
console.log(`세션 저장 완료 (${cookies.length}개 쿠키): ${authPath}`);
console.log('\nGitHub Secret SHOPIFY_STORAGE_STATE_B64 값:\n');
console.log(fs.readFileSync(authPath).toString('base64'));
