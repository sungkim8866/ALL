Score: 41/100

Content Quality (E-E-A-T, YMYL, thin/duplicate content, readability, AI-citation readiness). Site: https://formuland.com (YMYL: infant formula / infant health).

E-E-A-T breakdown (this skill's internal weighting, not Google's): Experience 40/100 (20%), Expertise 25/100 (25%), Authoritativeness 45/100 (25%), Trustworthiness 45/100 (30%), weighted 39. AI-citation readiness: 40/100.

## What works

- **The Nara Organics recall page is properly sourced.** `raw/pages/pages_nara_organics_formula_recall.html` links to the FDA recall notice, a CDC release, CIDRAP, infantbotulism.org (BabyBIG) and Cleveland Clinic. It includes UPC codes (860013251901, 860013251918), a lot-code grid, the distribution window, a symptom list and handling steps. These are the strongest trust signals on the site.
- **The HiPP collection FAQ explains regulation accurately.** /collections/hipp-formula says "The FDA doesn't formally 'approve' any infant formula ... HiPP's European formulas are not on the FDA's list of infant formulas marketed in the U.S." and cites Regulation (EU) 2016/127. This is a candid statement of import status.
- **Product pages have real product data.** The HiPP UK Stage 1, Holle Stage 1 and Lebenswert Stage 1 pages carry full ingredient lists, per-100 ml nutrition tables and an iron-standard caveat ("Customers in Australia, Canada, the United States, and China may need additional supplements"). They also have a dated disclaimer ("Last updated: July 11, 2026") that tells parents to consult a pediatrician.
- **Some newer content is written for the query.** Collection pages are long (median 1,805 words). The newer articles have clean, query-led meta descriptions, e.g. /blogs/parenting-tips/melatonin-for-babies-everything-you-need-to-know ("What the AAP and sleep experts say") and /blogs/parenting-tips/can-baby-formula-go-bad-in-heat (with FAQPage schema).
- **Templated-metadata check is clean.** `metadata_template.py` on 467 title/description pairs returned site_risk "low", templated_ratio 0.0 and shared_cta_phrases {}. There is no sign of a bulk CTA metadata job (details in onpage.md).

## Findings

### 1. Flagship buying guide gives health advice that contradicts EU law and the site's own product pages
- Severity: Critical
- Evidence:
  - The page is /blogs/parenting-tips/best-organic-baby-formula-which-is-safest-for-your-child (3,042 words of body text).
  - It says DHA/ARA come with "the possibility of algae and fungus, which is unnatural ... Avoid these whenever possible." It also says organic formula means "zero chance of GMOs, no pesticides and no DHA/ARA".
  - DHA has been mandatory in EU infant and follow-on formula since 2020 (Delegated Reg. (EU) 2016/127). The site's own Holle Stage 1 page says "Contains DHA (required by law for infant formula)". The HiPP UK Stage 1 page lists DHA 13.2 mg and ARA 13.2 mg per 100 ml.
  - The same article says palm oil leads "to lower bone mineral density". Yet its own top picks contain palm oil: Holle Stage 1 ingredients "palm oil*³" and HiPP UK Stage 1 "organic palm oil".
  - Further unsupported or overbroad claims:
    - "Organic formula is definitely the way you want to go"
    - "Carrageenan ... is prohibited in the European union" (overbroad; check it against Reg. (EC) 1333/2008 Annex II)
    - "As formula is typically made with cow's milk, it has less lactose"
  - The only external citation in the article is a Statista market-size link.
- Recommendation: Rewrite the article with a named, credentialed reviewer (registered dietitian or pediatrician). Remove the DHA/ARA and palm-oil advice, or restate it in line with EU 2016/127. Cite primary sources (EFSA, AAP, FDA) next to each health claim. Verify by searching the page for "Avoid these whenever possible" (should return 0 hits) and checking that every nutrient claim has a linked source.

### 2. No identifiable author, credentials or medical review on YMYL articles
- Severity: High
- Evidence:
  - Both saved articles show only "Posted by Formuland on ...". Their Article JSON-LD has `"author": {"@type": "Person", "name": "Formuland"}`, a brand typed as a Person, with `mainEntityOfPage.@id` set to "https://formuland.com" (the homepage).
  - /pages/about-us claims "We work with pediatric and nutritional experts", but no expert is named anywhere in the 18 saved pages.
  - Crawl titles show medical topics published under this same byline-less setup:
    - how-to-master-safe-co-sleeping-with-your-infant-child
    - melatonin-for-babies-everything-you-need-to-know
    - taking-care-of-a-coughing-baby
    - why-is-your-baby-vomiting-and-how-to-provide-relief
    - should-you-opt-for-soy-free-formula-for-your-baby-with-g6pdd
    - could-goat-milk-formula-help-baby-eczema
    - foods-to-avoid-during-pregnancy
- Recommendation:
  - Add author and "Medically reviewed by [name, RD/MD/IBCLC], [date]" blocks to the article template. Link them to bio pages with credentials.
  - Emit `author` as a real Person with `sameAs`, plus `reviewedBy` (or `lastReviewed`) on the page. Fix `mainEntityOfPage` to the article URL.
  - Verify with `parse_html.py` on two articles: author.name ≠ "Formuland" and a reviewer is present.

### 3. Conflicting preparation and feeding charts on infant-formula product pages
- Severity: High
- Evidence:
  - /products/hipp-formula-stage-1 shows HiPP's own chart: "0 - 2 weeks | 2 scoops | 60 ml | 7-8 feeds", "Level the scoop with the built-in leveller", "One level scoop = 30ml".
  - The same page then shows a second, generic chart: "Week 1 | Follow your doctor's advice", "Week 2–4 | 3 | 3 | 3.4 | 6–7", "level with a knife", "Use sachet within 3 weeks of opening" (the product is a tin). It also includes Stage 2/3/4 tables on a Stage 1 page.
  - The same generic block appears on /products/holle-formula-stage-1 and /products/lebenswert-stage-1. On Lebenswert it is copied from packaging: "indicated in the table on back of this page".
- Recommendation: Remove the generic "Preparation and Feeding Chart" snippet from the product template. Show only brand- and stage-specific instructions that match the label, with a link to the manufacturer's preparation PDF. Verify that each product page contains exactly one feeding table and no "sachet" or "back of this page" text where these don't apply.

### 4. Stale, uncited and outdated advice in old articles that still rank as live content
- Severity: High
- Evidence:
  - /blogs/parenting-tips/what-to-feed-your-infant (published 2016-09-23, schema dateModified 2025-04-21, 573 words, 0 links, no sources) advises: "you should do away with foods which are most likely to cause allergies". Current US guidance recommends introducing common allergens such as peanut alongside other first foods: the NIAID 2017 addendum and the Dietary Guidelines for Americans 2020–2025.
  - Grammar defects: "Questions necessarily arises", "they are so uses from feeding only on milk".
  - The crawl shows dated topics still presented as current:
    - "What to Know About the Current Baby Formula Shortage"
    - "...What's Best in 2022" (meta: "As of April 2022 ...")
  - Uncited statistics appear in crawl excerpts: "about 80% of baby formulas on the market today have been found to contain traces of arsenic" (how-to-make-the-best-baby-formula-in-7-simple-steps) and "8.2 million children below the age of five pass away" (what-s-the-best-water-for-baby-formula-preparation).
  - five-baby-formula-alternatives-you-should-consider-for-your-child opens "there are alternatives that may be better for your child's growth".
  - 71 articles share sitemap lastmod 2026-10-21. Date signals are being refreshed in bulk, but content like the above is not.
- Recommendation: Audit all 142 articles. Update or 301 the outdated ones into current guides, and noindex or delete the "alternatives to formula" type pieces unless a clinician reviews them. Add visible "Last reviewed" dates only when content actually changes. Verify that no article mentions "current shortage" or "2022" as present tense, and that every numeric claim has a citation.

### 5. Trust pages contradict each other, and one about page is orphaned
- Severity: High
- Evidence:
  - Scale and history claims disagree across pages:
    - /pages/our-company meta: "delivered over 500,000 orders since our inception in 2014. Our main office is strategically located in Europe"
    - /pages/our-company body: "a trusted partner to thousands of families"
    - /pages/about-us: "Founded in 2014 in California ... serving over 1 million families"
    - Product pages: "fulfilled over 1 million orders"
    - Reviews meta: "serving parents and caregivers for 10 years"
  - No street address or phone number was found in the home or our-company text. The only contact is care@formuland.com.
  - The footer anchor "Our Company" points to /pages/about-us, so /pages/our-company has 0 inbound links (crawl).
  - /pages/about-us has no H1 and no meta description.
- Recommendation: Merge into one about page: 301 /pages/our-company → /pages/about-us. Publish a single consistent set of facts (legal entity, registered address, founders or leadership names, order count with an as-of date). Add a postal address and phone to the footer and to Organization schema `address`/`telephone`. Verify with `grep -c "500,000"` across the site (should be 0) and by checking that Organization JSON-LD has an address.

### 6. The comparison chart, a key commercial and informational asset, exists only as images
- Severity: Medium
- Evidence: /pages/european-baby-formula-comparison-chart has 5 words of HTML text (the H1). The chart is three images with alt text "European baby formula comparison chart, part 1/2/3". The meta description promises coverage of "prebiotic, probiotic, starch, maltodextrin". `content_quality.py` on main text: tokens 5, flag "thin-content".
- Recommendation: Rebuild the chart as an HTML `<table>`: one row per product, columns for protein source, carbohydrate, palm oil, pre/probiotics, DHA mg/100 ml, starch/maltodextrin, and a source/date. Keep the image as a download. Verify that the page has more than 300 words of text and that the table is visible in raw HTML.

### 7. Editing and duplication defects in published copy
- Severity: Medium
- Evidence:
  - In the best-organic article, the paragraph "Pros and Cons of Jovie Organic Formula ... sensitivities to goat's milk as well." appears twice verbatim (count = 2), along with "susceptible vulnerable".
  - The taking-care-of-a-coughing-baby excerpt repeats a clause: "since the But as a parent, this is something you'll likely encounter many times, since the".
  - The recall page contains "so make sure to avoid running an open marketplace" (garbled).
  - `content_quality.py` on main text: best-organic overall 73, information_density 0.187, flag low-density. what-to-feed overall 65, density 0.052, filler 25.
  - Readability (Flesch / FK grade): best-organic 38.7 / 12.8; what-to-feed 64.7 / 9.2; /collections/hypoallergenic-formula 25.3 / 16.2 (26.1 words per sentence).
- Recommendation: Have an editor proof the articles. Target FK grade 8–9 for parent-facing copy, keep sentences under 20 words, and remove duplicate blocks. Verify by re-running `content_quality.py` (target overall ≥85, no low-density flag) and checking that readability is at or below grade 9.

### 8. Product descriptions: manufacturer copy, generic AI-style bullets, machine-translation errors and age mismatches
- Severity: Medium
- Evidence:
  - The HiPP UK Stage 1 description is in the manufacturer's first person: "We've given our packaging a thoughtful update", "Our new tin is fully recyclable".
  - Holle Stage 1 uses generic bullets: "Delicious Taste and Digestibility: Designed to be delicious for infants", "Customer Support: The availability of customer support and assistance through email...". It also makes an unqualified "Gluten-Free: Suitable for infants with gluten sensitivity".
  - The /products/hipp-comfort-formula-milk-new-german-600g meta says the formula "regulates chair" (mistranslation of German "Stuhl", stool).
  - Age mismatches:
    - URL hipp-organic-100-rice-cereal-from-4-months-200g has title "from 5 months"
    - URL holle-organic-wholegran-cereal-rice-from-4-months-250g has description "from 6 months"
  - 692 words (28–37% of extracted text) are identical across the three saved product pages, mostly the 10-clause legal disclaimer.
- Recommendation: Write original, label-verified descriptions for the top formula SKUs first. State the age once, from the pack, consistently across URL, title, H1 and copy. Collapse the legal disclaimer into a linked policy page or an accordion. Verify against the pack label, and check that each product's unique text is at least 60% of main content.

### 9. Reviews page is thin; social proof is client-side only
- Severity: Medium
- Evidence: /pages/formuland-reviews has 48–50 words of server HTML. The reviews sit in a `trustpilot-widget` div that renders only in the browser. The title asks "Is Formuland A Legit Company?" but no answer content is in the HTML.
- Recommendation: Server-render a selection of recent reviews with dates and the aggregate rating, with a link to Trustpilot. Add a short "how we source and verify" section. Verify that raw HTML has at least 10 review texts.

### 10. Recall page is well-sourced but under-signalled and commercial in tone
- Severity: Low
- Evidence:
  - It has no H1, no meta description, no author and no last-updated date.
  - The "Switching From Nara Organics" section makes uncited claims ("A2-type beta-casein proteins, which digest more rapidly") and recommends products the site sells.
- Recommendation: Add an H1, a visible "Updated [date]" and a reviewer. Cite the goat-milk claims or remove them, and keep the product recommendations visually separate from the safety guidance.

### 11. agents.md is listed in an "agentic discovery" sitemap
- Severity: Info
- Evidence: raw/sitemaps/sitemap_agentic_discovery.xml lists https://formuland.com/agents.md (836 words, crawl status 200).
- Recommendation: Content was not reviewed (later fetch blocked). Make sure it repeats only verifiable facts that match the about page, and keep it out of the main XML sitemap index if it is not meant for search.

## Not checked / limitations

- **Pages I could not read.** 31 articles and the blog index returned 429 (Cloudflare "Verifying your connection...") in the original crawl. My extra curl fetches all returned 403 managed challenges: /pages/contact, /agents.md and 5 articles (co-sleeping, melatonin, formula alternatives, make-formula-in-7-steps, EU vs US formula). A headless-browser attempt was denied as bot-check circumvention and was not retried. For those articles I used only crawl titles and meta-description excerpts. The contact page content and agents.md content were not reviewed.
- **Not compared against manufacturers.** Product originality was not checked against manufacturer sites; the evidence above is internal (voice, duplication).
- **Readability is approximate.** It is a custom Flesch implementation run on trafilatura text.
- **Raw-HTML quality scores are inflated.** `content_quality.py` on raw HTML scored 95–96 only because script tokens were counted. The main-text figures above are the ones to use.
- **Not checked:** Trustpilot rating, external reputation, backlinks and real author identities (out of scope / no data).
