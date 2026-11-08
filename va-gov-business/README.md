# VA·연방정부 대상 사업 기회 조사 보고서

> **기준일:** 2026년 11월 8일
> **대상:** 상이군인(Service-Disabled Veteran) 창업 예정자
> **조사 방법:** 공개 자료(VA·국방부·의회조사국(CRS)·VA 감사관실(OIG) 발표, 언론 보도, 계약 공고 집계 사이트) 웹 조사
> **주의:** 일부 수치는 2차 출처(언론·블로그·집계 사이트)입니다. 계약·입찰 결정 전에는 SAM.gov, USAspending.gov, 원문 공고로 반드시 다시 확인하세요. 이 문서는 법률·세무 자문이 아닙니다.
> **후속 문서:** 다른 대화 내용 팩트체크, 캘리포니아 기회, 정정 사항은 [followup-factcheck-and-new-directions.md](followup-factcheck-and-new-directions.md)에 있습니다.

---

## 0. 결론 먼저 (1분 요약)

### 질문 1. "진료 녹음 → AI 정리 → 필요한 곳 자동 입력 → 약 얘기하면 오더 자동 작성 → 의사는 최종 확인만 + 옛 CPRS 같은 UI + 모든 VA 시스템 연동" 어때?

문제를 보는 눈은 정확합니다. 하지만 **그 제품은 이미 VA가 전국에 깔고 있습니다.** 정면승부는 추천하지 않습니다.

- **VA Ambient Scribe 프로그램:** 2025년 10월 10개 병원에서 시범 운영 → 2026년 6월 약 4,000명 임상의 사용 → 2026년 9월 전국 1차진료·정신건강·재활·내외과 전문과로 확대 발표.
- **2026년 9월, 5년간 최대 $7억 7,572만짜리 다중 낙찰 계약(IDIQ) 체결:** Knowtex, Carahsoft(Abridge 유통사).
- **Abridge는 기존 CPRS(VistA)와 새 Oracle EHR 양쪽에 모두 연동**되어 있고, 75개 이상 VA 병원에서 쓰이고 있습니다(회사 발표).
- **"약 얘기하면 오더 초안 작성 → 의사 승인" 기능은 Oracle이 2026년 2월에 이미 출시했습니다.** Oracle은 VA가 옮겨가고 있는 EHR 회사입니다. 국방부(DHA)는 2026년 4월부터 Oracle의 AI 에이전트를 군 의료 전체로 확산하고 있습니다.
- **CPRS는 2026~2031년 사이에 Oracle로 교체됩니다.** CPRS를 닮은 UI를 만드는 것은 수명이 줄어드는 시스템 위에 집을 짓는 일입니다.

### 질문 2. 그럼 다른 건 없나?

있습니다. 아이디어의 핵심 패턴인 **"녹음/문서 → 구조화 → 양식 자동 작성 → 사람이 최종 확인"** 을 **대기업이 관심 두지 않는 좁은 시장**에 적용하세요. 그리고 **SDVOSB(상이군인 소유 소기업) 지위**를 무기로 쓰는 전략이 현실적입니다.

| 순위 | 기회 | 한 줄 설명 |
|---|---|---|
| 기반 | SDVOSB 인증 + 조달 기반 구축 | 무료이고 지금 바로 시작할 수 있습니다. **2026년 12월 8~9일 VA 최대 재향군인 기업 행사(NVSBE, 클리블랜드)** 가 있습니다 |
| 1 | 보훈상담관(VSO)용 AI 청구 준비 도구 | 고객은 카운티·주정부입니다. 기존 업체(Tyler VetraSpec 등)에서 생성형 AI 기능이 확인되지 않아 빈자리가 있습니다 |
| 2 | C&P 검사(DBQ) 품질검사·작성 보조 AI | 검사의 85% 이상을 4개 업체가 위탁 수행합니다. 2026~27년 재입찰이 있고, 의회가 DBQ 품질을 감독하고 있습니다 |
| 3 | 커뮤니티 케어(민간 병원)용 VA 서류·기록 반환 자동화 | 고객이 민간이라 VA 운영승인(ATO)이 필요 없습니다. 기록 반환 누락 문제는 감사에서 공식 지적됐습니다 |
| 4 | VA AI 품질검증·도입지원 서비스(하도급 포함) | VA의 AI 사용 사례 367건 중 215건이 '고위험'입니다. 감사관실이 안전관리 미흡을 지적했습니다 |
| 5 | 물품 납품 | 가능은 하지만 마진이 얇고 규정 리스크가 큽니다. 직접 제조하거나 독점 유통권이 있을 때만 권합니다 |

**하지 말 것:** CPRS 복제품으로 VistA에 직접 오더 쓰기 / 명의대여(rent-a-vet) / 재향군인에게 비인가 청구 대행료 받기 / TDIU(개인실업수당) 수령 중 소득 문제 무시하기.

---

## 1. 원래 아이디어 정밀 평가

### 1.1 요소별로 이미 누가 하고 있나

| 아이디어 요소 | 이미 하고 있는 곳 | 현재 상태 | 끼어들 여지 |
|---|---|---|---|
| 진료 대화 녹음 → 노트 자동 작성 | VA Ambient Scribe(Abridge, Knowtex), Microsoft Dragon Copilot, Oracle Clinical AI Agent | VA 전국 확산 중 | 거의 없음 |
| 기록을 필요한 곳에 입력(EHR 연동) | Abridge: VistA/CPRS와 Oracle Federal EHR 양쪽 | 운영 중 | 거의 없음 |
| 약·검사 얘기하면 오더 초안 → 의사 승인 | Oracle Clinical AI Agent 오더 생성 기능(2026년 2월 출시) | 상용화됨 | 거의 없음. VA 차세대 EHR의 기본 기능이 될 가능성이 큼 |
| 옛 CPRS 같은 UI 제공 | 없음. CPRS 자체가 퇴역 중 | Oracle 전환: 2026년 13곳, FY2027 26곳, 2031년 완료 목표 | 수명이 짧음 |
| 모든 VA 시스템 연동 | VA가 통제(병원별 VistA, Oracle EHR, VBMS, HSRM 등) | 외부 소기업 혼자서는 불가능 | VA의 후원이 필요 |

참고로 VA도 과거에 CPRS를 현대화하는 자체 프로젝트(eHMP 등)를 진행했습니다. 그러다 2017년 Cerner(현 Oracle Health) 도입으로 방향을 바꿨습니다.

### 1.2 VA AI 스크라이브 진행 경과

| 시점 | 내용 |
|---|---|
| 2024년 | VA "AI Tech Sprint"(상금 대회) 개최. 150개 이상 팀 참가, 1등 상금 $30만. 진료 기록(앰비언트 받아쓰기) 부문 1위는 Abridge와 Nuance(Microsoft)였고, Knowtex·DeepScribe·Commure 등도 수상 |
| 2025년 | 수상자에게 경쟁 없는 시범 계약 체결. 공고에는 Abridge와 Carahsoft(유통사)가 계약자로 나옴. Knowtex는 약 $1,500만 계약(2025년 7월 21일 시작, 2차 출처) |
| 2025년 10월 | 10개 VA 병원에서 시범 운영 시작 |
| 2026년 1월 | VA 감사관실(OIG) 예비 경고: VHA에는 생성형 AI 도구(VA GPT, M365 Copilot Chat)의 환자안전 위험을 관리하는 공식 체계가 없다 |
| 2026년 1월 20일 | "Ambient Scribe Post-MVP Development Support – SBIR" 사전 공고 |
| 2026년 6월 | 약 4,000명 임상의 사용. OIG 최종보고서가 고위험 분류와 거버넌스 강화를 권고 |
| 2026년 9월 | 전국 1차진료·정신건강·재활·내외과 전문과로 확대 발표. 5년 IDIQ(상한 $7억 7,572만) 체결: Knowtex, Carahsoft. Abridge가 선정 사실 발표 |
| 같은 시기 | Rise8(SDVOSB)과 Thoughtworks가 시범 10곳을 130곳 이상으로 넓히는 플랫폼 작업 수주(약 $490만, 2차 출처) |

**시사점 두 가지**

1. **문은 있었지만 이번 기회는 닫혔습니다.** 2022년에 창업한 스타트업(Knowtex)도 상금 대회를 통해 VA에 들어갔습니다. 하지만 5년 계약이 이미 체결됐으니 다음 큰 기회는 몇 년 뒤입니다. 기존 계약에 업체가 추가될 가능성은 불확실합니다.
2. **상이군인 기업이 'AI 회사'가 아니라 '통합·플랫폼' 역할로 들어갔습니다(Rise8).** 당신 같은 상이군인 기업이 노려볼 만한 자리가 바로 이런 역할입니다.

### 1.3 정면승부가 어려운 이유

1. **경쟁사의 덩치가 큽니다.** Abridge의 기업가치는 2025년 중반 약 $53억이었습니다. Menlo Ventures 추정(2025년 10월, 2차 인용)에 따르면 약 $6억 규모 시장에서 Microsoft/Nuance가 33%, Abridge가 30%를 차지합니다. 민간 최대 EHR 회사인 Epic도 자체 AI 스크라이브를 발표했습니다. Commure가 Augmedix를 $1.39억에 인수하는 등 시장은 이미 **인수합병으로 정리되는 단계**입니다.
2. **보안·인증 부담이 큽니다.** VA에 소프트웨어를 팔려면 여러 관문을 넘어야 합니다:
   - VA 운영승인(ATO, VA Handbook 6500 위험관리 체계)
   - 클라우드 제품이면 보통 FedRAMP Moderate 이상
   - Section 508 접근성
   - VA 기술참조모델(TRM) 등재
   - PIV 카드 로그인
   - 개인정보영향평가(PIA)

   흔히 수십만~수백만 달러와 1년 이상이 든다고 말합니다(이번 조사에서 공식 비용 수치는 확인하지 못했습니다). 2026년 개편된 FedRAMP 20x로 절차는 빨라지고 있지만 여전히 큰일입니다.
3. **환자안전 문제가 있습니다.** OIG는 VA의 생성형 AI 사용에 환자안전 관리 체계가 없다고 공식 지적했습니다(2026년 1월, 6월). **약 오더 자동화는 그중에서도 위험도가 가장 높은 영역입니다.** 마약류 전자처방(EPCS)은 처방자가 2단계 인증으로 직접 서명해야 합니다. 따라서 AI는 '초안'까지만 만들 수 있습니다.
4. **시스템 접근이 막혀 있습니다.** VistA 쓰기 권한과 Oracle EHR 연동은 VA와 Oracle이 통제합니다. 외부 소기업이 "모든 VA 시스템 연동"을 혼자 해낼 방법은 없습니다.
5. **지금은 품질 논란 국면입니다.** VHA가 지원한 연구에서 AI 노트의 품질 점수가 사람이 쓴 노트보다 낮게 나왔습니다. VA는 업체를 늘리기보다 **검증을 강화하는 단계**에 있습니다.

### 1.4 그래도 살릴 수 있는 것

**녹음/문서 → 구조화 → 양식 자동 작성 → 사람이 최종 확인**이라는 흐름 자체는 가치가 있습니다. 다만 다음 세 가지 조건을 만족하는 곳으로 옮겨야 합니다.

- (a) VA 진료 노트가 아닌 **다른 '양식' 영역:** DBQ, 보훈 청구 서류, 커뮤니티 케어 서류
- (b) 고객이 VA 본부가 아닌 곳: **검사 위탁업체, 카운티·주정부, 민간 병원.** 인증 장벽이 훨씬 낮습니다
- (c) **당신이 환자이자 청구인으로서 겪은 경험이 차별점이 되는 곳**

---

## 2. 당신의 무기: SDVOSB(상이군인 소유 소기업)

### 2.1 숫자로 보는 시장

- **VA는 2025 회계연도(FY2025)에 SDVOSB와 $101억어치를 계약했습니다.** VA 원청 계약의 약 21.6%이고, VA 내부 목표는 15% 수준입니다.
- 연방정부 전체로는 FY2025에 SDVOSB와 $325억을 계약했습니다(적격 원청 계약의 5.01%). 법정 목표는 FY2024 국방수권법(NDAA)으로 3%에서 5%로 올랐습니다.
- **자기인증은 더 이상 실적으로 인정되지 않습니다.** 반드시 SBA VetCert 인증을 받아야 합니다.

### 2.2 VA의 "Vets First" 우선권

| 제도 | 내용 |
|---|---|
| 2개 업체 원칙(Rule of Two) | 자격 있는 SDVOSB가 2곳 이상이고 합리적인 가격이 예상되면, VA는 SDVOSB끼리만 경쟁시켜야 합니다. 2016년 대법원 Kingdomware 판결로 확정됐습니다 |
| 우선순위 | SDVOSB → VOSB(재향군인 소유 기업) → 기타 소기업 순서 |
| 수의계약 | 간이구매 기준액(SAT, 2025년 10월 1일부터 $35만) 이하는 경쟁 없이 계약할 수 있습니다. SAT 초과 ~ $500만까지는 SDVOSB와 수의계약이 가능합니다(VAAR 806.302-570) |
| 소액구매 | $1만 5,000 이하는 구매카드로 바로 살 수 있습니다(2025년 10월 1일부터) |

참고: 2025년 행정명령 14240으로 공통 물품과 IT 구매가 GSA(연방조달청)로 모이고 있습니다. VA의 AI·IT 구매도 GSA 스케줄이나 GWAC(정부 공용 IT 계약) 같은 GSA 계약 수단을 더 많이 쓰게 됩니다. 그래서 장기적으로는 GSA 스케줄을 확보하거나, SDVOSB 전용 GWAC(Polaris SDVOSB, VETS 2) 보유 업체와 팀을 짜는 것이 중요해집니다. 2025년부터 연방조달규정(FAR) 전면 개정(RFO)도 진행 중이라 세부 규정이 바뀔 수 있습니다. APEX Accelerator에서 최신 내용을 확인하세요.

### 2.3 SBIR Phase III: 잘 알려지지 않은 지름길

- SBIR/STTR(중소기업 기술혁신 연구개발 지원 제도)은 2025년 9월 30일에 만료됐다가, 약 6개월 공백 뒤 **2026년 4월 13일 재승인됐습니다(2031년 9월 30일까지).**
- VA는 SBIR 참여 기관이 아닙니다. 하지만 **다른 기관(국방부, NIH 등)에서 SBIR Phase I/II를 받은 기술은 어느 기관이든 Phase III로 경쟁 없이 계약할 수 있습니다.** 금액 상한이 없고, GAO(회계감사원)도 기관의 재량을 폭넓게 인정합니다.
- 실제 사례: Rise8은 SBIR Phase III로 VA.gov 관련 약 $1,000만 계약을 받았습니다.
- 경로: Phase I(보통 수십만 달러, 타당성 검증) → Phase II(보통 100만 달러대, 개발) → Phase III(수의계약).

### 2.4 개인적으로 받을 수 있는 지원

| 지원 | 내용 |
|---|---|
| VR&E 자영업 트랙(Chapter 31) | 사업계획 수립, 소기업 경영 교육, 마케팅·재무 지원, 교육·면허 비용, 일부 창업비용(타당한 경우)을 지원합니다. 총비용 $2만 5,000 이상인 계획은 VR&E 본부 책임자(Executive Director)의 승인이 필요합니다(38 CFR 21.258). 지원 범위는 장애 정도 등에 따라 다르니 상담관과 확인하세요 |
| APEX Accelerator | 국방부가 지원하는 무료 정부조달 컨설팅입니다(옛 PTAC). SAM 등록과 입찰 검토를 도와줍니다 |
| VBOC(재향군인 창업센터) | SBA가 지원하는 무료 창업 상담·교육 기관입니다 |
| VA OSDBU | VA의 소기업 담당 부서입니다. 각 조달사무소에 소기업 담당자(Small Business Liaison)가 있습니다 |
| 주정부 프로그램 | 예: 캘리포니아 DVBE(주 계약 3% 목표, 상이 10% 이상·캘리포니아 거주 요건, LLC는 상이군인 100% 소유), 뉴욕 SDVOB(6% 목표). 사는 주의 프로그램을 확인하세요 |

### 2.5 이 무기를 쓸 때 반드시 지켜야 할 것

1. **하도급 제한(LOS):** 서비스 계약은 받은 금액의 50%를 넘게 SDVOSB가 아닌 업체에 하도급 줄 수 없습니다. 같은 자격(SDVOSB)을 가진 업체에 준 하도급은 자체 수행으로 인정됩니다. 물품에는 비제조업체 규칙(NMR)이 적용됩니다. 위반하면 VA가 정지·제재위원회에 회부하고 벌금을 물릴 수 있습니다.
2. **명의대여 단속이 강화되고 있습니다.** 2026년 6월, 명의만 빌린 업체로 SDVOSB 계약 등을 따낸 혐의로 두 회사가 **$2,130만**을 내고 합의했습니다(법무부 발표, 혐의 단계). 내부고발자 보상금만 $367만이었습니다.
3. **SDVOSB 자격 요건:** 상이군인이 지분 51% 이상을 '무조건·직접' 소유해야 하고, 최고 직위를 맡고, 일상 경영을 직접 통제해야 합니다. 다른 곳에서 풀타임으로 일하고 있으면 통제 요건을 갖추지 못한 것으로 추정될 수 있습니다.
4. **TDIU(개인실업 판정으로 100% 지급)를 받는다면 특히 조심하세요.** 사업 순소득이 1인 빈곤선(2026년 약 $1만 6,000 안팎, 출처마다 다름)을 넘으면 TDIU를 잃을 수 있습니다. 일반 장애등급(schedular) 보상금은 소득과 상관없습니다. **사업을 시작하기 전에 VSO나 인가받은 변호사와 꼭 상담하세요.**

---

## 3. 대안 사업 기회 상세

### 3.0 비교표

★ 점수는 이번 조사를 바탕으로 한 필자의 판단입니다.

| 기회 | 고객 | 경쟁 | 진입장벽 | 첫 매출까지 | 초기자본 | 당신 경험과의 적합도 | SDVOSB 효과 | 종합 |
|---|---|---|---|---|---|---|---|---|
| (원안) VA 진료 스크라이브 + CPRS UI + 오더 | VA | 매우 높음 | 매우 높음 | 2~4년 이상 | 매우 큼 | 높음 | 중 | ★☆☆☆☆ |
| ① VSO용 AI 청구 준비 | 카운티·주정부·VSO 단체 | 중(기존 업체는 있으나 AI는 비어 있음) | 낮음~중 | 6~12개월 | 작음 | 매우 높음 | 낮음(주 프로그램에서는 가점) | ★★★★☆ |
| ② DBQ 품질검사·작성 보조 | C&P 검사 위탁업체 | 낮음~중 | 중(대기업 보안 심사) | 9~18개월 | 중 | 높음 | 중 | ★★★☆☆ |
| ③ 커뮤니티 케어 서류 자동화 | 민간 병원·클리닉 | 중 | 낮음~중 | 6~12개월 | 작음~중 | 중 | 낮음 | ★★★☆☆ |
| ④ AI 품질검증·도입지원 서비스 | VA, 원청업체 | 중~높음 | 중(인력·실적 필요) | 6~18개월 | 작음(주로 인건비) | 중 | 매우 높음 | ★★★☆☆ (팀이 있으면 ★★★★☆) |
| ⑤ 재향군인 개인용 앱 | 개인 | 높음(VA 공식 앱과 경쟁) | 낮음 | 불확실 | 작음 | 높음 | 없음 | ★★☆☆☆ |
| ⑥ 물품 납품 | 연방기관 | 높음 | 중(규정 복잡) | 3~12개월 | 중~큼(재고) | 낮음~중 | 높음 | ★★☆☆☆ (자체 제품이 있으면 ★★★☆☆) |

### 3.1 ① 보훈상담관(VSO)용 AI 청구 준비 도구: 1순위 추천

**문제**

카운티·주의 보훈상담관(VSO)은 재향군인의 복무 기록과 VA·민간 의료기록 수백 쪽을 손으로 읽습니다. 그런 다음 양식을 작성합니다: 21-526EZ(장애보상 신청), 21-0966(청구 의사 통지), 21-4138(진술서) 등. 빠진 증거를 확인하고 기한도 관리해야 합니다. PACT Act(소각장 연기·고엽제 등 유해물질 노출 보상 확대법) 이후 청구량이 크게 늘었습니다.

**현재 쓰이는 도구(경쟁사)**

| 제품 | 현황 |
|---|---|
| Tyler Technologies VetraSpec | 30개 이상 주, 300개 이상 카운티, 4,500명 이상 VSO가 사용합니다. VA에 전자 제출을 지원합니다. Tyler가 2021년 DataSpec을 인수했습니다 |
| VetPro (Panoramic Software) | 사우스캐롤라이나 주 전체와 캘리포니아 CalVAR가 채택했습니다. 2026년 텍사스 파커 카운티 입찰에서 낙찰 권고를 받았습니다 |
| Concourse Technology | 카운티 입찰에 참여하고 있습니다 |
| CaseWorthy | 일반 사회복지 사례관리 도구입니다(청구 전용 아님) |
| 테네시 TVB | 테네시 주가 자체 시스템을 만들어 VetraSpec을 대체했습니다 |

→ **생성형 AI 기능을 갖춘 VSO 청구 도구는 이번 조사에서 확인되지 않았습니다.** 기존 업체들은 사례관리, 양식, 전자 제출 위주입니다.

**제품 구상 (원래 아이디어의 '청구 버전')**

1. 기록 업로드(VA Blue Button, 복무 의료기록(STR), 민간 기록) → AI가 진단·증상·날짜·복무 관련성을 **원문 페이지 인용과 함께** 시간순으로 정리
2. 장애등급 기준(38 CFR Part 4)과 비교해 **빠진 증거 체크리스트** 제시
3. 양식 초안을 자동으로 작성 → **VSO가 확인·수정·서명** (사람이 최종 확인)
4. 재향군인용 C&P 검사 준비 안내문 자동 생성
5. VetraSpec이나 VetPro로 내보내기 기능. **기존 제품을 대체하지 않고 보완하는 제품으로 팔면 판매가 훨씬 쉽습니다.**

**판매 방식**

- 카운티: 소액 구매나 공개 입찰(RFP). 공개된 카운티 의회 안건(예: 파커 카운티)에서 낙찰 금액을 찾아 가격 책정에 참고하세요.
- 주 보훈부: 주 전체 라이선스(루이지애나, 사우스캐롤라이나 사례).
- 전국 VSO 단체: DAV, VFW, American Legion 등.

**장벽과 주의점**

- **개인정보·보안:** SOC 2 수준의 보안이 필요합니다. 일부 주는 GovRAMP(옛 StateRAMP) 인증을 요구합니다. 예를 들어 애리조나는 2026년 7월 1일부터 갱신 계약에 이 기준을 적용합니다.
- **VA 연동(선택 사항):** VA 개발자 포털(Lighthouse API)로 일부 연동할 수 있지만, 실제 운영 접근을 얻으려면 VA의 심사와 시연을 거쳐야 합니다.
- **법적 경계선:** 돈을 내는 고객은 **인가받은 VSO 조직**이어야 합니다. 재향군인 개인에게 청구 준비 대가를 받는 모델은 피하세요(38 U.S.C. 5904 등). 비인가 청구 컨설턴트를 형사처벌하는 GUARD VA Benefits Act는 아직 계류 중이지만, 이 분야 규제는 강해지는 방향입니다.
- **리스크:** Tyler가 AI 기능을 추가하거나 VA.gov 자체 기능이 강화될 수 있습니다. 그래서 빨리 레퍼런스를 만들어야 합니다.

**왜 1순위인가:** 진입장벽이 낮고, AI 빈자리가 있고, 적은 돈으로 시작할 수 있습니다. 무엇보다 **당신이 직접 청구해 본 경험이 그대로 제품 설계 능력이 됩니다.**

### 3.2 ② C&P 검사 DBQ 품질검사·작성 보조 AI

**배경**

- C&P(보상·연금) 검사의 85% 이상을 위탁업체가 수행합니다: Leidos QTC, Maximus VES, OptumServe, Loyal Source(LSGS).
- 1~4지역 IDIQ의 최대 물량은 검사 900만 건입니다. **현 계약은 2026년 12월 31일 만료되고(최대 6개월 연장 가능), 6개 지역 전체 재입찰이 진행 중입니다.** 2026년 봄 업계 설명회가 예고됐고, 업무기술서(PWS) 초안 의견 접수는 2026년 8월 12일에 마감됐습니다(보도 기준).
- 하원 보훈위원회 소위원회는 2025년 11월 "DBQ 품질보증 미흡"을 감독 의제로 다뤘습니다.
- 위탁업체에서는 QC 직원이 DBQ의 완전성과 오탈자를 **사람이 직접** 검토합니다(VES 채용공고).
- VA는 DBQ 부정(복사·붙여넣기 등)을 찾아내는 AI 도입을 추진하고 있고, DAV는 검증 방법과 이의제기 절차를 요구하고 있습니다.

**제품 A (먼저, 쉬움): DBQ 품질검사 보조 도구**

- 작성이 끝난 DBQ와 검사 기록을 넣으면 문제를 자동으로 표시합니다:
  - 빠진 항목
  - 평가 기준과 맞지 않는 기재
  - 내부 모순(예: 관절 가동범위 수치와 기능 서술이 서로 다름)
- 결과적으로 **반려와 재작업이 줄어듭니다.**
- 녹음이 필요 없어서 동의·보안 부담이 적습니다.

**제품 B (나중, 어려움): 검사 중 녹음 → DBQ 항목 초안 → 검사의가 최종 확인**

당신 원래 아이디어의 '양식 버전'입니다. 다만 AI가 만든 DBQ가 '복사·붙여넣기'로 오인되지 않도록, **문장마다 근거(녹음 구간, 기록 페이지)를 연결하는 감사 추적 기능**이 핵심 차별점이 되어야 합니다.

**고객:** 4대 위탁업체, 재입찰에 새로 도전하는 업체(기술 차별화가 필요함), VA 의료장애검사실(MDEO).

**경쟁:** 범용 양식 AI(Instafill 등)가 DBQ 자동 작성을 홍보하고 있습니다. 위탁업체용 품질검사 제품은 확인하지 못했습니다. AI 스크라이브 대기업이 C&P 검사로 사업을 넓힐 위험은 있습니다.

**주의:**
- 재향군인이 돈을 내고 민간 의사에게 받는 '민간 DBQ' 시장은 VA의 부정 탐지 대상이 되기 쉽고 평판 위험이 커서 권하지 않습니다.
- 고객이 대기업(Leidos, Maximus, UnitedHealth)이라 보안 심사가 까다롭고 판매 기간이 깁니다.

### 3.3 ③ 커뮤니티 케어(민간 병원)용 VA 서류·기록 반환 자동화

**배경**

- 커뮤니티 케어 계약은 VA 진료의 약 40%를 관리합니다. 차세대 계약(CCN Next Gen)은 10년간 최대 $7,000억 규모로 2026년에 재입찰 중입니다.
- **감사에서 공식 지적된 문제:**
  - 민간 병원은 계약상 30일 안에 진료기록을 VA에 돌려줘야 하지만 자주 늦어지고, VA 직원이 손으로 쫓아다닙니다(3회 요청 규정).
  - 돌려받은 기록을 VA 전자기록에 넣는 비율이 시설마다 크게 다릅니다(11개 시설은 60% 미만).
  - 의뢰 관리 시스템(HSRM)을 쓰지 않는 기관은 이메일이나 팩스로 의뢰를 받습니다.
  - 일부 VA 시설은 아직도 팩스를 씁니다.

**제품:** 민간 클리닉용 도구입니다. VA 의뢰서(HSRM) 수신 → 승인 범위(SEOC)와 유효기간 추적 → 진료 후 VA가 요구하는 서류를 자동으로 묶어 반환 → 기한 알림 → 청구 누락 방지.

**장점:** 고객이 민간이라 VA 운영승인(ATO)이 필요 없습니다(HIPAA BAA와 SOC 2면 됩니다). 고객 수도 많습니다.

**단점:** SDVOSB 이점이 거의 없습니다. 수익관리(RCM) 대기업과 경쟁해야 합니다. CCN 전환기라 업무 흐름이 바뀔 수 있습니다.

**판매:** VA 환자가 많은 전문 클리닉(물리치료, 정형외과, 정신건강 등)부터 시작하세요. TriWest·Optum 네트워크 행사와 주 의사협회도 판로입니다.

### 3.4 ④ VA AI 품질검증·도입지원 서비스 (원청과 하도급)

**배경**

- VA의 AI 사용 사례는 2024년 약 227건에서 **2025년 367건으로 늘었고, 그중 215건이 '고위험(high-impact)'** 입니다. 건강·의료 분야가 가장 많습니다.
- 백악관 예산관리국(OMB) 지침(M-25-21)은 고위험 AI에 **배포 전 테스트, 영향평가, 지속 모니터링**을 의무화했습니다. 기한은 2026년 4월 3일이었고, 지키지 못한 기관이 많다는 보도가 있습니다.
- OIG(2026년 6월)는 AI 채팅 도구를 고위험으로 분류하고 거버넌스와 안전장치를 강화하라고 권고했습니다. GAO는 VA의 AI 거버넌스와 성과 측정 전략이 부족하다고 지적했습니다.
- 보훈급여청(VBA)은 FY2027 예산에서 자동화·AI 투자로 $1.3억을 요청했습니다.
- Oracle EHR 전환으로 FY2027에 26개 병원이 추가됩니다. 현장 교육·지원 인력 수요가 생깁니다.

**서비스 예시**

- AI 진료노트 품질 평가(임상의 패널과 자동 평가 도구), AI 레드팀(일부러 오류를 유도하는 테스트), 편향 점검
- Section 508 접근성 테스트, AI 영향평가 문서 작성
- 병원별 AI 스크라이브·EHR 전환 현장 밀착 교육(at-the-elbow)

**들어가는 방법:** SDVOSB 전용 입찰이나 수의계약($35만 이하, $500만 이하), 그리고 원청업체(Rise8, Carahsoft, Abridge, Knowtex, Leidos 등)의 하도급.

**필요 조건:** 임상정보학이나 데이터과학 인력 같은 실제 역량이 필요합니다. 하도급 제한 규정상 **인건비의 50% 이상을 자기 직원이 수행**해야 합니다. 혼자서는 어렵고, 사람을 뽑거나 파트너가 있어야 합니다.

### 3.5 ⑤ 재향군인 개인용 앱 (보조 아이디어)

- 기능 예: 진료 준비와 요약, 약 관리, C&P 검사 준비, 기록 정리(VA Lighthouse 환자건강 API, FHIR 표준 활용).
- 단점: 개인 고객으로는 돈을 벌기 어렵고 VA 공식 앱과 경쟁해야 합니다. **VA 시설 안에서 녹음하는 것도 자유롭지 않습니다.** 2017년 VA가 환자 녹음을 허용하는 메모를 냈다가 노조가 문제를 제기해 철회했습니다(노조 측 발표).
- → 단독 사업보다는 ①을 알리는 홍보 채널로 쓰기를 권합니다.

### 3.6 ⑥ 정부에 물건 납품하기

**두 가지 사업 모델**

| 모델 | 설명 | 핵심 규정 |
|---|---|---|
| 제조업체 | 직접(또는 내 통제 아래) 생산 | 물품 계약은 재료비를 뺀 비용의 50% 이상을 직접 수행해야 합니다(LOS) |
| 비제조업체(유통·재판매) | 다른 회사 제품을 사서 납품 | 비제조업체 규칙(NMR): 원칙적으로 **미국 소기업이 미국에서 만든 제품**이어야 합니다. SBA 면제(waiver)가 있으면 예외입니다. 직원 500명 이하이고 소매·도매가 주업이어야 합니다. 일반 소기업 세트어사이드는 $1만 5,000~$35만 구간에서 이 규칙이 면제되지만, SDVOSB 세트어사이드는 금액과 상관없이 적용됩니다 |

**공통 규정**

- 무역협정법(TAA): GSA 스케줄 등에서는 중국산을 납품할 수 없습니다.
- 미국산 우선법(Buy American Act): 미국산 부품 비율이 2024~2028년 65%, 2029년부터 75%여야 합니다.
- 국방부 섬유·식품에는 Berry 수정조항이 적용됩니다.
- AbilityOne(장애인 고용 단체 우선 공급 제도)에 지정된 품목은 경쟁 입찰 자체가 불가능합니다. SKILCRAFT 브랜드로 3,000개 이상 품목이 있고, 청소·위생·개인관리 용품도 포함됩니다.
- 캘리포니아 주 기관은 교도소 산업청(CALPIA) 생산 품목(비누, 청소·세탁·주방용 화학제품 등)을 먼저 사야 하고, 다른 곳에서 사려면 면제를 받아야 합니다. 카운티·시 같은 지방정부에는 적용되지 않습니다.

**판매 채널**

| 채널 | 특징 |
|---|---|
| 구매카드 소액구매($1만 5,000 이하) | 지역 VA 병원 구매 담당자에게 직접 영업합니다. GSA 스케줄이 있으면 GSA Advantage 쇼핑몰에 올라가 구매가 쉬워집니다 |
| GSA 스케줄(MAS) | 연방기관 공통 구매 목록입니다. 등록에 수개월이 걸리고 가격이 공개됩니다 |
| DLA DIBBS | 국방물류청 입찰 게시판입니다. 군 부품·물자 소액 견적 요청(RFQ)이 많습니다. CAGE 코드가 필요합니다 |
| VA MSPV 대체조달 | 의료·수술 소모품입니다. 2026년 6월 VA 조달사무소가 SDVOSB 전용 공고를 냈습니다. 같은 해 다른 MSPV 공고에서는 제조사나 공식 유통사(승인서 필요)만 낙찰받을 수 있다고 답했습니다 |
| VA 국가조달센터 연방공급목록(FSS) | 의료기기, 의약품 등 |

**현실**

- 일반 상품 재판매는 **마진이 얇고 가격 경쟁이 심합니다.** 명의대여나 비제조업체 규칙 위반으로 단속되기도 쉽습니다.
- 보조기·보철(Prosthetics) 구매에는 SDVOSB 우선 원칙이 적용되지 않는다고 본 GAO 결정 사례가 있습니다(재향군인의 선택권 등이 이유).
- **해볼 만한 경우:**
  1. 직접 만들 수 있는 제품이 있을 때
  2. 제조사 독점·공식 유통권을 확보했을 때
  3. 장애를 겪으며 떠올린 보조기구 아이디어가 있을 때

  다만 어느 경우든 소프트웨어보다 자본(재고, 인증)이 더 듭니다.
- 녹음 장치처럼 하드웨어와 소프트웨어를 결합한 제품은 VA 보안 인증이 더 어려우니 초기에는 피하세요.

---

## 4. 실행 로드맵 (추천)

### 4.0 먼저 스스로 확인할 다섯 가지

1. **TDIU를 받고 있나요?** 그렇다면 사업 구조와 소득 계획부터 VSO나 변호사와 상의하세요.
2. **풀타임으로 회사를 운영할 수 있나요?** SDVOSB 통제 요건과 관련됩니다.
3. **개발을 직접 할 수 있나요, 아니면 기술 공동창업자가 있나요?** AI 코딩 도구로 혼자 MVP(최소 기능 제품)를 만들 수는 있습니다. 하지만 보안과 정확성에 대한 책임은 그대로입니다.
4. **6~12개월 버틸 자금이 있나요?** 생활비와 초기 비용을 함께 생각하세요.
5. **어느 주에 살고 있나요?** 주정부의 상이군인 기업 프로그램을 확인하세요.

### 4.1 이번 달 (2026년 11월)

- [ ] **NVSBE 2026 등록:** 2026년 12월 8~9일, 오하이오주 클리블랜드 Huntington Convention Center. VA와 다른 연방기관의 구매 담당자, 원청업체를 직접 만나는 VA 최대 행사입니다. 등록: <https://portal.vetbiz.va.gov/nvsbe/2026/>
- [ ] **APEX Accelerator** 무료 상담 예약 (<https://www.apexaccelerators.us>에서 지역 사무소 찾기)
- [ ] **VBOC** 무료 상담 (sba.gov에서 "VBOC" 검색)
- [ ] **VR&E 상담** 신청: 자영업 트랙을 이용할 수 있는지 확인

### 4.2 0~3개월

- [ ] 회사 설립. 연방 SDVOSB는 본인 지분 51% 이상이면 되지만, **캘리포니아 DVBE까지 받으려면 LLC는 상이군인이 100% 소유**해야 합니다(51% 구조라면 주식회사 형태). 운영계약서에 본인의 통제권을 명시하세요. EIN 발급, 사업용 계좌 개설.
- [ ] **SAM.gov 등록**(UEI 번호, CAGE 코드). **무료입니다.** 돈을 받는 대행 사기를 조심하세요.
- [ ] **SBA VetCert SDVOSB 인증 신청** (<https://veterans.certify.sba.gov>). 무료입니다. DD-214, VA 장애등급 통지서, 정관·운영계약서 등이 필요합니다. 3년간 유효하고, 처리 기간은 보도마다 다릅니다(수주~수개월).
- [ ] NAICS(업종) 코드 선택:
  - 541511: 맞춤형 프로그래밍
  - 541512: 컴퓨터 시스템 설계
  - 541519: 기타 컴퓨터 서비스 (VA Ambient Scribe SBIR 공고에 쓰인 코드)
  - 513210: 소프트웨어 출판
  - 518210: 데이터 처리·호스팅
  - 541611: 경영 컨설팅
- [ ] 한 페이지짜리 회사 역량 소개서(Capability Statement) 작성
- [ ] **잠재 고객 인터뷰 20~30건:** VSO 10명, C&P 검사 관계자 5~10명, VA 환자가 많은 민간 클리닉 5~10곳. 인터뷰 결과로 집중할 분야 하나를 정하세요.

### 4.3 3~9개월

- [ ] MVP 개발. 처음에는 가짜 데이터나 공개 데이터로 만드세요. 실제 환자 정보는 보안을 갖춘 뒤에 다룹니다.
- [ ] HIPAA 정책과 BAA(업무제휴계약) 준비, SOC 2 Type I 준비(자동화 도구 활용)
- [ ] 무료 또는 저가 시범 운영 1~3곳 → 레퍼런스 확보
- [ ] SAM.gov의 RFI(정보요청)·Sources Sought(업체 수요조사)에 응답하기. VA가 SDVOSB 전용 입찰로 할지 결정할 때 근거가 됩니다. VA 조달 예측(Forecast)도 꾸준히 확인하세요.
- [ ] VA 상금 대회(Tech Sprint 등) 공지 확인하기. 2024년 Tech Sprint 수상자들은 실제로 수의계약을 받았습니다. (정정: 상금 대회 모음 사이트인 Challenge.gov는 2026년 3월 30일 문을 닫았고, 대회 정보는 USA.gov 등 다른 정부 사이트로 옮겨졌습니다.)

### 4.4 9~24개월

- [ ] 유료 고객을 늘리고 과거 실적(Past Performance) 쌓기
- [ ] SBIR 신청(국방부, 국방보건청(DHA), NIH 등) → Phase III 경로 확보
- [ ] GSA 스케줄 등록 검토, Polaris SDVOSB나 VETS 2 보유 업체와 팀 구성
- [ ] 이때가 돼서야 VA에 직접 SaaS 판매를 검토하세요. FedRAMP 20x를 받거나 VA 클라우드 안에 배포하는 방식이 있습니다.

### 4.5 비용 감각

대략적인 수치입니다. 이번 조사에서 공식 수치는 확인하지 못했습니다.

| 항목 | 대략적인 비용 |
|---|---|
| SAM.gov 등록, VetCert 인증 | $0 |
| 법인 설립, 회계, 보험 | 수천 달러 |
| MVP 개발 | 혼자 AI 도구로 하면 수천~수만 달러. 외주를 주면 훨씬 비쌉니다 |
| SOC 2 Type I | 도구와 감사비를 합쳐 수만 달러 수준이라는 것이 업계 통념입니다 |
| FedRAMP | 전통 방식으로는 수십만~백만 달러 이상이라는 것이 업계 통념입니다(20x로 줄어드는 추세). 초기에는 피하세요 |

---

## 5. 하지 말아야 할 것

| 하지 말 것 | 이유 |
|---|---|
| CPRS 복제품을 만들어 VistA에 직접 오더 쓰기 | 퇴역 중인 시스템이고, 환자안전·접근권한·인증 문제가 있습니다 |
| 명의대여(rent-a-vet) | 허위청구법(False Claims Act) 적용 대상입니다. 2026년 6월 $2,130만 합의 사례가 있습니다 |
| 재향군인에게 비인가로 청구 대행료 받기 | 38 U.S.C. 5904 위반 소지가 있고, 규제가 강해지는 추세입니다 |
| 동의 없이 진료 녹음하기 | VA 정책과 주법(녹음 동의 규정) 위반 소지가 있습니다 |
| 보안 없이 실제 환자 데이터 다루기 | HIPAA 위반입니다 |
| AI가 오더를 '실행'하게 만들기 | 항상 사람이 승인해야 합니다. AI는 초안까지만 만드세요 |
| TDIU를 받으면서 소득 계획 없이 사업 시작하기 | 수당을 잃을 수 있습니다 |

---

## 6. 최종 의견

제가 당신이라면 이렇게 하겠습니다.

1. **이번 달:** NVSBE에 등록하고 APEX·VBOC·VR&E 상담을 신청합니다. SAM.gov 등록과 VetCert 인증도 시작합니다.
2. **제품:** ① VSO용 AI 청구 준비 도구로 작게 시작합니다. 직접 겪어 본 청구 과정이 가장 큰 경쟁력입니다.
3. **병행:** ② DBQ 품질검사 도구는 재입찰에 나서는 업체와 파트너십을 알아봅니다. NVSBE에서 만날 수 있습니다.
4. **VA 본부를 상대로 한 대형 SaaS**는 실적과 보안 인증이 쌓인 2~3년 차의 목표로 둡니다.

원래 아이디어를 버리라는 뜻이 아닙니다. **같은 기술을 경쟁자가 없는 서류 업무에 먼저 적용하세요. 그렇게 SDVOSB 지위와 실적을 쌓은 다음에 VA 본진으로 들어가는 순서**를 추천합니다.

---

## 7. 용어 정리

| 용어 | 뜻 |
|---|---|
| SDVOSB / VOSB | 상이군인 소유 소기업 / 재향군인 소유 소기업 |
| VetCert | SBA의 재향군인 기업 인증 프로그램(2023년 VA에서 SBA로 이관) |
| SAM.gov, UEI, CAGE | 연방 조달업체 등록 사이트, 고유 사업자 번호, 국방 조달 코드 |
| NAICS | 업종 분류 코드. 업종마다 소기업 기준이 다름 |
| SAT / 소액구매 기준 | 간이구매 기준액 $35만 / 구매카드 기준액 $1만 5,000 (2025년 10월 1일부터) |
| IDIQ | 수량·시기를 정하지 않은 다년 계약. 실제 일은 작업 주문(Task Order)으로 발주 |
| GWAC / GSA 스케줄(MAS) / BPA | 정부 공용 IT 계약 / GSA 공통 구매 목록 / 반복 구매 협정 |
| RFI / Sources Sought / RFP | 정보 요청 / 업체 수요 조사 / 제안 요청(입찰) |
| LOS / NMR | 하도급 제한 규정 / 비제조업체 규칙 |
| TAA / Buy American | 무역협정법(원산지 제한) / 미국산 우선법 |
| ATO / FedRAMP / GovRAMP | 운영승인 / 연방 클라우드 보안 인증 / 주·지방정부 클라우드 보안 인증(옛 StateRAMP) |
| SBIR / STTR | 중소기업 기술혁신 연구개발 지원 제도(Phase I → II → III) |
| C&P / DBQ | 보상·연금 검사 / 장애급여 질문서(검사 결과 표준 양식) |
| VSO | 보훈상담관 또는 보훈단체(인가받은 청구 대리인) |
| CCN / HSRM / SEOC | 커뮤니티 케어 네트워크 / 의뢰 관리 시스템 / 승인된 진료 범위 |
| CPRS / VistA / EHR | VA 기존 의사용 화면 / VA 기존 전자기록 시스템 / 전자건강기록 |
| TDIU | 개인실업 판정(일을 못 해서 100% 등급으로 지급) |
| VR&E | 직업재활·고용 프로그램(Chapter 31) |

---

## 8. 출처

### VA AI 스크라이브와 계약
- [VA to gradually expand medical AI scribe following successful pilots — Nextgov (2026.9)](https://www.nextgov.com/artificial-intelligence/2026/09/va-gradually-expand-medical-ai-scribe-following-successful-pilots/415971/)
- [Abridge selected for VA ambient AI enterprise contract — Nextgov (2026.9)](https://nextgov.com/artificial-intelligence/2026/09/va-selects-abridge-ambient-scribe-under-new-enterprise-contract/416140)
- [Abridge ambient AI now available at all VA medical centers — Healthcare IT News](https://www.healthcareitnews.com/news/abridge-ambient-ai-now-available-all-va-medical-centers)
- [Abridge 보도자료 — BusinessWire](https://secure.businesswire.com/news/home/20260909133398/en/Abridge-Selected-to-Bring-Ambient-AI-to-Veteran-Care-Nationwide-Through-the-VA-Enterprise-Contract)
- [Abridge awarded VA enterprise contract — HIT Consultant (2026.9.22)](https://hitconsultant.net/2026/09/22/abridge-awarded-va-enterprise-contract-ambient-ai-veterans-health-administration/)
- [VA $775M ambient scribe IDIQ 분석 — ibl.ai (경쟁사 블로그, 참고용)](https://ibl.ai/blog/va-775m-ambient-scribe-idiq-ceiling-workflow-evaluation)
- [Rise8, Thoughtworks land VA contract for AI tech rollout — ExecutiveBiz](https://www.executivebiz.com/articles/rise8-thoughtworks-va-ambient-scribe-ai-contract)
- [VA plans to roll out Abridge and Knowtex AI tech — Fierce Healthcare](https://www.fiercehealthcare.com/ai-and-machine-learning/va-plans-roll-out-abridge-and-knowtex-ai-tech-across-its-health-system)
- [VA awards AI Tech Sprint winners pilot contracts — FedScoop](https://fedscoop.com/va-awards-ai-tech-sprint-winners-pilot-contracts-for-ambient-medical-transcription-services/)
- [VA awards Abridge and Nuance AI contract — Healthcare IT News](https://www.healthcareitnews.com/news/va-awards-abridge-and-nuance-ai-contract-help-reduce-burnout)
- [Special Notice: Ambient Scribe Sole Sources — HigherGov](https://www.highergov.com/contract-opportunity/special-notice-ambient-scribe-sole-sources-36c10g25q0088-s-34ad3)
- [Ambient Scribe Post-MVP Development Support (SBIR) 사전 공고 — cleat.ai](https://www.cleat.ai/government/contracts/ambient-scribe-post-mvp-development-support-sbir-va-26-00037387-jvtd)
- [Ambient AI scribe to roll out in Primary Care — VA Providence](https://www.va.gov/providence-health-care/stories/ambient-artificial-intelligence-ai-scribe-assistance-to-roll-out-in-primary-care/)

### Oracle, 국방부, VA EHR 전환
- [Oracle Health adds order creation capabilities to Clinical AI Agent — Oracle (2026.2.2)](https://www.oracle.com/news/announcement/oracle-health-adds-order-creation-capabilities-to-clinical-ai-agent-2026-02-02/)
- [AI Scribe technology reduces notetaking — DHA (2026.4.23)](https://dha.mil/News/2026/04/23/15/23/AI-Scribe-technology-for-medical-professionals-reduces-notetaking)
- [Pentagon rolls out AI medical scribe in clinical care — GovCIO](https://govciomedia.com/pentagon-rolls-out-ai-medical-scribe-in-clinical-care/)
- [VA resumes EHR rollouts at four Michigan medical sites — Nextgov (2026.4)](https://www.nextgov.com/modernization/2026/04/va-resumes-ehr-rollouts-four-michigan-medical-sites/412807/)
- [VA to complete Federal EHR deployment at nine additional sites in 2026 — VA](https://www.va.gov/philadelphia-health-care/news-releases/va-to-complete-federal-ehr-deployment-at-nine-additional-sites-in-2026/)
- [VA deploys Oracle Health EHR at 4 more sites — Becker's](https://www.beckershospitalreview.com/healthcare-information-technology/ehrs/va-deploys-oracle-health-ehr-at-4-more-sites/)
- [EHR rollout on track, VA Deputy Secretary tells Congress — GovCIO](https://govciomedia.com/ehr-rollout-on-track-va-deputy-secretary-tells-congress/)

### AI 스크라이브 시장
- [PitchBook Analyst Note: Healthtech AI Scribes (Q4 2025)](https://files.pitchbook.com/website/files/pdf/Q4_2025_PitchBook_Analyst_Note_Healthtech_AI_Scribes_20233.pdf)
- [Abridge — Sacra](https://sacra.com/c/abridge/)
- [Why the crowded market for ambient AI scribes might be ripe for consolidation — STAT (Benzinga 게재)](https://benzinga.com/content/42061994/stat-why-the-crowded-market-for-ambient-ai-medical-scribes-might-be-ripe-for-consolidation)

### VA의 AI 거버넌스와 감사
- [Review of VHA's use of generative AI (예비 경고) — VA OIG](https://www.vaoig.gov/reports/preliminary-result-advisory-memorandum/review-vhas-use-generative-artificial-intelligence)
- [VA OIG 최종보고서 (2026.6)](https://vaoig.gov/sites/default/files/reports/2026-06/vaoig-26-00182-140_-_final.pdf)
- [VHA's AI use poses potential patient safety risk — MeriTalk](https://meritalk.com/articles/vhas-ai-use-poses-potential-patient-safety-risk-oig-says/)
- [VA inventory reveals 367 AI systems — Military Times (2026.6)](https://www.militarytimes.com/veterans/2026/06/23/va-inventory-report-reveals-367-ai-systems-operating-in-healthcare-benefits-and-services/)
- [VA's latest AI inventory — Nextgov (2026.2)](https://www.nextgov.com/artificial-intelligence/2026/02/vas-latest-ai-inventory-includes-new-suicide-ehr-focused-use-cases/411270)
- [VA AI adoption strategy — ExecutiveGov](https://www.executivegov.com/articles/va-ai-adoption-strategy-healthcare)
- [VA increasingly looking to AI to enhance claims processing — Nextgov (2026.3)](https://nextgov.com/artificial-intelligence/2026/03/va-increasingly-looking-ai-enhance-claims-processing/411900)
- [AI is helping VA speed up claims processing — Nextgov (2026.4)](https://www.nextgov.com/artificial-intelligence/2026/04/ai-helping-va-speed-claims-processing-dems-worry-about-errors/412904/)
- [OMB 고위험 AI 요구사항 해설 — Winston & Strawn](https://www.winston.com/en/insights-news/white-house-memorandum-elaborates-on-prior-executive-order-with-requirements-for-high-impact-ai-used-by-federal-agencies)

### SDVOSB와 조달 제도
- [Service-Disabled Veteran-Owned Small Business Contracting Program — CRS (2026.9)](https://www.everycrsreport.com/reports/IF13309.html)
- [Sole-Source Contracts for Small Businesses — CRS (2026.9)](https://www.everycrsreport.com/files/2026-09-08_IF12853_1e52e52e6b1bbb89888dc2346415d62257b05b95.html)
- [VAAR 806.302-570 Noncompetitive procedures — acquisition.gov](https://www.acquisition.gov/vaar/806.302-570-noncompetitive-procedures-verified-veteran-owned-small-businesses.)
- [FAR inflation adjustment of thresholds — Federal Register (2025.8.27)](https://www.federalregister.gov/documents/2025/08/27/2025-16412/federal-acquisition-regulation-inflation-adjustment-of-acquisition-related-thresholds)
- [Congress increases SDVOSB prime contracting goal to 5% — Orange Slices](https://orangeslices.ai/congress-increases-sdvosb-prime-contracting-goal-to-five-percent/)
- [SDVOSB certification guide — USFCR](https://blogs.usfcr.com/sdvosb-certification)
- [VA OSDBU](https://www.va.gov/osdbu/) · [NVSBE 2026](https://www.va.gov/osdbu/nvsbe/) · [2026 NVSBE — VA News](https://news.va.gov/149848/2026-national-veterans-small-business-engagement/)
- [$21.3M SDVOSB fraud settlement — Sheppard Mullin](https://www.sheppard.com/insights/blogs/a-recent-21-3-million-settlement-to-resolve-alleged-sdvosb-fraud-scheme-signals-a-renewed-focus-on-protecting-veterans-and-small-businesses)
- [VA notice of limitations on subcontracting — HigherGov](https://www.highergov.com/document/va-notice-of-limitations-on-subcontracting-pdf-ad89c0/)
- [VA class deviation restricts SDVOSB nonmanufacturers — SmallGovCon](https://smallgovcon.com/service-disabled-veteran-owned-small-businesses/va-class-deviation-restricts-sdvosb-nonmanufacturers/)
- [EO 14240 (GSA 조달 통합) 해설 — Cooley](https://www.cooley.com/news/insight/2025/2025-03-25-new-executive-order-will-consolidate-federal-agency-procurement-functions-in-the-general-services-administration)
- [VA says GSA partnership is streamlining procurement for IT — GovCIO](https://govciomedia.com/va-says-gsa-partnership-is-streamlining-procurement-for-it/)
- [GSA second batch of Polaris SDVOSB awards — Washington Technology (2026.6)](https://www.washingtontechnology.com/contracts/2026/06/gsa-gets-moving-second-batch-polaris-sdvosb-awards/414094/)
- [VETS 2 — GSA](https://gsablogs.gsa.gov/technology/tag/vets2/)
- [SBIR/STTR reauthorized after six-month lapse — Crowell & Moring](https://www.crowell.com/en/insights/client-alerts/sbirsttr-programs-reauthorized-after-six-month-lapse)
- [Rise8 VA.gov Watchtower SBIR Phase III — Orange Slices](https://orangeslices.ai/rise8-scores-9-5m-va-gov-watchtower-sbir-phase-iii-contract-win)
- [GAO reaffirms agencies' discretion for SBIR Phase III — JD Supra](https://production.jdsupra.com/legalnews/gao-reaffirms-agencies-discretion-to-1788340)
- [FedRAMP 20x: what it is and where it stands — A-LIGN](https://www.a-lign.com/articles/fedramp-20x-what-it-is-and-where-it-stands-today) · [FedRAMP news 2026 — Knox](https://knoxsystems.com/resources/fedramp-news-2026)
- [StateRAMP rebrands to GovRAMP — GovRAMP](https://govramp.org/blog/stateramp-announces-rebrand-to-govramp-reflecting-mission-to-unite-public-and-private-sectors-in-advancing-cybersecurity/)

### 재향군인 개인 지원과 주의사항
- [38 CFR 21.257 Self-employment — govinfo](https://govinfo.gov/content/pkg/CFR-2023-title38-vol2/pdf/CFR-2023-title38-vol2-sec21-257.pdf)
- [Self-employment approval — DisabledVeterans.org](https://www.disabledveterans.org/self-employment-approval-easier/)
- [TDIU income limits 2026 — CCK Law](https://cck-law.com/blog/what-are-the-new-tdiu-income-limits-in-2026/)
- [California DVBE — Caltrans](https://dot.ca.gov/programs/civil-rights/dvbe) · [California LAO 보고서](https://www.lao.ca.gov/Publications/Report/5095)
- [New York SDVOB — NYS EFC](https://efc.ny.gov/node/31)

### 니치 시장 (C&P/DBQ, VSO, 커뮤니티 케어)
- [List of VA C&P exam contractors — VA Claims Insider (상업 사이트, 참고용)](https://vaclaimsinsider.com/list-of-va-c-and-p-exam-contractors/)
- [House VA subcommittee hearing document (2025.11.20)](https://docs.house.gov/meetings/VR/VR09/20251120/118660/HHRG-119-VR09-20251120-SD004.pdf)
- [MDE industry day notice — HigherGov](https://www.highergov.com/contract-opportunity/r499-industry-day-for-medical-disability-examinat-36c10x26q0089-r-ab014/)
- [Maximus VES medical disability exam contracts — Maximus](https://maximus.com/news/veterans-medical-disability-exam-services)
- [VES Quality Control Analyst 채용공고](https://hireaniner.charlotte.edu/jobs/a37723e5337eabea6f98068d157acb9b)
- [DAV, VA의 DBQ 부정 탐지 AI에 대한 우려 — DAV](https://www.dav.org/?p=32792)
- [AI DBQ form automation case study — Instafill.ai (업체 자료)](https://instafill.ai/case-studies/va-dbq-spine-and-joint-form-automation)
- [Tyler Veterans' Benefits (VetraSpec) — Tyler Technologies](https://www.tylertech.com/solutions/health-human-services/disability-benefits/veterans-benefits)
- [Tyler buys DataSpec — GovConWire](https://www.govconwire.com/articles/tyler-technologies-buys-veterans-claims-management-software-company-dataspec-lynn-moore-quoted)
- [Parker County, TX 2026 claims software RFP 안건](https://www.parkercountytx.gov/AgendaCenter/ViewFile/Agenda/_02232026-967)
- [SC county VA offices new claims system — SCDVA](https://scdva.sc.gov/news/sc-county-veterans-affairs-offices-utilizing-new-claims-management-system)
- [LDVA launches VetraSpec — Louisiana](https://vetaffairs.la.gov/ldva-launches-new-claims-management-software-vetraspec-to-streamline-services-provided-to-louisiana-veterans-and-their-families)
- [GUARD VA Benefits Act 진행 상황 — MOAA (2026)](https://www.moaa.org/content/publications-and-media/news-articles/2026-news-articles/advocacy/bipartisan-bill-takes-another-step-toward-protecting-veterans-from-predatory-claims-companies/)
- [GAO-20-643 Community care — GAO](https://www.gao.gov/products/GAO-20-643)
- [HSRM User Guide — VA](https://www.va.gov/COMMUNITYCARE/docs/providers/HSRM-User-Guide.pdf)
- [Senate VA Committee document (커뮤니티 케어 기록)](https://www.veterans.senate.gov/services/files/1DB11AF2-B4C8-4029-BE5A-3DABD5D1D07B)
- [VA CCN procurement enters proposal phase — Polsinelli](https://www.polsinelli.com/publications/va-ccn-procurement-enters-proposal-phase)
- [VA API standards: environments — VA (GitHub)](https://department-of-veterans-affairs.github.io/va-api-standards/general-guidelines/environments)
- [VA agrees to rescind patient recording policy — NFFE (노조 발표)](https://nffe.org/nffe_news/triumph-va-agrees-to-rescind-patient-recording-policy-after-nffe-takes-action-to-protect-employee-and-patient-privacy-patient-privacy)

### 물품 납품
- [MSPV Alternative Sourcing (36C24126Q0470) — HigherGov](https://www.highergov.com/contract-opportunity/6515-mspv-alternative-sourcing-36c24126q0470-k-6890f/)
- [VA SDVOSB set-asides not required for prosthetics (GAO) — SmallGovCon](https://smallgovcon.com/tag/sdvosb-set-asides/page/4/)

### 공식 사이트 (바로가기)
- [SAM.gov](https://sam.gov) · [SBA VetCert](https://veterans.certify.sba.gov) · [APEX Accelerators](https://www.apexaccelerators.us) · [VA 개발자 포털](https://developer.va.gov) · [Challenge.gov(2026.3.30 종료, 이전 안내)](https://www.challenge.gov) · [FedRAMP](https://www.fedramp.gov) · [USAspending](https://www.usaspending.gov) · [DLA DIBBS](https://www.dibbs.bsm.dla.mil)
