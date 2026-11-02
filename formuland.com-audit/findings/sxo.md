Score: 61/100

SXO Gap Score (Search Experience). This is separate from the SEO Health Score. Audit date: 2026-11-02.

**SERP caveat.** Every SERP observation below comes from one point-in-time sample, taken with the WebSearch tool on 2026-11-02 from a single location whose backend engine is unknown. It is NOT a verified Google SERP. "Order" means the order results were returned in, not confirmed Google positions. No PAA, Shopping, ads, AI Overview, featured snippet or related-search data was visible. When this file says formuland was "absent", it means absent from that sample only.

## Query → target page → SERP → mismatch summary

| # | Money query (sample) | Formuland target | What the sample returned (taxonomy type) | Formuland in sample? | Mismatch |
|---|---|---|---|---|---|
| 1 | "european baby formula" | / (homepage = Landing/Product listing) | Top 10 were 10/10 editorial or Blog (childrenscolorado.org, yahoo.com x2, hibobbie.com blog, sciencefriday.com, forward2me blog, kabrita blog, plus sponsored/spam domains). 0 retailers. | No | **HIGH** on the head term |
| 1b | "buy european baby formula online USA" | (same) | 6/9 retailer homepages or PLPs (europebabyformula.com, organicbabyformula.shop x2, organicsbestshop.com, euromallusa.com/collections/baby-formulas, formuland), 3/9 "where to buy" guides (sheknows, organiclifestart x2) | Yes, but via **/collections/organic-baby-formula**, not the homepage | ALIGNED by type, wrong URL |
| 2 | "hipp formula" / "hipp formula usa" | /collections/hipp-formula (Hybrid PLP) | The unqualified sample was mostly spam, with no HiPP official site. "hipp formula usa": 4/9 PLPs (formuland, organiclifestart, mommyformula, groworganicbaby), 5/9 "where to buy HiPP in USA" guides (hippformulausa.com, heybabymom, organicformulaeurope, euromallusa x2) | Yes, /collections/hipp-formula (5th in order) | **ALIGNED** |
| 3 | "hipp stage 1" / "hipp formula stage 1 buy USA" | /products/hipp-formula-stage-1 (PDP, UK recipe only) | Unqualified: mostly hip-osteoarthritis pages. Relevant results were carrefouruae PDP, cimri.com price comparison, dm.at, and the organiclifestart guide "HiPP Pre vs Stage 1". Commercial: 5/10 PLPs, 4/10 PDPs, 1 instructions page | Yes, 4 times (/collections/hipp-dutch-formula, /collections/hipp-formula, /products/hipp-ha-stage-1, /collections/hipp-formula-stage-1), but **never the target PDP** | **MEDIUM** |
| 4 | "goat milk formula" / "european goat milk baby formula buy" | /collections/goat-milk-formula (Hybrid PLP) | Head term top 10: 8/10 editorial, research or brand (pickyeaterblog "Best Goat Milk Formula" x2, kabrita blog, apollopharmacy x2, British Journal of Midwifery, doaj, oalib), 2/10 PLP or marketplace. Further down: best-of reviews (mommyhood101 "Tested & Reviewed", gimmethegoodstuff). Commercial: 9/9 retailer PLPs or PDPs | Head: no. Commercial: yes (3rd) | **MEDIUM** head / ALIGNED commercial |
| 5 | "european baby formula comparison (chart)" | /pages/european-baby-formula-comparison-chart (Comparison) | "comparison": editorial (hibobbie, kendamil blog, UCSF video, yahoo, thequalityedit). "comparison chart": 3 competitor retailer chart pages (organic-formula.com, organiclifestart.com/pages/…, organicformulashop.com/pages/…) plus buyer's guides | No | Type ALIGNED, format **CRITICAL** (see F1) |

## What works

- **The HiPP collection matches its SERP (hybrid PLP).** /collections/hipp-formula has 3,220 main-content words, a 40-row HTML table (Formula/Line/Type/Stage/Age/Milk/Size), a 6-row stage table, an H2 "HiPP German vs Dutch vs UK: What's the Difference?" and an honest FAQ: "Is HiPP formula FDA approved? No. The FDA doesn't formally 'approve' any infant formula… HiPP's European formulas are not on the FDA's list". Above the fold (screenshots/hipp-formula-desktop.png) it shows the H1, a sourcing and delivery promise, Trustpilot "Excellent, 4,298 reviews" and Age / Formula Type filters. It appeared for "hipp formula usa" and "hipp formula stage 1 buy USA".
- **The PDP decision block is strong on desktop.** screenshots/hipp-formula-stage-1-desktop.png shows the H1, "4.91 · 107 reviews on Judge.me", "No hidden costs at your door. All European import duties are covered by us", "Arrives Wed, Nov 4 – Mon, Nov 9 · insured & tracked", subscribe vs one-time options, and bundle tiers from 1 to 12 cans ($49.99 down to $45.99 per can). On mobile a sticky "$49.99 ADD TO CART" bar is visible.
- **The PDP intro explains the version difference.** "Unlike HiPP's German and Dutch recipes, the UK version relies on prebiotics alone…" (raw/pages/products_hipp_formula_stage_1.html).
- **The goat collection answers the safety signal from the SERP.** The sample said goat formula is unsuitable for CMPA. The page has a 7-row goat vs cow HTML table and says "Allergy Note: Not suitable for confirmed cow's milk protein allergy due to cross-reactivity" and "always consult your pediatrician before switching".
- **Product schema merges correctly.** Both PDP Product nodes share `@id` /products/hipp-formula-stage-1#product, so the offer (49.99 USD, InStock) and aggregateRating (4.91, 107) combine.
- **Formuland appeared in 4 of 5 commercial-modifier samples**, so the gaps below concern page type and experience rather than discoverability.

## Findings

### F1. The comparison chart page has no indexable comparison: it is 3 images and 5 words (page-type format mismatch)
Severity: Critical
Evidence: The `<main>` of https://formuland.com/pages/european-baby-formula-comparison-chart (raw/pages/pages_european_baby_formula_comparison_chart.html) is an H1 plus 3 `<img>` tags (Chart2_1.webp 2000x1320, Chart.ai_1_-2_page-0001.jpg 2000x1036, Chart_1.webp 2000x1797) with alt text "European baby formula comparison chart, part 1/2/3". crawl.jsonl records words=5, h2_count=0. A rendered check in Chromium found 0 `<table>`, 0 H2, 0 links to /products/, and Organization/WebSite schema only. The Shopify `?v=` upload timestamps are 2024-09-04 and 2024-10-29. Competitor chart pages in the same sample, rendered in Chromium on 2026-11-02:
- organicformulashop.com/pages/baby-formula-comparison-chart: 5 HTML tables (14–24 rows), 4,319 words, 14 H2s ("HiPP vs Holle vs Kendamil UK at a glance", "Every column explained", FAQ), 46 product links, BreadcrumbList.
- organiclifestart.com/pages/european-baby-formula-comparison-chart: 8 HTML tables (up to 27 rows), 1,713 words.

Formuland already builds HTML tables elsewhere (the 40-row table on /collections/hipp-formula).
Recommendation: Rebuild the page as an HTML comparison page using the criteria already promised in its meta description:
- One `<table>` per milk type (cow, goat, HA/specialty). Rows = brand and line (HiPP German/Dutch/UK, Holle, Lebenswert, Kendamil, Kabrita, Nannycare, Aptamil). Columns = stages, organic certification, prebiotic, probiotic, starch, maltodextrin, palm oil, coconut oil, DHA/ARA source, price per 100 g. Each row links to its PLP or PDP.
- Add these H2s: "How to read this chart", "Best for sensitive tummies", "Best palm-oil-free", "Which stage do I need?", "FAQ".
- Add a visible "Last reviewed: <date>, data from manufacturer labels" line and an author or reviewer.
- Keep the images only as a downloadable PDF.

Verify: the rendered page shows 1 or more `<table>` and more than 1,000 words in `<main>`, and parse_html.py lists the H2s.

### F2. Head-term mismatch: "european baby formula" returns editorial guides, the homepage is a shop front, and the commercial variant surfaces a different formuland URL
Severity: High
Evidence: The top 10 for the unqualified head term were 10/10 editorial or Blog, with 0 retailer pages (table row 1). The homepage is a Landing/Product listing page: 1,159 main words, H2s mostly product grids ("Our Organic Baby Formula Collections" followed by about 40 product cards). Its informational content is one short section, "Why Choose European Baby Formula…", and a 4-question FAQ. For "buy european baby formula online USA" the URL that surfaced was /collections/organic-baby-formula (title "Buy European Organic Baby Formula in the US | Formuland"). The homepage title also targets "European Baby Formula…", so the two pages overlap. Formuland has a 2,818-word guide at /blogs/parenting-tips/european-formula-vs-american-formula, but homepage main content links to **0** /blogs/ or /pages/ URLs (parsed from raw/pages/home.html).
Recommendation:
- Treat /collections/organic-baby-formula as the commercial "buy" target and keep the homepage as the brand/hub page.
- Build or upgrade one pillar guide, "European Baby Formula: A US Parent's Guide (Brands, Stages, Legality, How to Switch)", from the existing vs-American article. Link it from a homepage module placed directly under the hero: "New to European formula? → Read the guide | Compare brands | Take the 60-second Formula Quiz (/pages/formuland-quiz)".

Verify: home.html main content contains links to the guide, the chart and the quiz. Re-sample the head term in 8–12 weeks.

### F3. The homepage H1, value proposition and only hero CTA are invisible for about 24.5 seconds
Severity: High
Evidence: `.fhv2__overlay` (H1 "Trusted Organic European Formula for a Healthy Start in Life", the description, and the CTA "Shop Organic European Baby Formula") has CSS `opacity: 0`. It gets `is-visible` only on the hero `<video>`'s `ended` event, and the code has no timeout fallback. Measured in headless Chromium (tmp-sxo/hero_reveal.py):
- Video duration 20.223 s.
- Overlay revealed at 24.7 s (desktop 1366x900) and 24.5 s (mobile 390x844). Opacity was 0 at first sample, about 2 s after DOMContentLoaded.

screenshots/home-desktop.png and home-mobile.png show no H1 or CTA. The page includes only one `<video>`; the script references a mobile video ID that does not exist. If autoplay is blocked, `ended` never fires (not measured on real iOS).
Recommendation: Render the overlay visible by default (`opacity:1`) and keep the video as a background. If the fade must stay, add `setTimeout(reveal, 1500)` plus a reveal under `@media (prefers-reduced-motion: reduce)` and on the video `error`/`stalled` events.
Verify: in Chromium, `getComputedStyle(document.querySelector('.fhv2__overlay')).opacity === '1'` within 2 s of load on both viewports.

### F4. The safety and legality intent has no destination page, and the cautious-parent persona is the weakest (average 47/100)
Severity: High
Evidence: The sample for "is european baby formula legal and safe in the US FDA" returned childrenscolorado.org, formulaatlas.com ("Is European Baby Formula FDA-Approved? The Honest Answer"), euromallusa.com ("Is European Baby Formula Legal in the USA? 2026 Guide"), organiclifestart guides and a critical Substack post. Formuland was absent. The SERP summaries flagged AAP and Children's Hospital Colorado concerns, gray-market and counterfeit risk, and the UCSF piece "how to steer parents to safe choices". None of the 500 crawled formuland URLs has "legal", "FDA" or "import" in its title or URL (raw/crawl.jsonl). The only FDA answer is the HiPP collection FAQ. The homepage FAQ covers shipping, lost orders and English instructions, but not legality.
Recommendation: Publish /pages/is-european-baby-formula-legal-in-the-us. Cover:
- FDA status (enforcement discretion vs not-listed, written in the same honest tone as the HiPP FAQ)
- the personal-use rule (already stated in the PDP disclaimer)
- how formuland sources stock ("manufacturers or official wholesale distributors", from the homepage)
- batch and expiry checks, counterfeit red flags, and what to tell your pediatrician
- named reviewer credentials and a "last reviewed" date

Link it from the homepage FAQ, every PDP trust row ("Authentic Supply → How we source"), and the HiPP and goat FAQs. For E-E-A-T depth, run `/seo content`.
Verify: the page exists and is linked from home, the PDP template and collection templates. Re-sample the legality query.

### F5. Duties, shipping, origin and discount messages contradict each other across pages, sometimes on the same page
Severity: High
Evidence:
- Duties, both on /products/hipp-formula-stage-1: "All European import duties are covered by us — the price you see is the price you pay" vs Legal Disclaimer §4: "you are responsible for complying with all laws applicable to you, and for any duties, taxes, or charges that may apply to your order".
- Free shipping: promo bar "Fast and Free Shipping on 4/6/8/12 Packs" (screenshots), PDP rendered "A single can ships for $24.99. Add one more and your whole order ships free" (hipp-formula-stage-1-desktop.png), and HiPP and goat collections "Free shipping is available on orders over $100". Two HiPP UK cans cost $99.98.
- Origin: HiPP collection "Every tin ships from Europe" / "Orders ship from Europe" vs homepage "Sourced in the EU, Dispatched from US Hubs" vs PDP "Reliable U.S. Dispatch — We stock inventory directly inside temperature-controlled hubs".
- Delivery time: collections "most orders arrive in 2 to 4 business days" vs PDP "Insured · 2–7 day delivery".
- Subscription: "Subscribe & save 7%" next to "$44.63/can … Save 11%" (desktop screenshot).

Recommendation:
- Choose one shipping-origin statement and one free-shipping rule, keep them as shared snippet or metafield values, and render them in the promo bar, PDP, PLP and FAQ.
- Rewrite disclaimer §4 to "…for complying with laws applicable to you. Import duties on orders shipped to US addresses are included in the price."
- Show the subscription discount against a single basis (for example "Save 11% vs single can").

Verify: grep the rendered home, PLP and PDP text for "ships from", "free shipping" and "duties". Each should have one consistent phrasing.

### F6. "hipp stage 1" is a multi-version query, but the target is a single UK-recipe PDP
Severity: Medium
Evidence: In the commercial sample, PLPs (5/10) outnumbered PDPs (4/10). Formuland surfaced 4 times, never with /products/hipp-formula-stage-1. One of those was /collections/hipp-formula-stage-1 (crawl: H1 "HiPP Stage 1 Infant Formula – Organic European Milk for Newborns (0–6 Months)", **500 words**, Organization/WebSite schema only). The PDP slug and title ("Order HiPP Formula Stage 1") claim the generic term for the UK recipe only. Its saved main content links to one collection (breadcrumb /collections/stage-1-formula) and to no other HiPP Stage 1 versions. JS-loaded recommendations were not measured. screenshots.json puts the mobile PDP H1 at 860 px, below the 844 px fold. The 4-line breadcrumb and gallery come first.
Recommendation:
- Make /collections/hipp-formula-stage-1 the "hipp stage 1" target. Expand it to about 1,200 words with a Stage 1 table (German vs Dutch vs UK vs HA vs Goat: carbohydrate, probiotic, starch, size, price) and an FAQ ("Pre vs Stage 1?"). That FAQ answers the organiclifestart guide that appeared in the sample.
- On the PDP add a server-rendered "Other HiPP Stage 1 versions" switcher (German / Dutch / HA / Goat) above the description.
- On mobile, truncate the breadcrumb to a single line.

Thin content: run `/seo page`.
Verify: the PDP HTML contains links to the 4 sibling Stage 1 PDPs, and the mobile H1 is at or above 700 px.

### F7. The "goat milk formula" head term wants best-of comparisons, but the PLP has no brand comparison
Severity: Medium
Evidence: Head-term top 10 was 8/10 editorial or research. Best-of formats ranked: pickyeaterblog "The Best Goat Milk Formula (2024 Guide)", mommyhood101 "Best Goat Milk Formulas of 2026, Tested & Reviewed", gimmethegoodstuff "4 Goat Milk Formulas Reviewed". /collections/goat-milk-formula has 1,751 main words, brand H3s (HiPP, Holle, Kendamil) and a goat vs cow table, but no table comparing goat brands.
Recommendation: Add an H2 "Goat Milk Formula Brands Compared" with an HTML table: brand/line, stages, whole milk vs skim, A2, organic, DHA source, palm oil, price per 100 g. Each row links to its PDP. Add a "Best for…" line per brand.
Verify: the rendered page contains a second `<table>` with 1 or more rows per goat brand stocked.

### F8. The mobile homepage first view is covered by a loyalty pop-up
Severity: Medium
Evidence: screenshots/home-mobile.png and tmp-sxo/home-mobile-after-wait.png show a card ("Formuland Shop · Earn 2pts + Win $200! · JOIN") over the logo and header. Combined with F3, the first mobile viewport shows no value proposition and no product.
Recommendation: Delay the loyalty prompt until the second page view or until 50% scroll, or turn it into a non-overlapping inline banner. Check it against Google's intrusive-interstitial guidance.
Verify: a fresh mobile session screenshot shows the logo and H1 without the overlay.

### F9. The hero CTA points to a temporary collection URL that redirects
Severity: Low
Evidence: Homepage hero button href="/collections/organic-baby-formula-temp". This is the only reference across all 500 crawled URLs. In the browser the navigation ended at /collections/organic-baby-formula (canonical, 72 product links). The hop's status code was not captured because curl returned a 403 challenge.
Recommendation: Change the href to /collections/organic-baby-formula.
Verify: grep home.html for "-temp" returns 0.

### F10. Schema gaps on SXO-relevant templates
Severity: Low
Evidence: The PDP Offer has price, availability and priceValidUntil but no `shippingDetails`, no `hasMerchantReturnPolicy` and no `gtin` (tmp-sxo/parsed_products_hipp_formula_stage_1.json). Node `@id`s are relative ("/products/…#product"). The chart page has no ItemList or table-related markup. The homepage has only Organization and WebSite schema.
Recommendation: Add OfferShippingDetails (US, matching the single free-shipping rule from F5), MerchantReturnPolicy (matching the "Baby Satisfaction Guarantee" store-credit terms), GTIN, and absolute `@id`s. Use `/seo schema` to generate them.
Verify: schema_ecommerce_validate.py reports no missing recommended Offer fields.

## User stories (each cites a SERP signal from the sample)

**Q1 "european baby formula" (homepage)**
1. As a **parent researching a switch from US formula**, I want to understand how European formula differs from American formula, because I've read it's "cleaner", but I'm blocked by an **information gap**: the homepage hides its H1 for 24 s and links to no guide. *(Signal: head-term top 10 was all editorial, including "Is European Baby Formula Really Better?" (yahoo) and "european vs us baby formula" (sciencefriday).)* Stage: Awareness.
2. As a **cautious parent**, I want to know buying it is legal and safe, because pediatric sources warn about importing, but I'm blocked by a **trust gap**: no legality page. *(Signal: childrenscolorado.org at #1; the summary noted "talk to your pediatrician" and past recalls.)* Stage: Consideration.
3. As a **ready buyer**, I want a trustworthy US-delivering shop, but I'm blocked by **comparison fatigue** across near-identical importers. *(Signal: the "buy … online USA" variant returned 6/9 lookalike retailer home pages and PLPs.)* Stage: Decision.

**Q2 "hipp formula" (HiPP PLP)**
1. As a **US parent**, I want to know where I can legally buy HiPP and why stores don't stock it, but I'm blocked by **conflicting claims**. *(Signal: 5/9 results were "Where to buy HiPP in the USA" / "Why HiPP is not sold in the USA" guides.)* Stage: Consideration.
2. As a **first-time HiPP buyer**, I want to choose between German, Dutch and UK, but I'm blocked by **technical confusion**. *(Signal: the SERP summary listed Combiotic, Comfort, HA and AR lines, and one result incorrectly called Combiotic hypoallergenic.)* Stage: Consideration. The PLP already serves this well.

**Q3 "hipp stage 1" (UK PDP)**
1. As a **newborn parent**, I want to know whether to start on Pre or Stage 1, but I'm blocked by an **information gap** on the PDP. *(Signal: the organiclifestart guide "HiPP Pre vs HiPP Stage 1" surfaced for the bare query.)* Stage: Consideration.
2. As a **buyer who wants "HiPP Stage 1"**, I want to see every Stage 1 version and its price in one place, but I land on a UK-only PDP with no sibling links. *(Signal: the commercial sample returned 5/10 Stage-1 PLPs, including organicsbestshop /collections/hipp-baby-formula-stage-1.)* Stage: Decision.

**Q4 "goat milk formula" (goat PLP)**
1. As a **parent of a gassy or fussy baby**, I want to know whether goat formula is safe and actually gentler, but I'm **skeptical of marketing claims**. *(Signal: BJM and apollopharmacy editorial pieces; the summary said differences are "more modest than marketing sometimes implies".)* Stage: Awareness. The PLP's CMPA note helps here.
2. As an **evaluating parent**, I want the best goat formula brands ranked side by side, but I'm blocked by **comparison fatigue**. *(Signal: "Best Goat Milk Formula (2024 Guide)", "Best Goat Milk Formulas of 2026, Tested & Reviewed".)* Stage: Consideration.

**Q5 "european baby formula comparison" (chart page)**
1. As a **data-driven parent**, I want an ingredient-by-ingredient table (palm oil, starch, probiotics), but I'm blocked by three unreadable images. *(Signal: competitor chart pages with 5–8 HTML tables surfaced for "comparison chart".)* Stage: Consideration.
2. As a **parent weighing a US "European-style" brand**, I want to see how European imports compare with Bobbie or Kendamil US, but I'm blocked by missing US-vs-EU context. *(Signal: hibobbie.com blog, us.kendamil.com blog and thequalityedit "How does European-inspired Bobbie compare…".)* Stage: Consideration.

## Persona scores (3 personas, per task instruction)

Persona evidence:
- **P1, New parent switching from US formula** (Consideration): EU-vs-US editorial in Q1 and Q5, and the "HiPP Pre vs Stage 1" guide.
- **P2, Budget-conscious repeat buyer** (Decision/retention): summaries quoting "typically more expensive" and "$30 to $50 per can", competitors' "Spend $150 to get free shipping" and bulk discounts, and "Aptamil UK… best budget EU import".
- **P3, Cautious parent worried about safety/legality** (Awareness/Consideration): the legality sample (childrenscolorado, formulaatlas, euromallusa, substack) and the UCSF "steer parents to safe choices" piece.

Cells show Relevance/Clarity/Trust/Action (each out of 25) = total out of 100.

| Target page | P1 Switcher | P2 Budget repeat | P3 Cautious |
|---|---|---|---|
| / (home) | 14/8/16/13 = **51** | 15/12/15/15 = **57** | 9/7/13/8 = **37** |
| /collections/hipp-formula | 21/18/19/18 = **76** | 15/14/17/15 = **61** | 20/13/16/14 = **63** |
| /products/hipp-formula-stage-1 | 18/15/20/15 = **68** | 22/14/16/22 = **74** | 15/12/12/12 = **51** |
| /collections/goat-milk-formula | 19/17/18/17 = **71** | 13/13/17/15 = **58** | 20/16/17/14 = **67** |
| /pages/european-baby-formula-comparison-chart | 14/6/7/3 = **30** | 6/6/7/3 = **22** | 6/5/5/3 = **19** |
| Average | 59 | 54 | **47 (weakest)** |

Score evidence:
- **Home.** H1 and CTA hidden for 24.5 s (Clarity). The quiz appears only in the header. No legality answer. The "EU vs US Hubs" origin statement conflicts with the HiPP page.
- **HiPP PLP.** Version table, FDA FAQ and filters above the fold. No subscription or bundle mention for P2. The FAQ sits deep in a 3,220-word page.
- **PDP.** Bundles, subscription and sticky add-to-cart support P2. Duties contradiction and three free-shipping rules (Trust/Clarity). Disclaimer hidden in a tab. Mobile H1 below the fold.
- **Goat PLP.** CMPA note and goat vs cow table. No price or brand comparison.
- **Chart page.** Images only, no source or date, 0 product links.

Weakest persona, P3 (Cautious parent, 47). Top issue: no legality/FDA page, plus contradictory duties and origin claims. Fix: F4, then F5.
Systemic weak dimension: **Action** for P3 (8–14). No page offers a "How we source / is it legal" next step.
Priority actions:
1. F4 legality page linked from PDP, PLP and home (P3).
2. F5 single shipping, duties and origin rule (P3 and P2).
3. F1 HTML chart (all personas, lowest page).
4. F3 hero fix (P1).
5. F6 Stage 1 hub and sibling switcher (P1 and P2).

## Not checked / limitations

- **The SERP data is not Google.** WebSearch tool, single sample on 2026-11-02, US-only tool, engine and location unknown. Order is the order returned, not a verified rank. No PAA, Shopping, ads, AI Overview, snippets or related searches. The unqualified "hipp formula" and "hipp stage 1" samples were dominated by spam or unrelated (hip-joint) results, so commercial variants served as proxies. Search volumes, GSC impressions and CTR were not available.
- **Rendering.** render_page.py failed by design (homepage-render.json: "url_safety: DNS resolution failed"). Analysis used saved HTML in raw/pages, the plugin's offline parse_html.py and headless Chromium through the proxy. Formuland began returning 403 challenges and fetching stopped. curl to competitor sites returned Shopify "Verifying your connection" 403s, so competitor pages were rendered once each in Chromium.
- **Not measured:**
  - /collections/hipp-formula-stage-1 and /collections/organic-baby-formula content (crawl row only)
  - JS-loaded PDP recommendations
  - real iOS autoplay and Low Power Mode behaviour of the hero
  - prefers-reduced-motion (the probe returned null)
  - chart legibility on mobile
  - cart and checkout totals, and the duties actually charged at delivery
  - the Formula Quiz flow
  - blog articles that returned 429 in the crawl (e.g. hipp-baby-formula-german-vs-dutch-vs-uk, hipp-vs-kendamil)
- **Method notes.** Three personas per the task brief; the plugin method recommends 4–7. No wireframes (not requested). Trustpilot counts differ by page snapshot (4,180 vs 4,298), probably due to widget timing.
- **Follow-ups.** `/seo schema` (F10), `/seo content` (F4 E-E-A-T), `/seo page` (/collections/hipp-formula-stage-1, 500 words). For a PDF report, use `/seo google report`.
