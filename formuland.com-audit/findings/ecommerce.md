Score: 63/100

Data source: On-page analysis (static) of saved raw HTML (`raw/pages/*.html`, `raw/crawl.jsonl`, `raw/sitemaps/*`, `raw/robots.txt`, `raw/home.html`) plus the saved desktop/mobile screenshots (rendered state). DataForSEO Merchant: not available, so no marketplace data. Sub-scores (judgement): product template 62, category pages 70, Merchant/free-listing readiness 52, content uniqueness 60, URL hygiene 68, internal linking 72.

Fetch note: after the 500-URL crawl, Cloudflare started answering every curl request with HTTP 403 and `cf-mitigated: challenge`. This hit /products.json, /collections/hipp-formula?page=2, /collections/hipp-formula?sort_by=..., ?filter.p.product_type=..., /collections/hipp-formula/products/hipp-formula-stage-1 and /policies/shipping-policy (6 requests, saved in `tmp-ecommerce/fetch/`). I did not try to get around the challenge. Everything below uses data already collected. The crawl itself logged 33 URLs with HTTP 429 and title "Verifying your connection..." (`raw/crawl-summary.json` -> `non_200`).

## What works

- Server-rendered product content. The three saved product pages contain ingredients, per-100ml nutrition tables, preparation steps and feeding charts in raw HTML (HiPP stage 1: 4,553 words; Holle: 3,998; Lebenswert: 2,812). Crawl median for products is 1,977 words (min 1,247, max 5,445).
- Reviews are visible and machine-readable. HiPP stage 1 shows 4.91 / 107 Judge.me reviews with a star breakdown (102 five-star, 2 four, 2 three, 1 one). Holle shows 4.85 / 55 and Lebenswert 4.57 / 7. The second Product JSON-LD block carries 20, 21 and 7 review objects, with merchant replies.
- The HiPP page opens with a Formuland-written summary. It includes an "Ideal for: Newborns from birth" line and explains how the UK recipe differs from the German and Dutch ones ("relies on prebiotics alone rather than a combined prebiotic and probiotic blend"). That is useful stage/age guidance.
- Collection pages work as landing pages. /collections/hipp-formula has an H1, a short intro above the grid, 15 cards per page, a crawlable `?page=2..6` pagination, `<link rel="next">`, and about 3,300 words of buying-guide content below the grid. Median collection length is 1,805 words; the crawl shows 74 of 74 collections at HTTP 200.
- Out-of-stock products stay live at HTTP 200 with a "Sold Out" state and `availability: OutOfStock` on all four offers (Lebenswert stage 1). That is the correct way to keep rankings.
- The robots.txt rules are sensible for a Shopify store: `/collections/*sort_by*`, `/collections/*+*` and `/collections/*filter*&*filter*` are disallowed, and cart, checkout and account are blocked.
- Price is consistent between `og:price:amount`, the Offer price and the variant JSON for all three pages (49.99, 27.49, 20.99 USD). The HiPP gallery serves 600/800/1200 px srcset images and the og:image is 1200x1200.
- Bundle pricing is clear: the HiPP 1-can price is $49.99 and the 12-can price is $45.99 per can. Holle 12 cans is $25.49 per can against $27.49 for one.

## Findings

### 1. Shipping cost, delivery window and origin are stated inconsistently across templates
Severity: High
Evidence:
- Rendered HiPP PDP (`screenshots/hipp-formula-stage-1-desktop.png`): "One-time purchase $49.99/can + $24.99 shipping", with "A single can ships for $24.99. Add one more and your whole order ships free".
- The same page in raw HTML says "Free shipping included · buy once, no commitment." (`fpe__freenote`, hidden until JS runs). The announcement bar says "Fast and Free Shipping on 4/6/8/12 Packs".
- /collections/hipp-formula copy says "Free shipping is available on orders over $100" and "most orders arrive in 2 to 4 business days". Its meta description says the same. /collections/hypoallergenic-formula says "2 to 7 business days" and "$100 or more ... ship free". The product strip says "Insured · 2–7 day delivery". The legacy Lebenswert popup says "Most orders are delivered within 1-6 business days".
- Origin: collection copy says "Every tin ships from Europe". The product trust block says "We stock inventory directly inside temperature-controlled hubs" and "Reliable U.S. Dispatch". The legacy popup says "Shipments out of California Warehouse" and uses an example date of "9/2/2017".
Impact: Merchant Center and free listings compare on-page shipping with the feed, and a mismatch triggers disapprovals. Shoppers also see a 50% shipping surcharge on a single can ($74.98 landed) that the structured data and some copy do not disclose.
Recommendation: Pick one shipping policy (threshold, single-can fee, delivery range, origin) and publish it from one source, for example a metafield or snippet used by the PDP strip, collection intros, meta descriptions and the announcement bar. Remove the 2017 example and the California/Europe contradiction. Make the Merchant Center shipping settings match. Verify by searching the rendered HTML of 5 PDPs and 5 collections for "business days", "free shipping" and "ships from" and confirming one value each.

### 2. Product structured data lacks the shipping, returns and identifier fields free listings use
Severity: High (structured-data validity itself belongs to the schema agent)
Evidence: The theme Product block (e.g. /products/hipp-formula-stage-1) has `sku`, `brand`, `category` and an `offers` array with price, `priceCurrency`, `availability` and a per-variant `url`. It has no `shippingDetails`, no `hasMerchantReturnPolicy`, no `gtin` or `mpn`, no `itemCondition`, and one `image`. Lebenswert's image is `..._800x.jpg`. `priceValidUntil` is "2026-11-03", the day after the audit date, so it appears to be a rolling date. The returns text on the page is only the "Baby Satisfaction Guarantee": one opened item per order, store credit, "Contact Customer Care within 30 days". The refund and shipping policies are linked only from the footer (`/policies/refund-policy`, `/policies/shipping-policy`). I could not fetch them.
Recommendation: Add `OfferShippingDetails` (rate, destination US, handling and transit time) and `MerchantReturnPolicy` (return window, store-credit refund type) to each Offer. Add GTIN (barcode) per variant where available and `itemCondition: NewCondition`. Replace the rolling `priceValidUntil` with a real date or remove it. State the store-credit-only return rule on the PDP itself. Verify with the Rich Results Test and Merchant Center diagnostics.

### 3. Feeding instructions on the HiPP page conflict with each other (safety-relevant, YMYL)
Severity: High
Evidence: /products/hipp-formula-stage-1 has two tables. HiPP's own: "0 - 2 weeks ... 2 scoops, 60 ml", "One level scoop = 30ml". A generic "Preparation and Feeding Chart" then follows: "Week 2–4 ... 3 scoops ... 3 fl oz", "400g makes 100 fl oz of formula ... 800g makes 200 fl oz", and Stage 1 to Stage 4 tables for every brand. The generic chart and the line "Use sachet within 3 weeks of opening" also appear on the Lebenswert page (a box) and the HiPP page (a tin).
Recommendation: Show only the manufacturer's chart for each SKU and delete the generic multi-stage chart from product templates. Have a qualified reviewer check each brand's chart. Verify by confirming each PDP has one feeding table and that it matches the pack label.

### 4. Generic, partly unsupported product copy on Holle and Lebenswert; large boilerplate on every product
Severity: High (content uniqueness and YMYL)
Evidence:
- Holle stage 1 description: "Customer Support: The availability of customer support and assistance through email ensures parents can get additional information about the product", "Gluten-Free: Suitable for infants with gluten sensitivity". Lebenswert stage 1: "Convenient Ordering: Place an order to ensure your baby has access to this healthy formula" and "Deficiency in essential fatty acids linked to learning difficulties and dyslexia." That reads as templated filler, and the dyslexia line has no source on the page.
- The HiPP description contains HiPP UK pack-marketing text ("Same Nourishing Formula, Smarter Tin Design", "¹As required by law."). It reads as manufacturer copy, but I did not compare it to HiPP's site, so that is not verified. The Product JSON-LD description is exactly 5,000 characters on HiPP, so it looks truncated.
- Every PDP embeds the same 10-section "Legal Disclaimer". 44 of 249 crawled products carry a second H1, "Formuland — Product & Purchase Disclaimer" (`h1_multiple` in the crawl, e.g. /products/holle-stage-2-organic-baby-formula).
Recommendation: Rewrite Holle and Lebenswert descriptions around age/stage, what differs from the sibling stages, ingredient base (milk type, organic, starch, palm oil), allergens and who it suits. Remove the dyslexia and "gluten sensitivity" claims unless a citation is added. Load the legal disclaimer once (a modal or the `/policies/` page) and keep it out of the H1/H2 outline. Verify with a crawl that no product has more than one H1 and no duplicated 10-section block.

### 5. Price, bundle tiles and delivery date are filled in by JavaScript on the new PDP template
Severity: Medium
Evidence: In raw HTML the `fpe` section (`pdp_europe_main`) has empty placeholders: `data-fpe-grand`, `data-fpe-one-per`, `data-fpe-sub-tiles`, `data-fpe-one-tiles` and "Arrives" with no date. The visible price text, the bundle tiles, "Arrives Wed, Nov 4 – Mon, Nov 9" and "+ $24.99 shipping" only appear after JS (screenshot). On mobile (`screenshots/screenshots.json`) the H1 sits at y=860 against an 844 px viewport; price and Add to cart show only in a sticky bar. The breadcrumb label is the long collection title "European Stage 1 Baby Formula – HiPP, Holle, Kendamil & Jovie Newborn Nutrition" and wraps to 4 lines on mobile.
Recommendation: Server-render the base price, shipping line and stock status (Liquid) and keep JS for bundle selection. Shorten the breadcrumb label (use a separate short collection title). Verify with `curl` that the price and shipping text are in the HTML.

### 6. Two product templates are live, with stale or contradictory claims on the legacy one
Severity: Medium
Evidence: HiPP and Holle use the new `fpe` template; Lebenswert stage 1 uses an older layout ("Get this item between - (2-7 business days)", "1 Box (US) ... Shipping from EU", "Long Shelf Life of 6+ Months"). Its variant labels read "*exp date 11 Jan 2027", about 10 weeks after the audit date of 2026-11-02, which contradicts the 6+ month claim. The breadcrumb parent is `/collections/baby-formula-deals-family-savings-internal`, a collection that is not in `sitemap_collections_1.xml`.
Recommendation: Migrate remaining products to one template. Remove or make dynamic the shelf-life claim and the exp-date text, or hide expiry labels on sold-out variants. Point breadcrumbs at an indexable, sitemap-listed category. Verify by listing the templates in use per product type in the theme admin and re-crawling for "exp date" and "2017".

### 7. Collection-scoped product URLs are linked from every collection card; canonical not verified
Severity: Medium
Evidence: On /collections/hipp-formula each card links to both `/products/<handle>` (3 times) and `/collections/hipp-formula/products/<handle>` (once). Across the 71 crawled collections that link out, 770 distinct `/collections/<c>/products/<p>` URLs are linked (`raw/crawl.jsonl` `internal_link_targets`). The product sitemap contains no collection-scoped URLs (316 `/products/` URLs). My attempt to fetch one such URL and read its canonical was blocked by the Cloudflare challenge, so the canonical on those URLs is not verified. Variants are quantity bundles (1/2/4/6/12 cans); Offer URLs use `?variant=`. Variant titles embed the price text ("2 Cans (UK) - $49.99 each") and so can drift from the real price.
Recommendation: Change the card template to `product.url` without `within: collection`. Confirm that `/collections/x/products/y` canonicals to `/products/y` (curl or GSC URL inspection). Keep price text out of variant names and render it from the variant price.

### 8. 50 product pages have no canonical tag and 142 have no hreflang
Severity: Medium
Evidence: `raw/crawl-summary.json` `missing_canonical` lists 50 product pages (Weleda, Earth Mama, HiPP Baby Soft, HiPP Extra Soft diapers, LittleOak, etc.); all 50 also show hreflang = 0. Across 249 crawled products, hreflang counts are 0 (142), 3 (46), 29 (4) and 32 (57). The HTML of these pages was not saved, so the cause is not verified (it may be a parsing gap or a template variant). One emoji-handle product (`%F0%9F%8D%BC-aptamil-profutura-duoadvance-pre-infant-formula-ready-from-birth-800g`) has a canonical with lowercase percent-encoding (`%f0%9f...`), flagged as a mismatch.
Recommendation: Check one of the 50 in view-source. Make every product template output `<link rel="canonical" href="{{ canonical_url }}">`, and rename the emoji-handle product slug. Verify with a re-crawl that `missing_canonical` is empty.

### 9. Thin and overlapping category pages; no collection targets the head term
Severity: Medium
Evidence: 21 of 74 collections have fewer than 500 words and 9 fewer than 300: /collections/anti-reflux-formula 121, pregnancy-care-for-expectant-moms 182, lowenzahn-formula 215, toddler-care 215, baby-food-jars 246, postpartum-recovery-essentials 256, hipp-comfort-formula 261, mom-supplements 278. They have no FAQPage block, while 48 of 74 collections do. Overlap: `hipp-formula`, `hipp-uk-formula`, `hipp-german-formula`, `hipp-dutch-formula`, `hipp-formula-stage-1..4`, `hipp-anti-reflux-formula`, `anti-reflux-formula`, `formula-for-gassy-babies` and `sensitive-baby-formula` target adjacent queries. "European baby formula" is targeted by the homepage title ("European Baby Formula | Shop Organic European Formula Brands ...") and by `organic-baby-formula` ("Buy European Organic Baby Formula in the US"); I found no dedicated collection. The /collections/hypoallergenic-formula H1 has a stray colon ("Hypoallergenic Formula for Babies : FROM EUROPE'S TRUSTED BRANDS").
Recommendation: Expand the 9 thin collections to at least 600 words with age/stage guidance and shipping facts, or merge them into the stronger sibling and 301. Decide one primary collection per query cluster and cross-link. Verify with the crawl `words` column and a query-to-URL map in Search Console.

### 10. Faceted navigation and pagination: mostly safe, with gaps to confirm
Severity: Medium
Evidence: The HiPP collection filters are `filter.p.m.my_fields.baby_age` (10 inputs) and `filter.p.product_type` (5 inputs), with values such as "FROM BIRTH" and "BABY FORMULA". Robots blocks `*sort_by*`, `*+*` and `*filter*&*filter*`. If Shopify serialises the spaces as "+", single-filter URLs are also blocked. I could not fetch a filtered URL to confirm either the encoding or the canonical. Blocked URLs cannot pass canonical signals, so any that get linked or indexed stay unmanaged. The HiPP collection HTML links `/collections/all?sort_by=best-selling` 5 times, a URL robots.txt disallows. The crawl fetched page 1 of each collection only, so 28 of 249 products show 0 or 1 inbound links in that sample (12 with 0, e.g. premibio, bebe-m and premiriz formulas), and these are most likely deeper in pagination. Page-2+ canonical and title were not checked.
Recommendation: Check one filtered URL and one `?page=2` URL in the live HTML for canonical and robots meta. Remove sort links that point to disallowed URLs from crawlable HTML. Consider increasing the page size or adding "View all" for collections that span 6 pages. Verify with a crawl that includes `?page=` URLs.

### 11. Out-of-stock presentation in grids
Severity: Low
Evidence: /collections/goat-milk-formula shows 3 "Sold Out" badges in the saved HTML; HiPP Dutch Goat Stage 2 and 3 are cards 2 and 3, ahead of in-stock Holle items. Lebenswert stage 1 shows "Sold Out" with a back-in-stock helper script (`_BISConfig`, `inventory_quantity = 0`) but no visible "Notify me" text in the raw page, whereas the new template includes "Notify me when available". Lebenswert has 3 products in the catalog, one of them sold out.
Recommendation: Sort in-stock first (or push sold-out to the end), add a visible notify form and a link to the closest in-stock stage on sold-out PDPs. Verify visually on the three collections.

### 12. International storefronts: coverage mismatch and untested localisation
Severity: Medium
Evidence: The currency selector in `raw/home.html` lists Australia, Bahrain, Canada, Hong Kong, Japan, Kuwait, New Zealand, Philippines, Qatar, Saudi Arabia, Singapore and the UAE. The home hreflang set covers AE, AU, HK, JP, NZ, PH, QA, SA, SG in three languages each (en/ja/zh-Hant pointing to /en-intl, /ja-intl, /zh-intl), so Bahrain and Kuwait have no hreflang. `x-default` appears twice on the home and Holle pages. Canada points to www.formulandcanada.com, and handles differ (`hipp-formula-first-milk` vs `hipp-formula-stage-1`). `sitemap-children.txt` shows separate product, collection, page, blog and article sitemaps for en-intl, ja-intl and zh-intl. I did not fetch any /en-intl/, /ja-intl/ or /zh-intl/ page, so translation quality, currency display and shipping eligibility there are not measured.
Recommendation: Remove the duplicate `x-default`, align the selector and hreflang country lists, and check that ja/zh pages are translated (not English under a translated URL) before keeping them in sitemaps. Verify with Search Console International Targeting/coverage reports.

### 13. Comparison and review hub pages have almost no raw HTML content
Severity: Medium
Evidence: Raw word counts in the crawl: /pages/european-baby-formula-comparison-chart 5, /pages/compare 2, /pages/formuland-reviews 50. These look JS-rendered; I did not render them. The comparison chart is a natural landing page for the "european baby formula" cluster.
Recommendation: Server-render the comparison table (or add an intro and a static HTML table) and the review excerpts. Verify with `curl` that the key text is in the HTML.

### 14. Trust-claim drift and selective review display
Severity: Low
Evidence: Trustpilot count differs by source: "4,180 reviews" in the saved product HTML, "4,298 reviews" in the live collection screenshot, "more than 4,100" in collection copy. The widget states "Showing our 4 & 5 star reviews". The Judge.me JSON-LD marks merchant replies with `digitalSourceType: TrainedAlgorithmicMediaDigitalSource`. "Subscribe & save 7%" in the heading sits next to "Save 11%" in the price row (may depend on the selected tile; not verified).
Recommendation: Drive review counts from one live source, or drop hard-coded numbers. Label the widget accurately (or show all ratings). Keep the AI-reply disclosure if replies are AI-written.

### 15. Sitemap and on-page hygiene for products
Severity: Low
Evidence: All 316 product URLs share one `lastmod` (2026-11-02T10:26:24-08:00), so it carries no change signal. The utility product `/products/shipping-protection-formula` is in the sitemap, with a missing meta description and 0 inbound links in the crawl. 130 of 249 product titles exceed 60 characters and 226 of 249 meta descriptions exceed 160 (median 291). Title patterns vary ("| Order ... - Formuland", "Buy ... - Formuland", "– Formuland"). In the crawl, 14,206 of 22,743 images have empty alt (62%); how many are product images is not determined.
Recommendation: Exclude non-merchandise products from the sitemap (tag with `noindex`/hide from Online Store). Standardise a title template `{Brand} {Stage} ({size}) | Formuland` and keep descriptions at or under about 155 characters. Add descriptive alt text to gallery images.

### 16. Agent-commerce signals present
Severity: Info
Evidence: `raw/home.headers` includes `link: </.well-known/ucp>; rel="ucp"; version="2026-08-25"`. `sitemap_agentic_discovery.xml` lists `https://formuland.com/agents.md` (200, 836 words in `crawl.jsonl`). I did not read the content of agents.md, so any instructions in it are not assessed.
Recommendation: Review agents.md and /.well-known/ucp for accuracy against the shipping and returns facts fixed in finding 1.

## Not checked / limitations

- Marketplace pricing, seller landscape and competitor price medians: DataForSEO Merchant was not available, so none of it is measured. I did not run a search or competitor fetch for "european baby formula", so competitor context is not measured.
- Cloudflare challenged all my curl requests after the crawl; I did not bypass it. Not verified: canonical on collection-scoped product URLs, canonical and robots meta on `?page=2`, `?filter...` and `?sort_by...` URLs, the contents of /policies/shipping-policy and /policies/refund-policy, /products.json (catalog-wide stock and price data), and agents.md. Googlebot's treatment by Cloudflare was not tested.
- Catalog-wide stock rate (how many of the ~316 products are sold out) is not measured; 67 product URLs were not in the crawl sample and only 3 product pages were read in full.
- /en-intl/, /ja-intl/, /zh-intl/ and formulandcanada.com pages were not fetched.
- Merchant Center account status, feed content and diagnostics were not accessible; free-listing eligibility for imported infant formula was not assessed. Manufacturer-copy comparison was not done (no manufacturer pages fetched).
- Page speed and Core Web Vitals of PDPs were not measured here.
