Score: 68/100

(Analyst-estimated from the evidence below.)

# Image optimization - formuland.com

## Scope and sources

- Crawl data: `raw/crawl.jsonl` and `raw/crawl-summary.json` (467 HTML pages returned 200 with images; 22,743 `<img>` tags, 14,206 with empty alt, 0 with a missing alt attribute). The coordinator brief quoted 13,910 of 22,308 from an earlier pass; the final crawl summary (including retries) is used here.
- Static HTML: the 18 pages in `raw/pages/` (home, product, collection pages parsed in detail with `tmp-perf/imgs.py` and `imgs2.py`).
- Runtime: throttled mobile Chromium runs `tmp-perf/runs/home-1.json` and `pdp-1.json` (390x844, DPR 2): delivered MIME types, rendered size versus natural size, loading attributes. Lab only; the collection page was not run.

## What works

- Delivered formats are modern: Shopify's CDN negotiates on `Accept`. Home (41 image requests): 18 WebP (637 KB), 3 AVIF (47 KB), 16 SVG (63 KB). Product page (43): 14 WebP (255 KB), 7 AVIF (81 KB). No JPEG or PNG was delivered to Chromium.
- Product page main image is correctly tuned: `loading="eager"`, `fetchpriority="high"`, srcset `...width=600 600w, ...`, `sizes="(min-width: 1200px) 621px, (min-width: 961px) 45vw, calc(100vw - 40px)"`, `width="1200" height="1200"`, WebP 37 KB, and it is the LCP element at 2.83 s throttled.
- Below-the-fold images are lazy: home 86 `loading=lazy` of 122 images, product page 38 of 72.
- Every `<img>` in the crawl has an `alt` attribute (0 missing across 22,743), and `og:image` is present on all 500 crawled URLs (`crawl-summary.json`: `no_og_image` empty).
- Product gallery and collection card images carry alt text (product page: "HiPP UK Organic Stage 1 from Birth (800g) - Formuland"; collection page `collection_custom_2` section: 30 non-empty alts).

## Findings

### 1. Desktop hero end-image is downloaded on mobile (210 KB, hidden) - Severity: High
Evidence: `raw/home.html` hero has two end images, both `loading="eager"`. The desktop one (`556e25b3681582df669e1b8b784580bc323c2013.jpg?width=1920`, class `fhv2__end-image--desktop has-mobile`) is `display:none` at max-width 768px, yet in the mobile run it was fetched: 210 KB WebP, natural 1920x1255, rendered 0x0. That is 28% of the home page's 750 KB of image bytes, and the biggest image request on the page. The mobile image (`header-section_1.png?width=900`) is 37 KB WebP. Both are `alt=""` and not `aria-hidden`. The hero video itself is a separate, much larger problem (see `performance.md` Finding 1).
Recommendation: replace the two sibling `<img>` with one `<picture>` (`<source media="(min-width:769px)" srcset=...>` plus the mobile `<img>`), or render only the matching one in Liquid; do not mark a hidden image eager. Verify: the mobile network log must no longer contain the `width=1920` file; image bytes on home should drop by about 210 KB.

### 2. 28 country-flag images with `loading="eager"` on every page - Severity: Medium
Evidence: every page has 28 `<img class="fl-cs__flag" src="//cdn.shopify.com/static/images/flags/xx.svg?width=40" alt="" width="20" height="14" loading="eager">` in the header country selector (`raw/pages/*.html`, 17 of 17 parsed pages with images). That is 13,076 of the 22,743 images in the crawl (57.5%). In the runs they produced 13 unique SVG requests per page load, plus the logo SVGs (16 SVG requests, 63 KB total on home). The selector is a hidden dropdown.
Recommendation: render the flags only when the selector opens (or use `loading="lazy"`, or a single sprite / CSS flags), which also removes 28 DOM nodes and requests from each of the 500 pages. Verify: eager image count on the home page falls from 31 to about 3, and no `/static/images/flags/` requests occur before the dropdown is opened.

### 3. Empty-alt images: mostly decorative flags; a small residual needs review - Severity: Medium
Verdict: the 14,206 empty alts are NOT duplicate gallery images and NOT missing product-image alts.
Evidence:
- 13,076 of 14,206 (92.0%) are the 28 header country flags above (28 per page x 467 pages). `alt=""` is correct for these because the country name is adjacent text; the number is a template artefact and not a content problem.
- The remaining 1,130 (5.0% of all images) in the 18 saved pages are: section background images marked `aria-hidden="true"` (for example `collection_goat_why_buy`, `fcc_org_why_buy`, `pdp_trust_signals`, `pdp_guarantee_banner`, `home_about_us`), which are correctly decorative; and a few that look meaningful or functional and need a decision: homepage hero end images (not `aria-hidden`); blog images inside links with `alt=""` (`acid-reflux-in-infants.png`, `baby-formula-myths.png`, `feeding-baby-formula.png`, in `raw/pages/blogs_parenting_tips_best_organic_baby_formula_which_is_safest_for_your_child.html`), which leaves the link without a name unless it has other text (not verified); `jovie.jpg` in the same article body; the Lebenswert product page icons (`fast-delivery_540x.png`, `sent_...png`, `guarantee-certificate_...png`, `shelf-life-expired_1_540x.png`, `shipping-value-prop.png`), probably decorative beside text (not verified); About page `section.webp` / `section-2.webp` backgrounds.
- Alt text quality for non-empty alts (product page run): the same alt "HiPP UK Organic Stage 1 from Birth (800g) - Formuland" is reused on 5 gallery images, "Review Image" x4, "Formuland Logo" x3, and `ClickCease` for a noscript tracking pixel.
Recommendation: keep `alt=""` on flags and on `aria-hidden` backgrounds; add descriptive alt (or `aria-label` on the link) to the link-wrapped blog thumbnails and any informative article images; give each product gallery image a distinct alt (front of tin, back label, scoop, ingredients), and replace "Review Image" with a short description. Verify with a crawl that counts empty-alt images excluding `.fl-cs__flag` and `aria-hidden` ancestors; the residual should only be decorative.

### 4. Low srcset coverage and some oversized images - Severity: Medium
Evidence (rendered images in the mobile runs): home 11 of 44 have srcset/data-srcset and sizes; product page 16 of 55. Over-sized images (natural width more than 3x CSS width at DPR 2): 5 on home, 9 on the product page. Examples: Tapcart banner icon `assets.tapcart.com/app-config/6BvYk09PBe_....jpg` 768x768 AVIF 35 KB rendered at 60x60 (third-party, on every page); wipes thumbnails `...infused-moisturizing-wipes-9484036.jpg?width=400` 400x400 shown at 30x30 and 66x66 (cross-sell/cart items); 400x265 screenshot PNG shown at 66x66. `<picture>` is used for 0 images on home and 1 on the product page.
Recommendation: emit 2-3 widths with `sizes` for product cards and cross-sell items (Shopify `image_url: width:` variants at 1x/2x of the displayed size), request `width=120/200` for small thumbnails, and ask Tapcart to serve a smaller icon. Verify with the Lighthouse "Improve image delivery" insight.

### 5. Missing width/height attributes on rendered images - Severity: Low
Evidence: in the runs, 17 rendered images on home and 29 on the product page lack a `width` and/or `height` attribute. Measured CLS is still tiny (0.0015-0.0062), so impact today is low, but any slower image on a real device can shift the layout. Which individual elements are affected was not itemised.
Recommendation: output `width` and `height` on every `<img>` (Shopify `image.width`/`image.height`) or use CSS `aspect-ratio`. Verify by counting `img:not([width]):not([height])` in DevTools and re-measuring CLS.

### 6. Eager images on above-the-fold list are inflated - Severity: Low
Evidence: home has 31 eager, 86 lazy, 5 without a loading attribute; product page 30 eager, 38 lazy, 4 without. Most eager ones are the 28 flags plus header logo SVGs. Product page thumbnails (58x58) are lazy, which is acceptable.
Recommendation: fix via Finding 2 and add `loading="lazy"`/`decoding="async"` to the remaining images that have no attribute.

### 7. Heavy source files reach clients that do not send `Accept: image/webp` - Severity: Info
Evidence: `curl -I 'https://formuland.com/cdn/shop/files/header-section_1.png?v=1787083354&width=900'` (no Accept header) returned `content-type: image/png`, 462,095 bytes, `vary: Accept`, whereas Chromium received 37 KB WebP for the same URL. Browsers are fine; some crawlers, scrapers and email clients may get the large PNG.
Recommendation: upload hero artwork as JPEG/WebP instead of PNG, and keep `og:image` files small.

## Not checked / limitations

- Collection page images were not run in the browser (static analysis of `raw/pages/collections_hipp_formula.html` only: 88 `<img>`, 37 empty alt of which 28 are flags).
- Only home and product page were measured at runtime, 1 throttled run each (`home-1`, `pdp-1`); no medians. Rendered-versus-natural sizes are for a 390 px viewport at DPR 2 only; desktop not measured.
- Alt-text quality of the non-empty alts was reviewed for the product page and partly for home and collection pages only, not for all 22,743 images; the 1,130 residual empty alts were inspected only on the 18 saved pages, not site-wide.
- Which links wrap the empty-alt blog images, and whether they have other accessible text, was not verified.
- Image file sizes at source (upload) resolution and EXIF/metadata were not checked.
