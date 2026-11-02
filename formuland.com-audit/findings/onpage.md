Score: 56/100

On-Page SEO (titles, meta descriptions, headings, internal linking, anchor text).

Data: raw/crawl.jsonl, 468 pages with HTTP 200 out of 500 sitemap URLs. These are 249 products, 111 articles, 74 collections, 32 pages and the home page. The other 32 returned 429. Figures below are recomputed on the 468 and differ slightly from the task's numbers (203 vs 196 titles >60, 338 vs 337 descriptions >160).

## What works

- **Titles and descriptions are unique.** 0 duplicate titles. Only 1 duplicate meta description (/pages/avada-sitemap = /pages/avada-sitemap-products).
- **Collection titles are concise and keyword-led.** Median 42 characters, only 1 over 60. Examples: "HiPP Formula: Dutch, German, UK, HA & Goat | Formuland" and "Organic Goat Milk Baby Formula | Formuland".
- **No bulk metadata templating.** `metadata_template.py --pairs-file` on 467 pages returned site_risk "low", templated_ratio 0.0, templated_count 0, shared_cta_phrases {} and no site_flags.
- **Every crawled article has an H2.** 0 of 111 have none. Articles have Article schema, and 48 collections have FAQPage schema.
- **Most images have alt attributes.** img_missing_alt_total = 0, though 14,206 of 22,743 images have empty alt, mostly decorative.

## Findings

### 1. Meta descriptions are auto-excerpts that run long and break mid-sentence
- Severity: High
- Evidence:
  - 338 of 468 descriptions exceed 160 characters:
    - products: 226 of 249 (median 291 characters)
    - articles: 92 of 111 (median 320)
    - pages: 18 of 32
  - 146 of the long ones do not end in punctuation, so they are cut mid-sentence.
  - The excerpts carry body-copy errors into the SERP:
    - /blogs/parenting-tips/what-to-feed-your-infant: "Questions necessarily arises..."
    - /blogs/parenting-tips/taking-care-of-a-coughing-baby: "since the But as a parent ... since the"
  - The template check flags 62 products where the description echoes the title (description_echoes_title), e.g. /products/holle-organic-milk-cereal-with-bananas-from-6-months-250g: "...from 6 months (250g)99 percent...".
  - 104 descriptions carry a brand suffix (brand_suffix_in_description).
  - Collections follow a soft formula: 72 of 74 open with a verb (Buy/Shop/Explore), 55 contain "at Formuland", and 26 use shipping-time or "reliably" phrasing.
- Recommendation: Write a 140–155 character description in Shopify's SEO field for each page. Start with the top 50 revenue products and collections and all YMYL articles. Lead with the differentiator (stage, age, protein source, origin) rather than the product name. Verify by re-crawling: the count over 160 characters should drop below 50, and no description should end without punctuation.

### 2. Key trust and YMYL pages have no meta description and no H1
- Severity: High
- Evidence:
  - Missing meta description (13): /pages/about-us, /pages/contact, /pages/nara-organics-formula-recall, /pages/compare, /pages/collab, /pages/byob, /pages/froonze-new-customer-account, /pages/hipp-byob-bundle, /pages/customer-portal, /pages/formuland-quiz, /pages/quiz-results, /pages/choose-your-free-gift, /products/shipping-protection-formula.
  - Missing H1 (6): /pages/contact, /pages/formuland-faqs, /pages/customer-portal, /pages/about-us, /pages/nara-organics-formula-recall, /pages/choose-your-free-gift.
  - The recall page (1,657 words) is the site's most news-driven page and has neither.
- Recommendation:
  - Add an H1 to the page templates used by about, FAQ, recall and contact, e.g. "Nara Organics Formula Recall (June 2026): Lots, Symptoms, Refunds".
  - Add descriptions to those four pages.
  - Noindex the utility pages (customer-portal, froonze, quiz-results, choose-your-free-gift, shipping-protection).
  - Verify with `parse_html.py` on each URL: h1 count = 1 and meta_description is present.

### 3. Product template outputs a second H1
- Severity: Medium
- Evidence: 45 pages have more than one H1. 44 are products, where the second H1 is "Formuland — Product & Purchase Disclaimer" (e.g. /products/lebenswert-stage-1, /products/holle-stage-2-organic-baby-formula). /pages/our-company has a second H1, "Contact Us".
- Recommendation: Change the disclaimer heading in the product-template snippet to `<h2>` or `<p class="h4">`, and do the same for the "Contact Us" section. Verify that the h1 list in a re-crawl has length 1 on all products.

### 4. Titles are long and inconsistent, with keyword stuffing on the home page
- Severity: Medium
- Evidence:
  - 203 of 468 titles exceed 60 characters: products 130 of 249 (median 61), articles 69 of 111 (median 64).
  - The main cause is the Shopify default suffix "\n– Formuland", used on 297 titles. The site mixes three separators: "–" ×297, "|" ×147, "-" ×17.
  - The home page title is 106 characters: "European Baby Formula | Shop Organic European Formula Brands & Baby Formula From Europe Online - Formuland".
  - 11 products use the doubled pattern, e.g. "HiPP UK Stage 1 Organic Infant Formula (800g) | Order HiPP Formula Stage 1 - Formuland". The H1 on that page ("HiPP UK Organic Stage 1 from Birth (800g)") does not match it.
  - Titles truncated in source:
    - ultimate-guide-to-finding-the-best-baby-formula-10-ingredients-to-avoid: "...10 Ingredients to Avo – Formuland"
    - best-hypoallergenic-baby-formula: "The Best Hypoallergenic Formula | Find the Best Hypoallergenic Baby Fo..."
  - Product titles state age inconsistently with their URLs: hipp-organic-100-rice-cereal-from-4-months-200g has title "from 5 months".
- Recommendation:
  - Standardise on "Primary keyword + qualifier | Formuland", at most 60 characters.
  - For products, use "Brand Stage N Formula (size) – Origin | Formuland".
  - Rewrite the home title, e.g. "European Baby Formula – HiPP, Holle, Kendamil | Formuland".
  - Make the H1 and title name the same product and age.
  - Verify by re-crawling: under 40 titles over 60 characters, and 0 titles with " | Order ".

### 5. Weak internal linking to commercial collections, one about page and older articles
- Severity: Medium
- Evidence:
  - Among the 468 crawled 200 pages, 67 have one or fewer inbound links from other crawled pages, and 41 have none. crawl-summary reports 88 across all 500 sitemap URLs, including the 429s.
  - Zero-inbound pages include:
    - /collections/hipp-formula-stage-1, -2, -3 and -4
    - /collections/kendamil-organic-formula
    - /collections/ready-to-feed, /collections/baby-food-jars, /collections/baby-food-pouches, /collections/formuland-bundles
    - /pages/our-company
    - 6 premibio and 3 bebe-m products
    - /blogs/parenting-tips/unpacking-the-issue-dented-formula-cans-creased-boxes-explained. This explainer goes unlinked even though every product page has a "minor dents" note.
  - /blogs/parenting-tips/what-to-feed-your-infant has 0 contextual body links. Article inbound links: median 2, with 9 at one or fewer.
- Recommendation:
  - Link the HiPP stage collections from /collections/hipp-formula and from the stage filters in the main navigation.
  - Decide whether /collections/kendamil-organic-formula duplicates /collections/kendamil. If it does, 301 it; if not, link it.
  - Link the dented-cans article from the product "Please Note" block.
  - Add 3–5 contextual links per article to the matching collections and products.
  - Verify by re-running the inbound count: under 20 pages at one or fewer links, and 0 core collections with none.

### 6. Anchor-text and link-target errors
- Severity: Medium
- Evidence:
  - In /blogs/parenting-tips/best-organic-baby-formula-which-is-safest-for-your-child, the anchor for "Jovie organic baby formula" is split into two links, "Jovie organic baby formu" and "la". Both point to /collections/holle-goat-milk-formula, a different brand.
  - The footer anchor "Our Company" points to /pages/about-us, which leaves /pages/our-company orphaned.
- Recommendation:
  - Merge the split anchor and point it at the Jovie collection or product (or remove the link).
  - 301 /pages/our-company → /pages/about-us, or point the footer at it.
  - Verify by extracting anchors from the article body: no anchor shorter than 3 characters, and brand anchors resolve to the same brand.

### 7. Thin utility pages are indexable and listed in the sitemap
- Severity: Low
- Evidence:
  - 31 crawled pages have under 300 words:
    - /pages/compare (2), /pages/froonze-new-customer-account (4), /pages/european-baby-formula-comparison-chart (5)
    - /pages/avada-sitemap-blogs (6), /pages/collab (13), /pages/byob (15), /pages/hipp-byob-bundle (22)
    - /pages/formuland-reviews (50), /pages/contact (57)
    - /blogs/parenting-tips/now-shipping-to-canada (85), /blogs/parenting-tips/our-site-has-a-new-look (266)
    - /collections/anti-reflux-formula (121)
  - The crawl-summary noindex list is empty, so none of these carry noindex.
- Recommendation:
  - Noindex and drop from the sitemap: compare, froonze, collab, byob, quiz-results, the avada-sitemap-* duplicates and the two announcement posts.
  - Expand the comparison chart (as an HTML table), reviews and anti-reflux pages, which have search demand.
  - Verify that the sitemap excludes the noindexed URLs and that crawl thin pages drop below 10.

### 8. Generic or malformed headings
- Severity: Low
- Evidence:
  - Product H2s are generic ("Description", "Ingredients", "Legal Disclaimer").
  - The home page has the H2 "Formuland".
  - The /collections/hypoallergenic-formula H1 reads "Hypoallergenic Formula for Babies : FROM EUROPE'S TRUSTED BRANDS" (all caps, stray space).
  - The about page section headings sit at H2/H3 with no H1.
- Recommendation: Use descriptive H2s, e.g. "HiPP UK Stage 1 ingredients" or "How to prepare HiPP UK Stage 1". Fix the hypoallergenic H1 to sentence case. Verify by heading extraction.

## Not checked / limitations

- **Pages missing from the crawl data.** 31 articles and the blog index returned 429 in the crawl. My additional fetches returned 403 Cloudflare challenges, and a browser-based retry was denied as bot-check circumvention and not repeated. On-page data for those 32 URLs is missing.
- **Inbound counts are a lower-bound sample.** Only about 250 of the products were crawled, and counts include navigation and footer links.
- **Not checked:** SERP rendering of titles and descriptions, search-console query data, and the hreflang locale variants (/en-intl/, /ja-intl/, /zh-intl/).
