# 업로드 보고서 검토와 ManuKind 맞춤 정리

> **기준일:** 2026년 11월 8일
> **검토 대상:** `SDVOSB_opportunity_map_2026-11-08.md`(사업 기회 지도 v2), `SDVOSB_shortlist_2026-11-08.md`(입찰 후보 상세), `SDVOSB_bids_2026-11-08.csv`(전국 SDVOSB 전용 공고 419건)
> **방법:** SAM.gov 공고 원문(API), USAspending, 공식 사이트, 언론 보도와 대조했습니다.
> **주의:** 법률·세무 자문이 아닙니다.
>
> 관련 문서: [첫 보고서](README.md) · [팩트체크 후속](followup-factcheck-and-new-directions.md) · [기회 탐색](opportunity-scan-2026-11.md)

---

## 0. 결론

1. **두 보고서의 사실관계는 대부분 맞습니다.** 다만 "지금 바로 SDVOSB 전용 입찰"을 중심으로 짜여 있습니다. 인증, 현장 인력, 지역을 따져 보면 **이번 달에 실제로 낼 수 있는 건은 거의 없습니다.**
2. **가장 중요한 발견:** ManuKind는 **뉴질랜드에서 만드는** 마누카꿀·산양유 아기 물티슈와 기저귀 브랜드입니다.
   - PX 입점과는 잘 맞습니다.
   - 하지만 정부 '입찰'(FEMA 키트 등)은 소기업 전용 물품 입찰이 요구하는 '미국 소기업이 미국에서 만든 제품' 요건(비제조업체 규칙, NMR) 때문에 거의 막힙니다.
3. **판을 바꿀 질문:** ManuKind나 Formuland를 운영하는 법인을 사장님이 51% 이상 소유하고 풀타임으로 경영하나요?
   - 그렇다면 **새 회사를 만들 필요 없이 기존 회사로 SDVOSB·DVBE 인증**을 받을 수 있습니다.
   - 앞서 제가 "직장을 다니면서는 인증이 어렵다"고 한 판단도 달라집니다. 그 '직장'이 바로 이 회사라면 풀타임 요건을 충족하기 때문입니다.

---

## 1. ManuKind 제품으로 갈 수 있는 경로

ManuKind 공식 사이트 기준으로 물티슈는 뉴질랜드산, 물 99.4%, 대나무 원단 100%, 독일 Dermatest 피부 테스트를 받았습니다. 마누카꿀 기저귀도 있고, Amazon, Formuland, Granola Baby에서도 판매합니다.

| 경로 | 뉴질랜드산 제품 가능? | 이유 |
|---|---|---|
| **군 PX**(AAFES·NEXCOM·MCX·CGX) 매장과 온라인 | ◎ 가능 | 재판매하는 소매라서 소기업 제조·원산지 입찰 규칙이 적용되지 않습니다 |
| **VA 병원 매점**(PatriotMart) | ○ | 지역 매점 책임자가 승인합니다. 재향군인 소유 기업을 우대하고, 미국산을 일부 섞도록 권장합니다 |
| 커미서리(DeCA) | △ | 일반 소매 실적이 필요합니다. 안 팔리면 회수하는 판매 보증 조건이 있고, 유통업체가 필요합니다 |
| FEMA 영유아 키트 (소기업 전용) | ✕에 가까움 | 소기업 전용 물품 입찰은 미국 소기업이 미국에서 만든 제품이어야 합니다. 뉴질랜드산은 그 비율에 들어가지 않습니다 |
| GSA 스케줄 | ✕ | 개인위생용품 품목(SIN 335210)이 2025년 4월 폐지됐고, 개인위생품은 범위 밖입니다 |
| 군 어린이집 | ✕ | 기저귀와 물티슈는 부모가 가져옵니다 |
| 캘리포니아 신생아 무료 기저귀(Golden State Start) | △(가능성 낮음) | 'Baby2Baby 기저귀'를 무료로 대량 배포하는 사업이라 저단가 중심입니다 |

---

## 2. 업로드 보고서 팩트체크

### 사업 기회 지도 v2

| 주장 | 판정 | 확인 결과 |
|---|---|---|
| 캘리포니아 DVBE: 장애 10% 이상, 캘리포니아 거주, 51% 이상 소유·경영 | ⚠️ 빠진 것 있음 | **LLC는 상이군인이 100% 소유해야 합니다**(DGS 안내). 51%만 소유하는 구조라면 주식회사여야 합니다 |
| SB/DVBE 옵션 $5,000~$249,999.99 (2026년 3월 18일 시행 기준) | ⚠️ 날짜 혼동 | 금액은 맞습니다. 3월 18일 조정은 **공공공사 기준액**(2026년 4월 기준 $484,000) 얘기입니다. 물품·서비스 한도를 $35만으로 올리는 법안(SB 70, SB 1044)은 아직 통과가 확인되지 않았습니다 |
| Golden State Start: 2026년 5월 시작, 15개 병원에서 203개로 확대, 계약 $620만 | ⚠️ 보도와 다름 | 2026년 5월 8일 발표, **첫해 65~75개 병원**, 예산은 전년도 $740만에 $1,250만 추가 제안, 신생아 1명당 'Baby2Baby 기저귀' 400개(언론·Baby2Baby 발표 기준) |
| FEMA 영유아 키트: 중소기업 전용 장기계약 | ✅ 맞음 | 소기업 전용(업종 624230)입니다. 최근 주문은 RCG of North Carolina 약 $133만(3건), Asset Group 약 $40만(3건) |
| → ManuKind를 현 계약업체에 공급 제안 | ⚠️ 가능성 낮음 | 위 1장 이유(미국 소기업 제조 요건) 때문입니다 |
| Santa Maria 클리닉 개원 지원(IOTA) 11/30 | ✅ 공고는 맞음 | SDVOSB 전용 사전공고입니다(업종 541614). 하지만 **의료장비 기획, 이전·개원 계획 전문 인력이 필요한 전문 서비스**입니다. 과거 수주사도 전문 업체(Concourse Federal 등)라 첫 사업으로는 비현실적입니다 |
| GSA 스케줄 개인위생용품 2025년 폐지 | ✅ 맞음 | 2025년 4월 개정(Refresh 26)에서 SIN 335210(Personal Care Items)이 폐지됐습니다 |
| 육군 DEVCOM 병사센터 연구공모(BAA) 2030년 2월까지 상시 | ✅ 맞음 | 전투식량, 병사 보호장비 등 연구개발 분야입니다. 소비재 판매 채널은 아닙니다 |
| SDVOSB 수의계약 상한 $500만(제조 $850만) | ✅ 맞음 | 의회조사국(CRS, 2026년 9월) 기준과 같습니다 |
| 침대패드 같은 소모품 직접 입찰은 하지 말 것 | ✅ 맞음 | 제 조사와도 같습니다(VA는 Medline 일괄공급 체계) |
| 유아용품은 CPSIA 시험성적과 CPC 필요 | ⚠️ 품목별로 다름 | **물티슈는 FDA 화장품**입니다(MoCRA 등록·리스팅, 성분 표시). **기저귀는 소비자제품**이라 어린이용 제품 규정(CPSC) 적용 범위를 시험소에 확인하세요 |
| 의료장비 유통, 가구, IT 큰 시장 | ✅ 숫자는 타당 | 다만 셋 다 **SDVOSB 인증 + 제조사 대리점 계약 + 해당 업계 전문성**이 필요한 별개의 사업입니다 |

### 입찰 후보 상세(shortlist)

| 주장 | 판정 | 확인 결과 |
|---|---|---|
| 모든 건이 SAM 등록과 SBA 인증을 요구 | ✅ 맞음 | 제출 시점에 인증이 있어야 합니다 |
| 빵 납품 4건(텍사스): SDVOSB 전용이라 대기업 기존 업체 제외, 비제조업체 규칙 면제 없음 | ✅ 맞음 | 원문 질의응답에 "No waiver applies"가 확인됩니다. **하지만 텍사스 4곳 배송과 이번 주 인증은 캘리포니아 사장님에게 비현실적입니다.** 그래도 "대기업이 빠지는 SDVOSB 전용 건에 소기업 제조사 제품을 대는 유통 모델"이라는 통찰은 맞습니다 |
| 트랙 B: 지금 인증 신청하고 창문 청소(11/19, 11/27) 노리기 | ❌ 비현실적 | 새 업체는 SAM 등록에 수 주가 걸리고, 인증 심사도 기다려야 하며, 제출 시점에 인증이 있어야 합니다. 게다가 매사추세츠·인디애나 현장 서비스는 **인건비 50% 이상을 자기 직원이 수행**해야 합니다 |
| 북가주 멸균물품 택배: 준비 시간이 가장 넉넉 | ⚠️ 부분적 | 착수는 2027년 6월이지만 **제출 마감 12/2에 인증이 있어야 합니다.** 차량과 기사를 고용하는 새 사업입니다 |
| 전국 SDVOSB 공고 419건(CSV) | 참고용 | 국방물류청 부품 179건, 건설 113건(면허·보증 필요)이 대부분이라 ManuKind와 맞는 건은 거의 없습니다 |

---

## 3. 통합 우선순위 (ManuKind 반영)

| 순위 | 할 일 | 이유 |
|---|---|---|
| 1 | **ManuKind PX 입점** (지금 시작) | 제품이 이미 있고 원산지 규칙에 걸리지 않습니다. 재향군인 소유라는 이야기가 군 가족 시장에 맞습니다 |
| 2 | **기존 법인으로 인증이 가능한지 확인** → 가능하면 SAM.gov, SBA VetCert, 캘리포니아 DVBE(LLC면 100% 소유), LA 카운티 인증 | 인증이 있으면 이후 모든 정부 사업의 입장권이 됩니다 |
| 3 | 인증 후 **SDVOSB 유통 사업** 추가 | 대기업이 빠지는 SDVOSB 전용 물품 건에 미국 소기업 제조사 제품을 공급합니다(예: 빵, 담요 보온기). 캘리포니아 안에서 물류를 감당할 수 있는 품목부터 시작하고, 국방물류청 DIBBS 소액 견적(침대 방수 패드 등)으로 연습하세요 |
| 4 | (선택) 한국어 통번역 | 자본이 거의 들지 않는 부업형입니다([기회 탐색 보고서](opportunity-scan-2026-11.md) 7장) |
| 5 | 장기 | VA 가구·의료장비 유통, 개원 지원(IOTA) 팀 참여, LA 의류 공장 연결 |

---

## 4. ManuKind PX 입점 체크리스트

**신청 경로**
1. **VA 병원 매점에서 시범 판매:** 웨스트 LA나 롱비치 VA 매점 책임자, 또는 VCSRSS@med.va.gov에 제품 목록, 가격, 사진을 보냅니다.
2. **RangeMe 무료 프로필:** AAFES, NEXCOM, MCX, CGX에 동시에 노출됩니다. AAFES는 30일 안에 회신합니다.
3. **AAFES 온라인 마켓플레이스:** 초기비 $1,500에 월 사용료·거래 수수료·판매 수수료가 붙고, 군인에게만 주는 혜택가가 필요합니다. 문의: MarketplaceTeam@aafes.com
4. 매장 입점, 그다음 커미서리(DeCA).

**준비물**
- GS1 바코드
- EDI(전자 거래 문서, MCX는 SPS Commerce 사용)
- 대금 조건 Net 30
- 보험: 업로드 보고서 기준 MCX는 일반배상 $1M/$2M과 제조물책임 $2M(원문 재확인 필요)
- 강제노동 금지 정책 서약
- 원산지 표시 "Made in New Zealand"

**규제**
- **물티슈(FDA 화장품):**
  - 뉴질랜드 제조시설을 FDA에 등록하고, 미국 브랜드 법인이 제품을 리스팅해야 합니다(MoCRA). 연매출 100만 달러 미만 소기업은 등록·리스팅 면제가 될 수 있으니 확인하세요.
  - 이상사례 보고, 영어 성분 표시(INCI), 미국 연락처 표시가 필요합니다.
  - **'항균', '발진 치료' 같은 효능 문구를 쓰면 의약품이 됩니다.** 마누카꿀 제품에서 특히 주의하세요. 공식 사이트 첫 화면, 소개, FAQ에서는 이런 문구가 보이지 않았습니다.
- **기저귀(소비자제품):** 어린이용 제품 규정(CPSC)이 적용되는지 시험소에 확인하세요.
- **캘리포니아:** Prop 65 경고문 규정이 적용됩니다.

**가격 위치 (업로드 보고서의 PX 온라인 조사)**

| 브랜드 | 1매당 |
|---|---|
| Exchange Select (PX 자체 브랜드) | 약 $0.03 |
| Pampers | 약 $0.05 |
| Honest | 약 $0.08 |
| **ManuKind** | **약 $0.11~0.13** |
| WaterWipes | 약 $0.18 |

Honest와 WaterWipes 사이의 프리미엄 위치입니다. PX 도매가는 소비자가의 절반 이하가 일반적이니 마진을 먼저 계산하세요.

**거래 금지 확인:** PX는 현역 군인, 연방공무원(매점 직원 포함), 그 가족과는 거래하지 않습니다(AR 215-8).

---

## 5. 답해 주시면 바로 다음 단계로 갈 질문

1. ManuKind나 Formuland를 운영하는 법인은 LLC인가요, 주식회사인가요? 사장님 지분은 몇 %이고, 다른 주주나 투자자가 있나요?
2. 사장님이 그 회사에서 풀타임으로 일하나요(다른 직장 없음)?
3. VA 장애등급이 10% 이상인가요? TDIU를 받고 있나요?
4. 어느 카운티에 사시나요?

---

## 출처

- [ManuKind 공식 사이트](https://manukind.com)
- [Santa Maria IOTA 사전공고](https://sam.gov/opp/4a8041a3553c4a4c8509984ee373c306/view)
- [빵 납품 공고(질의응답 포함)](https://sam.gov/opp/59fe94dec8b945fcb4a2c1ce3bd5e4ac/view)
- [창문 청소 공고(Bedford)](https://sam.gov/opp/e0ea83b588f74b11a92463e49de4c21c/view)
- [북가주 멸균물품 택배 사전공고](https://sam.gov/opp/25004d459fbb499f95fb40672879ee2c/view)
- [FEMA 영유아 키트 입찰 70FB7023R00000006](https://sam.gov/opp/af9dd4ddfd324993bf388a7618ca3d8d/view) · [HigherGov 요약](https://www.highergov.com/contract-opportunity/70fb7023r00000006-infant-toddler-kits-70fb7023r00000006-o-5d565)
- [DEVCOM 병사센터 BAA](https://sam.gov/opp/e8c7609f0f154df4afda846595bca888/view)
- [Golden State Start — Baby2Baby](https://baby2baby.org/goldenstatestart/) · [AP 보도(WTOP 게재)](https://wtop.com/lifestyle/2026/05/california-will-lead-the-nation-in-providing-free-diapers-to-newborns-at-dozens-of-hospitals/)
- [SB/DVBE 옵션 안내(DGS, 2026)](https://www.dgs.ca.gov/-/media/Divisions/PD/OSDS/Outreach/Resources/2026/SB_DVBE_Option.pdf) · [DGS 공공공사 기준액 조정 공지 P0126](https://www.dgs.ca.gov/-/media/Divisions/PD/PTCS/Broadcast-Bulletins/2026/P-Bulletins/BB-P0126-SB-and-DVBE-Option-Acquisition-Methods--Dollar-Threshold-Increase-for-Public-Works-Contract.pdf)
- [GSA MAS Refresh 26 폐지 SIN 해설 — Winvale](https://info.winvale.com/blog/gsa-retire-32-sins-multiple-award-schedule-solicitation-refresh-26) · [GSA Refresh 27 주요 변경(공식)](https://vsc.gsa.gov/drupal/sites/default/files/internal-files/UPDATED%206.18%20_%20Significant%20Changes%20Attachment%20for%20MAS%20Refresh%20%23%2027%20-%20June%202025.pdf)
- [DVBE 인증 요건(DGS)](https://www.dgs.ca.gov/-/media/Divisions/PD/OSDS/Certification/DVBE/DVBE_CertificationRequirements.pdf)
- [13 CFR 128.203 (SDVOSB 통제·풀타임 요건)](https://www.law.cornell.edu/cfr/text/13/128.203)
- [AAFES 신규 공급업체 안내](https://www.aafes.com/about-exchange/doing-business/prospective.htm) · [AAFES 마켓플레이스 안내(PDF)](https://www.shopmyexchange.com/assets/pdfs/marketplacebrochure2024.pdf)
- [VA 매점 PatriotMart](https://vacanteen.va.gov/VACANTEEN/PatriotMart_Concessionaire.asp)
- [USAspending](https://www.usaspending.gov) (FEMA 영유아 키트 주문 조회)
