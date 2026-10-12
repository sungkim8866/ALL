// 매주 월요일 실행할 Shopify 주문 export 조건.
// 조건을 바꾸려면 이 파일만 수정하면 됩니다.
module.exports = {
  store: 'formuland',

  // 저장된 보기(saved view) ID — 스크린샷의 URL에서 가져옴
  savedViewId: '185134711003',

  // 검색창에 들어가는 검색어 (태그) + 날짜 조건
  //   appstle_1st  ship-protect  Date is Last 7 days
  query: 'appstle_1st ship-protect processed_at:"past_week"',

  order: 'processed_at desc',

  selectedColumns: [
    'ORDER_DATE',
    'CUSTOMER_NAME',
    'DELIVERY_STATUS',
    'TOTAL_PRICE',
    'FINANCIAL_STATUS',
    'FULFILLMENT_STATUS',
    'ORDER_TAGS',
    'DELIVERY_METHOD',
  ],

  // Export 모달에서 선택할 항목
  //   "50+ orders matching your search" (검색 조건에 맞는 주문 전체)
  exportScope: /orders matching your search/i,
  //   "CSV for Excel, Numbers, or other spreadsheet programs"
  exportFormat: /CSV for Excel/i,

  // "다시 한번 반복" — 같은 export를 몇 번 실행할지
  repeat: 2,

  // 로그인 세션(쿠키) 저장 위치
  authFile: 'auth/shopify-state.json',
  downloadDir: 'downloads',
  screenshotDir: 'screenshots',
};
