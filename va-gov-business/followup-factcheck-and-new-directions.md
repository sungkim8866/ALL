# 후속 보고서: 다른 대화 팩트체크와 새로운 기회

> **기준일:** 2026년 11월 8일
> **목적:** 다른 Claude 대화에서 받은 조언을 검증합니다. 대상은 직장 병행 6개월 테스트, 캘리포니아 DVBE, 장비 재판매, 위생·세정 소모품, 소프트웨어 아이디어 3가지입니다. 그리고 당신 상황에 맞는 새 방향을 정리합니다.
> **전제로 삼은 당신의 상황(대화에서 추정):** 캘리포니아 거주, 현재 직장 있음, 군 보급·구매 경력, 위생·개인관리·세정 소모품 OEM 소싱 경험, 한국어·영어 가능, 비개발자지만 Claude Code로 자동화를 만들어 본 경험 있음.
> **주의:** 법률·세무 자문이 아닙니다. 인증과 계약 결정 전에 APEX Accelerator, DGS, 변호사와 확인하세요.

첫 보고서는 [README.md](README.md)에 있습니다.

---

## 0. 한눈에 보기

### 가장 중요한 정정 세 가지

1. **"직장 다니면서 SDVOSB로 테스트"는 그대로 하면 위험합니다.**
   - SBA 규정(13 CFR 128.203(i))은 상이군인이 회사의 정상 영업시간 동안 풀타임으로 일할 것을 요구합니다. 다른 직장 때문에 회사를 통제할 시간이 부족해서도 안 됩니다.
   - 근무시간이 더 짧으면 SBA는 **통제하지 않는 것으로 간주**하고, 반대 증거는 회사가 내야 합니다.
   - 실제로 상이군인이 다른 직장을 가진 회사가 SBA 심판(OHA)에서 요건 충족을 입증하지 못한 사례가 있습니다.
   - **해결책:** 직장에 다니는 동안에는 SDVOSB 인증이 필요 없는 길로 시험해 보세요. SAM.gov 일반 소기업 등록, 캘리포니아 SB 인증 검토, 군 PX·VA 매점 납품 등이 있습니다. VetCert와 DVBE는 풀타임으로 전환할 때 신청하세요.
2. **위생·세정 소모품은 정부의 '의무 구매처'와 정면으로 부딪힙니다.**
   - 연방기관은 **AbilityOne**(SKILCRAFT 브랜드, 3,000개 이상 품목, 청소·위생·개인관리 포함)에서 먼저 사야 합니다.
   - 캘리포니아 주 기관은 **교도소 산업청 CALPIA**에서 먼저 사야 합니다. 대상에는 비누, 청소·세탁·주방용 화학제품, Green Seal 인증 세제 등이 들어 있습니다.
   - 이 품목들은 기관이 면제를 받지 않는 한 일반 업체가 팔기 어렵습니다.
   - **해결책:** 의무 구매처가 적용되지 않는 곳으로 방향을 바꾸세요. **카운티·시·학군 같은 지방정부**, 목록에 없는 품목, **군 PX·VA 매점 같은 소매 채널**이 있습니다.
3. **소프트웨어 아이디어 3개 중 2개는 이미 경쟁이 치열합니다.**
   - AI 입찰 도우미 분야에는 투자받은 회사가 넘칩니다. GovDash($3,000만, 2026년 1월), NationGraph($1,800만, 2026년 3월), Procurement Sciences, Cleatus, pWin.ai 등입니다.
   - DVBE 매칭은 주정부가 무료 도구(Cal eProcure 검색, CSCR 무료 광고)로 이미 제공합니다.

### 새로 찾은 기회 Top 5 (당신 상황 기준)

| 순위 | 기회 | 당신에게 맞는 이유 | 인증 필요? |
|---|---|---|---|
| 1 | 군 PX(AAFES·NEXCOM)와 VA 매점(VCS PatriotMart)에 재향군인 브랜드 개인관리 제품 납품 | OEM 소싱 경험을 그대로 씁니다. 의무 구매처·재판매 규칙이 적용되지 않고, 작게 시작할 수 있습니다 | 없음 |
| 2 | 캘리포니아 'SB/DVBE 옵션' 수의계약($5,000 초과~$250,000 미만, 견적 2개) | 공식 입찰 없이 주 기관이 바로 계약할 수 있는 가장 빠른 첫 실적 경로입니다 | 캘리포니아 SB 또는 DVBE |
| 3 | LA 카운티 DVBE 15% 가격 우대(최대 $15만) | 카운티는 연방·주 의무 구매처 대상이 아니라 소모품도 경쟁할 수 있습니다 | 카운티 인증 |
| 4 | 지방정부 위생·청소 소모품 | 품목 지식을 그대로 씁니다. 캘리포니아 기관은 친환경 인증을 따지는 경우가 많습니다 | 선택 |
| 5 | 한국어-영어 번역·통역 서비스 | 이중언어 능력을 살리고, 자본이 거의 필요 없습니다 | 선택 |

---

## 1. 다른 대화 내용 팩트체크

| 주장 | 판정 | 실제 |
|---|---|---|
| 과거 실적이 쌓이면 계속 이긴다 | ✅ 대체로 맞음 | 맞습니다. 다만 경쟁 협상 조달에서 실적이 없는 신규 업체는 '중립' 평가를 받도록 FAR에 규정돼 있습니다(FAR 15.305, RFO 개정으로 조항 위치가 바뀌었을 수 있음). 첫 계약이 불가능한 건 아닙니다. 소액 구매는 대부분 가격 위주로 결정됩니다 |
| 1년 기본 + 옵션 4년 구조라 몇 년간 매출이 이어진다 | ⚠️ 부분적 | 흔한 구조는 맞지만 옵션 행사는 정부의 권리이지 의무가 아닙니다. 정부는 편의를 이유로 계약을 해지할 수도 있습니다 |
| 정부는 돈 떼일 일이 없다 | ⚠️ 과장 | 지급은 대체로 안정적입니다(연방 Prompt Payment Act). 하지만 **2025년 10월 1일~11월 12일 43일 셧다운** 때 계약업체 대금이 밀렸습니다. 공무원과 달리 계약업체는 소급 지급이 보장되지 않습니다 |
| SDVOSB 우선배정은 연방정부 전체에 있다 | ✅ 맞음 | 연방 목표는 5%입니다. VA는 법적 의무이고, 다른 기관은 담당자 재량입니다 |
| 직장 다니면서 VetCert 받고 테스트하면 된다 | ❌ 위험 | 위 0번의 정정 1을 보세요(13 CFR 128.203(i) 풀타임 요건) |
| 재고 부담 없는 브랜드 지정 장비 재판매부터 (첫 답변) | ❌ 앞뒤가 안 맞음 | 같은 대화 뒤쪽에서 스스로 정정했습니다. SDVOSB 세트어사이드에는 금액과 상관없이 비제조업체 규칙이 적용되므로, 면제 없이 대기업 브랜드 장비를 공급할 수 없습니다. 다만 **일반 소기업 세트어사이드는 $1만 5,000~$35만 구간에서 이 규칙이 면제**됩니다. 인증 없이 일반 소기업으로는 가능하다는 뜻입니다 |
| SDVOSB 세트어사이드는 금액과 상관없이 재판매자 규칙이 적용된다 | ✅ 맞음 | 위 설명과 같습니다 |
| 델·키사이트 같은 제조사는 공인 리셀러를 요구하는 경우가 많다 | ✅ 대체로 맞음 | 보증과 위조품 방지 때문에 공인 유통 증빙을 요구하는 공고가 많습니다 |
| 캘리포니아는 3% DVBE 목표가 있고, 못 채우면 입찰이 무효 | ✅ 대체로 맞음 | DVBE 참여 요건이 있는 공고에서 요건이나 서류를 못 맞추면 부적격(nonresponsive) 처리됩니다. 다만 모든 공고에 요건이 있는 건 아닙니다. 인센티브 조건도 공고마다 다릅니다(예: 4% 이상 참여해야 인센티브) |
| DVBE를 참여시키면 평가 가격을 깎아준다 | ✅ 맞음 | DVBE 인센티브입니다 |
| CSU·주 법원·교육청 입찰에도 적용된다 | ⚠️ 부분적 | CSU, 주 법원, 주 교육부(CDE)에는 적용됩니다. '교육청'을 지역 학군으로 이해하면 대부분 해당하지 않습니다 |
| DVBE는 본사가 캘리포니아에 있어야 한다 | ❌ 틀림 | DVBE 요건은 **상이군인 본인이 캘리포니아에 거주(domiciled)** 하는 것이고, 회사 본사는 미국 안에만 있으면 됩니다. 본사가 캘리포니아여야 한다는 건 '캘리포니아 SB 인증' 쪽 요건입니다. 그리고 **LLC는 상이군인이 100% 소유해야 DVBE가 됩니다.** 이 내용이 빠져 있었습니다 |
| DVBE 인증은 DGS에서 따로, 무료 | ✅ 맞음 | |
| 이름만 빌려주면 안 된다 | ✅ 맞음 | 상업적으로 유용한 기능(CUF)을 실제로 수행해야 합니다 |
| 위생·세정·포장 소모품 2~3개 품목에 집중하라 | ⚠️ 중요한 누락 | AbilityOne(연방)과 CALPIA(주) 의무 구매처 확인이 빠졌습니다. 포장재는 상대적으로 덜 겹칩니다 |
| FedRAMP 20x: 2026년 6월 새 규정, 8월부터 접수 | ✅ 맞음 | |
| SBIR 2026년 4월 13일 재승인(2031년까지) | ✅ 맞음 | |
| DVBE 하도급 매칭 툴은 수요가 분명하다 | ⚠️ 약함 | 주정부가 같은 기능을 무료로 제공합니다(Cal eProcure SB/DVBE 검색, CSCR 무료 하도급 광고, Caltrans 입찰자 목록). 유료 매칭 서비스도 이미 있고, 명의대여 알선으로 오해받을 위험도 있습니다 |
| 재향군인 업체용 AI 입찰 도우미 | ❌ 경쟁 과열 | 투자받은 AI 조달 스타트업이 많습니다(4.1의 표 참고) |
| 규정 준수 관리 툴 | ⚠️ 가능하지만 시장이 작음 | 시장이 작고, 정부계약용 회계·ERP(Unanet, Deltek 등)가 일부 기능을 이미 제공합니다 |
| 의료정보를 안 다루니 보안 인증이 필요 없다 | ⚠️ 부분적 | FedRAMP는 필요 없지만, 고객 데이터를 다루면 SOC 2 같은 보안 요구는 받습니다 |

---

## 2. 첫 보고서에서 고칠 점

1. **DVBE용 회사 형태:** 첫 보고서에는 "LLC 설립, 본인 지분 51% 이상"이라고 썼습니다. 하지만 캘리포니아 DVBE까지 받으려면 **LLC는 상이군인이 100% 소유**해야 합니다(DGS 안내 기준). 51%만 소유하는 구조라면 주식회사(corporation) 형태가 필요합니다. → README에 반영했습니다.
2. **위생·청소 제품의 의무 구매처:** AbilityOne은 언급했지만 캘리포니아 CALPIA가 빠져 있었습니다. → README에 반영했습니다.
3. **비제조업체 규칙 예외:** 일반 소기업 세트어사이드는 $1만 5,000~$35만 구간에서 면제된다는 점을 추가했습니다. → README에 반영했습니다.

첫 보고서에 쓴 "다른 직장이 있으면 통제 요건이 문제될 수 있다"는 내용은 맞았습니다. 13 CFR 128.203(i)로 확인했습니다.

---

## 3. 새로 찾은 기회 상세

### 3.1 군 PX와 VA 매점: 재향군인 브랜드 개인관리 제품 (새 제안 1순위)

- **AAFES(육·공군 PX):** 소기업을 포함해 새 공급업체를 계속 찾고 있고, 재향군인·군 배우자 소유 기업을 위한 기회를 따로 안내합니다. 개인관리·생활용품도 구매 카테고리에 들어 있습니다. 2022년에는 재향군인이 소유한 면도용품 회사(Caisson Shaving)가 AAFES 매장에 입점했습니다.
- **VA 매점(Veterans Canteen Service, VCS):**
  - PatriotMart 프로그램을 통해 재향군인과 소기업이 VA 병원 안에서 직접 판매할 수 있습니다.
  - 지역 매점 책임자(Canteen Chief)에게 제품 목록, 가격, 사진을 보내면 되고, 재향군인 소유 기업을 우대한다고 안내합니다. 일부 안내 페이지가 오래돼서 현재 절차는 직접 확인해야 합니다.
  - 전국 단위 납품은 VCS 본부의 상품 담당자와 협상합니다.
- **왜 좋은가:**
  - 정부 '조달'이 아니라 '소매 입점'이라서 SDVOSB 인증, 비제조업체 규칙, AbilityOne·CALPIA 의무 구매처가 적용되지 않습니다.
  - 당신의 OEM 소싱, 성분, 포장 경험을 바로 쓸 수 있습니다.
  - 가까운 VA 병원 매점에서 작게 시험해 볼 수 있습니다.
- **주의할 점:**
  - 소매 마진, 재고, 반품 부담이 있습니다.
  - 제조물 책임보험이 필요합니다.
  - 화장품이면 FDA 규정(MoCRA: 시설 등록, 제품 리스팅 등)이 적용됩니다. 이미 아는 분야일 겁니다.
  - 현재 직장과 품목이나 공급처가 겹치면 이해충돌 문제가 생깁니다(6번 참고).

### 3.2 캘리포니아 SB/DVBE 옵션 (공식 입찰 없는 수의계약)

- 주 기관은 $5,000 초과 ~ $250,000 미만의 물품·서비스·IT 계약을 **인증받은 SB(마이크로비즈니스 포함) 또는 DVBE와 바로 맺을 수 있습니다.**
  - 조건은 인증 업체 2곳 이상에서 견적을 받는 것입니다. SB 견적과 DVBE 견적을 섞을 수는 없습니다.
  - 이 방식에는 이의제기(protest) 권한이 없습니다.
  - 금액 한도는 옵션 연도까지 포함한 전체 계약 기간 기준입니다.
- **의미:** 주정부에서 첫 실적을 만드는 가장 빠른 길입니다. 각 기관 구매 담당자에게 "우리는 SB/DVBE 옵션으로 계약할 수 있는 업체"라고 소개하는 방식으로 영업합니다.
- **캘리포니아 SB 인증**은 상이군인 요건이 아니라 독립 소유, 본사 캘리포니아, 매출 기준 위주입니다. 그래서 **직장에 다니는 동안에도 검토해 볼 만한 인증**입니다(세부 요건과 매출 기준은 DGS에서 확인).
- 단, CALPIA 품목과 주 차원의 의무 계약 품목은 제외됩니다.

### 3.3 LA 카운티 DVBE 15% 가격 우대

- LA 카운티는 인증받은 DVBE의 입찰 가격을 15% 깎아서 평가합니다(최대 $15만, 건설은 7%). 다른 우대 제도와는 중복되지 않습니다.
- 카운티 공공사업국은 연방 SDVOSB 인증으로도 자격을 인정한다고 안내합니다. 다만 옛 VA 인증 기준의 문구라서, 지금의 SBA 인증을 인정하는지는 확인이 필요합니다.
- 카운티는 연방(AbilityOne)이나 주(CALPIA) 의무 구매처 대상이 아니라서, 위생·청소 소모품으로도 경쟁할 수 있습니다.
- 사는 지역의 다른 카운티나 시에도 자체 우대 제도가 있는지 확인하세요.

### 3.4 지방정부 위생·청소 소모품 (의무 구매처가 없는 시장)

- 카운티, 시, 학군, 특별구(수도, 공원 등)는 연방 AbilityOne이나 주 CALPIA 의무 대상이 아닙니다.
- 캘리포니아 기관은 친환경 구매 기준(Green Seal 등)을 요구하는 경우가 많아서, 성분과 인증을 아는 사람에게 유리합니다.
- 연방 쪽은 2025년 FAR 전면 개정(RFO)으로 환경 요건이 많이 빠졌습니다. 하지만 **바이오기반 제품(USDA BioPreferred) 구매 의무는 법률에 근거한 것이라 유지됩니다.**

### 3.5 한국어-영어 번역·통역

- 캘리포니아 주·지방 기관은 Dymally-Alatorre 이중언어 서비스법에 따라, 일정 규모 이상인 비영어 사용 주민에게 번역·통역을 제공해야 합니다. 한국어는 LA와 오렌지카운티 등에서 수요가 큰 언어입니다.
- 자본이 거의 들지 않고, 실제로 일을 직접 하므로 CUF 요건을 쉽게 충족합니다. SB/DVBE 옵션으로 수의계약도 가능합니다.
- 단, 법원 통역과 의료 통역은 자격증이 필요합니다. 문서 번역부터 시작하세요.

### 3.6 (검증 필요) 보훈 노숙인 지원 비영리단체에 공급

- VA는 2026년 10월에 시작한 회계연도(FY2027)에 SSVF와 GPD 보조금 $11.7억을 680개 단체에 배정했습니다(SSVF $8.55억, GPD $3.18억).
- 이 단체들은 FAR이 아니라 보조금 규정에 따라 구매하므로 세트어사이드나 의무 구매처가 적용되지 않습니다.
- 다만 위생용품 구매 수요가 실제로 있는지는 인터뷰로 확인해야 합니다. 노숙 재향군인 지원 행사(스탠드다운)는 기부 물품에 많이 의존합니다.

---

## 4. 소프트웨어: 현실 점검과 대안

정부 소프트웨어가 바뀌고 있는 건 사실입니다. VA의 AI 사용 사례만 367건이고, FedRAMP 20x로 인증도 빨라지고 있습니다. 하지만 바뀌는 만큼 **투자받은 회사들이 빠르게 들어오고 있습니다.**

### 4.1 피해야 할 것: AI 입찰 도우미 (경쟁 과열)

| 회사 | 투자 유치 |
|---|---|
| GovDash | $3,000만 (2026년 1월) |
| Procurement Sciences (Awarded AI) | $3,000만 |
| NationGraph | $1,800만 (2026년 3월) |
| pWin.ai | $1,000만 |
| GovSignals | $550만 (시드) |
| Cleatus | $400만 (시드, 2026년 6월) |
| GovEagle | $250만 (2025년 3월) |

이 밖에도 HigherGov, Govly, SweetSpot, SamSearch, BidPulsar 같은 공고 검색·분석 사이트가 이번 조사 중에 계속 나왔습니다.

### 4.2 피해야 할 것: DVBE 매칭

주정부의 무료 도구(Cal eProcure SB/DVBE 검색, CSCR 무료 하도급 광고, Caltrans 입찰자 목록)와 유료 매칭 서비스가 이미 있습니다.

### 4.3 현실적인 소프트웨어 경로 (비개발자 + 직장인 기준)

1. **먼저 내 사업용 자동화부터 만드세요.** 이미 Claude Code로 Shopify 주문 자동 추출을 만들었습니다. 같은 방식으로 다음을 만들 수 있습니다:
   - 내 품목만 보는 입찰 모니터(Cal eProcure, SAM.gov, LA 카운티)
   - AbilityOne·CALPIA 충돌 체크
   - 견적서 자동 작성

   팔 생각 없이, 내 영업 시간을 줄이는 용도입니다.
2. **팔 만한 틈이 보이면 그때 제품으로 만드세요.** 6개월 동안 직접 써 보면서 "다른 사람도 돈을 낼 만한 불편"이 확인될 때입니다.
3. **틈새 후보(가설):** 작은 소비재 제조사가 정부나 PX에 진출할 때 필요한 서류를 정리해 주는 도구입니다. 원산지 증명, TAA·Buy American 확인, 안전보건자료(SDS), 바이오기반·친환경 인증 자료, 공급업체 등록 서류 같은 것들입니다. 당신의 성분·인증 지식과 조달 지식이 겹치는 거의 유일한 지점입니다. 처음에는 컨설팅 서비스로 수요를 확인하세요.
4. **장기 후보:** 첫 보고서의 보훈상담관(VSO)용 AI 청구 도구입니다. 시장 공백은 여전히 가장 크지만, 개인정보 보안과 정확성 책임이 무거워서 기술 파트너가 생긴 뒤에 하는 것이 맞습니다.

---

## 5. 당신 상황에 맞춘 실행안

### 직장에 다니는 동안 (지금~6개월)

- [ ] SAM.gov 등록(일반 소기업으로). 무료입니다.
- [ ] 캘리포니아 SB(마이크로비즈니스) 인증이 가능한지 DGS에 확인
- [ ] DGS 소기업 인증 부서(OSDS)에 "직장을 다니면서도 DVBE 인증이 가능한지" 직접 문의: (916) 375-4940, Certification@dgs.ca.gov
- [ ] 개인관리·위생 품목 2~3개 선정 → AbilityOne·CALPIA 목록과 겹치는지 확인
- [ ] 가까운 VA 병원 매점 책임자(Canteen Chief)에게 연락(PatriotMart), AAFES 공급업체 페이지로 신청
- [ ] 지방정부(카운티, 시, 학군)의 소액 견적 요청에 일반 소기업으로 응찰
- [ ] Claude Code로 내 품목 입찰 모니터 만들기

### 반응이 있으면 (풀타임 전환 시)

- [ ] SBA VetCert(SDVOSB), DGS DVBE, LA 카운티 DVBE 인증 신청
- [ ] VA 세트어사이드, DVBE 하도급, SB/DVBE 옵션, LA 카운티 15% 우대를 본격적으로 활용
- [ ] 회사 형태 다시 검토(DVBE를 받으려면 LLC는 상이군인 100% 소유)

### 반응이 없으면

6개월 뒤 정리합니다. 잃는 것은 시간과 적은 비용입니다.

---

## 6. 주의: 현재 직장과의 관계

- 지금 다니는 회사와 같은 품목, 같은 공급처, 같은 고객을 상대하면 이해충돌이나 영업비밀 문제가 생길 수 있습니다. 캘리포니아에서는 경업금지 약정이 대부분 무효지만, 재직 중 충실 의무와 영업비밀 보호는 그대로 적용됩니다.
- 현재 회사의 제품을 당신 회사가 정부에 공급하는 구조도 가능은 합니다. 미국 소기업 제조사의 제품이라면 비제조업체 규칙을 충족합니다. 다만 두 회사가 '계열관계'나 명의대여로 의심받지 않도록 APEX나 변호사와 함께 구조를 짜야 합니다.
- DVBE는 일상 경영을 맡는 상이군인이 소유자와 달라도 된다고 DGS가 안내합니다. 하지만 그 사람이 실제로 경영해야 하고, 형식만 갖추면 명의대여가 됩니다.

---

## 7. 답해 주시면 더 좁혀 드릴 질문

1. 캘리포니아 어느 지역에 사시나요? LA 카운티라면 15% 우대가 큽니다.
2. VA 장애등급이 10% 이상인가요? TDIU를 받고 계신가요?
3. 지금 직장이 위생·개인관리 제품 회사인가요? 그 회사와 겹치지 않는 품목을 다룰 수 있나요?
4. 직장 근무시간은 어떻게 되나요(풀타임인지)?

---

## 출처

### 직장 병행과 SDVOSB 통제 요건
- [13 CFR 128.203 — Cornell LII](https://www.law.cornell.edu/cfr/text/13/128.203)
- [eCFR 13 CFR Part 128](https://www.ecfr.gov/current/title-13/chapter-I/part-128)
- [SBA: Full-time devotion still matters for SDVOSBs — SmallGovCon](https://smallgovcon.com/service-disabled-veteran-owned-small-businesses/sba-full-time-devotion-still-matters-for-sdvosbs/)
- [Fully devoted or just rebuttable? — SmallGovCon](https://smallgovcon.com/service-disabled-veteran-owned-small-businesses/fully-devoted-or-just-rebuttable-qualifying-veterans-overcome-full-time-devotion-rule/)

### 비제조업체 규칙
- [Back to Basics: The Nonmanufacturer Rule — SmallGovCon](https://smallgovcon.com/sba-size-protests/back-to-basics-the-nonmanufacturer-rule/)
- [Even rules have their limits (OHA) — SmallGovCon](https://smallgovcon.com/sbaohadecisions/even-rules-have-their-limits-says-sba-oha-about-the-nonmanufacturer-rule/)

### 캘리포니아 DVBE, SB, SB/DVBE 옵션
- [DGS SB/DVBE 인증 신청](https://www.dgs.ca.gov/pd-certification)
- [DVBE Certification Requirements — DGS](https://www.dgs.ca.gov/-/media/Divisions/PD/OSDS/Certification/DVBE/DVBE_CertificationRequirements.pdf)
- [SB/DVBE Eligibility — Caltrans](https://www.dot.ca.gov/programs/civil-rights/dvbe/sb-dvbe-eligibility)
- [SB and SB-PW Certification Eligibility Requirements — DGS](https://www.dgs.ca.gov/-/media/Divisions/PD/OSDS/Certification/SB/SB-and-SB-PW-Certification-Eligibility-Requirements.pdf)
- [SB/DVBE Option 안내 — DGS](https://www.dgs.ca.gov/-/media/Divisions/PD/OSDS/Outreach/Resources/SB_DVBE_Option.pdf)
- [State Administrative Manual 1233 — DGS](https://www.dgs.ca.gov/Resources/SAM/TOC/1200/1233)
- [Government Code 14838.5](https://california.public.law/codes/ca_gov't_code_section_14838.5)
- [DVBE Program Requirements — DGS](https://www.dgs.ca.gov/-/media/Divisions/PD/Acquisitions/Solicitation-Document-Attachments/DVBE-Program-Requirements.pdf)
- [DVBE Incentive — DGS](https://www.dgs.ca.gov/-/media/Divisions/PD/Acquisitions/Solicitation-Document-Attachments/DVBE-Incentive.pdf)
- [CDE 입찰 첨부(부적격 처리 조항)](https://www.cde.ca.gov/fg/fo/r8/documents/aepd20rfpatt3.pdf)
- [CUF Departmental Policy — DGS](https://www.dgs.ca.gov:443/-/media/Divisions/PD/OSDS/Certification/CUF/CUF-Departmental-Policy.pdf)
- [SDSU DVBE Participation Requirement (CSU 적용 예)](https://bfa.sdsu.edu/financial/procurement/docs/DVBEPartReq.pdf)
- [DVBE Rules and Procedures — San Luis Obispo Superior Court (법원 적용 예)](https://www.slo.courts.ca.gov/system/files/general/dvbe-rules-and-procedures.pdf)

### LA 카운티
- [LA County Business Preference Programs — LA County Public Works](https://dpw.lacounty.gov/core-service-areas/municipal-services/contracts/la-county-business-preference-programs/)
- [DVBE — LA County Public Works](https://dpw.lacounty.gov/Contracts/DVBE)
- [Preference Programs Implementation Guidelines — LA County](https://isdtest.lacounty.gov/wp-content/uploads/2024/06/Preference_Programs_Implementation_Guidelines-1-3.pdf)
- [LA County Certification Portal](https://certify.lacounty.gov)

### 의무 구매처 (AbilityOne, CALPIA)와 환경 요건
- [SKILCRAFT products — AbilityOne](https://www.abilityone.gov/procurement_list/product_skilcraft.html)
- [SKILCRAFT — NIB](https://nib.org/skilcraft)
- [State Contracting Manual 5-2 (CALPIA/CALCTRA) — DGS](https://www.dgs.ca.gov/PD/Resources/SCM/TOC/5/5-2)
- [CALPIA Exemption Instructions](https://www.calpia.ca.gov/wp-content/uploads/calpia/about/doing_business/exemptions/CALPIA%20Exemption%20Instructions-PIA.pdf)
- [VA RFO Deviation Part 23 — acquisition.gov](https://login.acquisition.gov/sites/default/files/page_file_uploads/VA_RFO_Deviation_Part-23.pdf)
- [2024 FAR sustainable procurement revisions — EPA](https://epa.gov/greenerproducts/2024-revisions-federal-acquisition-regulation-strengthening-federal-sustainable)

### 군 PX와 VA 매점
- [The U.S. Air Force Exchange — ND APEX Accelerator (2026.5)](https://blogs.und.edu/ndapex/2026/05/the-u-s-air-force-exchange/)
- [AAFES seeks businesses — DVIDS](https://www.dvidshub.net/news/printable/453992)
- [AAFES stores add veteran-owned Caisson Shaving](https://business.dptribune.com/dptribune/news/read/42527313/aafes_stores_add_veteran_owned_caisson_shaving_company)
- [Doing Business with VCS — VA OSDBU](https://www.va.gov/osdbu/library/dbwvaVcs.asp)
- [PatriotMart Concessionaire — VA Canteen Service](https://vacanteen.va.gov/VACANTEEN/PatriotMart_Concessionaire.asp)

### 소프트웨어 경쟁 현황
- [GovDash secures $30M — SiliconANGLE (2026.1)](https://siliconangle.com/2026/01/15/govdash-secures-30m-expand-ai-driven-government-contracting-software/)
- [Procurement Sciences funding — ExecutiveBiz](https://www.executivebiz.com/articles/procurement-sciences-funding-govcon-ai-platform)
- [NationGraph raises $18M — GrantedAI](https://grantedai.com/news/nationgraph-18m-ai-government-contracting)
- [Cleatus — Technical.ly](https://technical.ly/entrepreneurship/cleatus-aims-to-be-the-ai-operating-system-for-govcon.md)
- [pWin.ai secures $10M — Techleap](https://finder.techleap.nl/news/feed/pwin-ai-secures-10m-for-gov-contracts)
- [GovSignals seed — Pulse 2.0](https://pulse2.com/govsignals-5-5-million-seed-raised-to-automate-process-of-finding-government-funding)
- [GovEagle $2.5M](https://www.goveagle.com/blog/goveagle-secures-2-5m-in-funding-from-strategic-cyber-ventures-y-combinator-to-enhance-its-ai-powered-proposal-platform)
- [How to find subcontractors (유료 매칭 서비스 예) — ComplianceNews](https://compliancenews.com/blog-details/10)

### 기타
- [2025 셧다운 43일 종료 — Fortune](https://fortune.com/2025/11/13/trump-government-shutdown-over-43-days-historic-length/)
- [셧다운 이후 계약업체 영향 — Washington Examiner](https://www.washingtonexaminer.com/?p=3893790)
- [VA $1.17B 노숙 재향군인 보조금 — Newsweek](https://newsweek.com/map-shows-va-sending-1-17-billion-fight-veteran-homelessness-12443433)
- [SSVF NOFO — Federal Register (2026.1.6)](https://www.govinfo.gov/content/pkg/FR-2026-01-06/html/2026-00009.htm)
