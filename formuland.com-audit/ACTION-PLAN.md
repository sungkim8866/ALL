# formuland.com SEO 실행 계획

**순서:** Critical(즉시) → High(1주 안) → Medium(1개월 안) → Low(백로그)

각 항목에는 다음이 적혀 있어요.
- **무엇을:** 고칠 내용
- **어디서:** 고칠 위치
- **담당:** 맡을 사람
- **확인 방법:** 고친 뒤 제대로 됐는지 확인하는 방법

근거는 `FULL-AUDIT-REPORT.md`와 `findings/*.md`에 있어요.

---

## 1단계: Critical (1주 차)

### C1. 잘못된 건강 정보 수정 또는 비공개
- **어디서:**
  - `/blogs/parenting-tips/best-organic-baby-formula-which-is-safest-for-your-child`
  - `/blogs/parenting-tips/what-to-feed-your-infant`
- **무엇을:**
  - "DHA/ARA는 가능하면 피하라" 문단을 삭제하거나 바로잡으세요. DHA는 EU 2016/127 규정상 필수 성분이고, 자사 HiPP·Holle 상품 페이지와도 일치해야 해요.
  - 판매하지 않는 Jovie 추천 문단을 삭제하세요.
  - "알레르기 유발 식품은 피하라"는 옛날 조언을 최신 지침에 맞게 고치세요.
  - 전문가 감수를 받기 전까지는 글을 임시로 비공개하는 것도 방법이에요.
- **담당:** 콘텐츠 담당. 소아과의사 또는 영양사 감수가 필요해요.
- **확인 방법:** 페이지 원문에서 "Avoid these whenever possible"과 "Jovie"를 검색했을 때 0건이어야 해요.

### C2. 상품 페이지의 공통 수유량 표 삭제
- **어디서:** 상품 템플릿 섹션. HiPP, Holle, Lebenswert 상품에 공통으로 들어가 있어요.
- **무엇을:** 제조사 공식 수유량 표만 남기세요.
- **담당:** Shopify 테마 개발자
- **확인 방법:** `/products/hipp-formula-stage-1`에 수유량 표가 하나만 보이고, 0~2주 "2 scoops / 60 ml"로 나와야 해요.

### C3. 홈 첫 화면 영상 교체
- **무엇을:**
  - H1·설명·버튼 오버레이가 처음부터 보이게 하세요(opacity:1, 영상 종료를 기다리지 않게).
  - 포스터 이미지를 넣어서, 이 이미지가 LCP 요소가 되게 하세요.
  - 모바일용 영상을 2MB 이하로 만들고 faststart(moov를 파일 앞으로)로 인코딩하세요. 아니면 모바일에서는 영상 대신 이미지만 써도 돼요.
  - 버튼 링크를 `/collections/organic-baby-formula-temp`에서 `/collections/organic-baby-formula`로 바꾸세요.
- **담당:** 테마 개발자, 디자이너
- **확인 방법:** Chrome DevTools에서 모바일 + Slow 4G로 측정해서 H1이 2.5초 안에 보이고 LCP가 4초 미만이어야 해요. 홈페이지 전체 전송량도 5MB 미만이어야 해요.

### C4. 배송·관세·반품 안내 하나로 통일
- **무엇을:**
  - 아래 내용을 문서 하나로 정하세요.
    - 무료배송 기준 금액
    - 배송 기간
    - 출고지(유럽 / 미국 허브)
    - 관세 부담 주체
    - 반품 조건
  - 이 내용을 다음 모든 곳에 똑같이 적용하세요.
    - 상품 페이지 본문과 고지문
    - 컬렉션 페이지
    - 오래된 팝업("California warehouse", 2017년 날짜가 있는 것)
    - `/policies/refund-policy`
    - Google Merchant Center 설정
- **담당:** 운영, 테마 개발자
- **확인 방법:** 대표 상품 3개, 컬렉션 3개, 정책 페이지에 같은 숫자가 나와야 해요.

### C5. Cloudflare 봇 차단 설정 확인
- **무엇을:** Shopify/Cloudflare 보안 이벤트를 확인하세요. 인증된 검색·AI 봇이 예외인지 확인하세요.
- **담당:** 운영(Shopify 지원팀에 문의)
- **확인 방법:** Search Console 크롤링 통계에서 4xx 비율을 보세요. URL 검사의 실시간 테스트로 블로그 글 3개를 확인하세요.

---

## 2단계: High (2~3주 차)

| # | 무엇을 | 어디서 | 담당 | 확인 방법 |
|---|---|---|---|---|
| H1 | canonical 태그를 조건 없이 항상 출력 | `theme.liquid` 또는 상품 템플릿 | 개발자 | canonical이 빠졌던 상품 3개(예: HiPP Baby Soft, Weleda, LittleOak)의 원본 HTML에 canonical이 있어야 함 |
| H2 | 고지문 제목 H1 → H2 | 상품 템플릿 | 개발자 | 상품 페이지 H1이 1개 |
| H3 | H1과 메타 설명 추가 | about-us, nara 리콜, contact, FAQ 페이지 | 콘텐츠 | 각 페이지 H1 1개, 메타 설명 140~155자 |
| H4 | 회사 소개 페이지 통합 | `/pages/our-company` → `/pages/about-us` (301 리다이렉트) | 콘텐츠, 운영 | 숫자(주문 수, 설립 연도, 본사 위치), 주소, 창업자 정보를 한 세트로 통일 |
| H5 | 저자·감수자 체계 시작 | 블로그 템플릿, 저자 페이지 | 콘텐츠, 전문가 | 건강 관련 글 상위 20개에 "Medically reviewed by 이름, 자격, 날짜" 표시 |
| H6 | 상품 JSON-LD 정리 | 상품 템플릿 (`findings/schema.md`의 붙여넣기 코드) | 개발자 | Product 하나로 통합, BreadcrumbList·gtin13·shippingDetails 추가, C4 후 MerchantReturnPolicy 추가. Rich Results Test 통과 |
| H7 | 비교표를 HTML 표로 다시 만들기 | `/pages/european-baby-formula-comparison-chart` | 콘텐츠, 개발자 | 브랜드·원유(소/산양)·단계·주요 성분·가격 열, 상품 링크, 작성·감수 날짜 포함. 원본 HTML 본문 1,000단어 이상 |
| H8 | JavaScript 줄이기 | GTM 컨테이너, Appstle 등 앱 embed | 마케팅, 개발자 | 쓰지 않는 태그 삭제, 필요 없는 템플릿에서 앱 embed 끄기. 홈 JS 압축 용량 1.5MB 미만 |
| H9 | 메타 설명 다시 쓰기 | 주요 컬렉션 20개, 상품 상위 50개, 많이 읽히는 글 30개 | 콘텐츠 | 160자 초과 비율 50% 미만 |

---

## 3단계: Medium (2개월 차, 콘텐츠·권위)

- **M1. 새 페이지 "Is European baby formula legal and safe in the US?"**
  - FDA 상태, 수입, 보관과 조제 방법을 다루세요.
  - 상품·컬렉션·홈에서 이 페이지로 링크하세요.
- **M2. "european baby formula" 대표 가이드(필러) 만들기**
  - 홈 첫 화면 아래에 "New to European formula?" 안내 모듈을 넣으세요.
- **M3. HiPP Stage 1 허브 만들기**
  - `/collections/hipp-formula-stage-1`을 확장하세요.
  - 상품 페이지에 UK/독일/네덜란드 버전 전환 링크를 넣으세요.
- **M4. 산양유 컬렉션에 브랜드 비교표 추가**
- **M5. 상품 설명 다시 쓰기**
  - Holle와 Lebenswert 상품 설명을 실제 정보로 다시 쓰세요.
  - 근거 없는 효능 주장(난독증 등)은 삭제하세요.
  - 반복되는 법적 고지 블록은 줄이세요.
- **M6. 오래된 블로그 글 정리**
  - 출처를 달아 업데이트하거나, 통합·삭제하세요(301).
  - "Our site has a new look", "Now shipping to Canada" 같은 공지성 글도 포함이에요.
- **M7. 색인 정리**
  - customer-portal, quiz-results 같은 유틸 페이지에 `seo.hidden`을 설정하세요.
  - Avada HTML 사이트맵 6개는 삭제하고 앱도 제거하세요.
- **M8. 제목(title) 템플릿 정리**
  - 50~60자로 맞추세요: 핵심 키워드 + 차별점 + 브랜드.
  - 홈 제목의 키워드 반복을 없애세요.
- **M9. 내부 링크 보강**
  - Stage 컬렉션(1~4), Kendamil organic 컬렉션을 메뉴·이동 경로(breadcrumb)·관련 글에서 링크하세요.
- **M10. 이미지 최적화**
  - 국기 28개를 지연 로드하세요.
  - 히어로 이미지는 `<picture>` 하나로 화면 크기별로 하나씩만 받게 하세요.
  - 링크로 감싼 썸네일에 alt를 넣으세요.
- **M11. Organization 정보 정리**
  - Organization 데이터를 하나로 합치세요.
  - 법인명, 주소, 창업자, sameAs를 넣으세요.
- **M12. 모바일 팝업 조정**
  - 적립금 팝업은 몇 초 뒤에 뜨게 하거나 인라인 배너로 바꾸세요.

---

## 4단계: Low / 지속 관리

- **robots.txt:** AI 봇 그룹에도 Disallow 규칙을 복사하세요. 학습(training) 허용 여부도 정하세요.
- **hreflang:** Markets 태그가 있는 페이지에서 테마의 중복 x-default를 빼세요.
- **Shopify에 보고할 것:**
  - WebMCP `add_to_cart` 설명 문구("사용자에게 확인을 구하지 말라")
  - agents.md에 적힌 shop.com / shop.app 링크 불일치
- **측정 연결:** Search Console, CrUX, GA4를 claude-seo에 연결해서 실제 사용자 데이터로 다시 측정하세요.
- **변경 추적:** drift 기준선을 저장해 두고, 배포할 때마다 바뀐 점을 비교하세요.
- **이번에 못 본 부분 재감사:** 차단된 블로그 글 32개, 해외 스토어, 캐나다 사이트
- **로컬에서 추가 측정:** Lighthouse 에이전트 브라우징과 axe 접근성 검사를 실행하세요.
