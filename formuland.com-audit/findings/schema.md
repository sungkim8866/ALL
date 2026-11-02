Score: 62/100

# Structured data audit: https://formuland.com (Shopify, theme 168377417947)

Audited 2026-11-02. Sources: 17 saved pages in `raw/pages/` (the 18th, the blog index, was a "Verifying your connection" 429 challenge page and was re-fetched), `raw/crawl.jsonl` (500 URLs), 3 extra Chromium loads for the shipping policy, refund policy, `/collections/holle-formula` and the real blog index, and the plugin's `schema_ecommerce_validate.py` and `parse_html.py`. Working files are in `tmp-schema/`.

## Detection results

| Template | Pages checked | JSON-LD blocks found | Source (line in saved HTML) |
|---|---|---|---|
| Home | 1 | Organization + WebSite (`@graph`) | theme layout, line 2664 (identical 1,565-byte block on all 17 saved pages) |
| Product (HiPP, Holle, Lebenswert) | 3 | Org/WebSite + **Product (theme)** + **Product (Judge.me)** | theme line 4690; Judge.me `class="jdgm-server-jld"` line 6110 |
| Collection (hipp, goat-milk, hypoallergenic) | 3 | Org/WebSite + FAQPage (7 Q each) | no CollectionPage, ItemList or BreadcrumbList |
| Collection `/collections/holle-formula` (extra fetch) | 1 | Org/WebSite + CollectionPage + BreadcrumbList (56 KB, 73 nested Products) + FAQPage | custom block |
| Article | 2 | Org/WebSite + Article | no BreadcrumbList |
| Pages | 8 | Org/WebSite on all; AboutPage + a second Organization on `/pages/about-us` and `/pages/our-company`; FAQPage on `/pages/formuland-faqs` (27 Q) | reviews page, comparison chart, compare, recall page: no page-specific schema |
| Blog index (re-fetched) | 1 | Org/WebSite only | no Blog/CollectionPage/BreadcrumbList |

- All 31 JSON-LD blocks in the saved pages parse as valid JSON. `crawl-summary.json` `invalid_jsonld` is empty.
- Microdata and RDFa: none. There is no `itemtype`, `itemprop` or `typeof` on any saved page, so no format conflicts.
- Crawl-wide types (468 pages returned 200; 32 blog articles returned 429 and are not measured):
  - products 249/249: Product
  - collections 74: FAQPage on 48, CollectionPage on 2, BreadcrumbList on 2 (`/collections/holle-formula`, `/collections/kendamil`)
  - articles 111/111 measured: Article, with FAQPage on 2
  - pages 32: AboutPage on 2, ContactPage on 1, FAQPage on 1
- Deprecated types (HowTo, SpecialAnnouncement, CourseInfo, EstimatedSalary, LearningVideo): none found.

## What works

- Every product page carries a Product with `name`, `image`, `url`, `sku`, `brand`, `category` and per-variant `Offer` with `price`, `priceCurrency` and `availability`. This clears the merchant-listing minimum (`name` + `image` + `offers`) on all 3 sampled products. The plugin validator reports 0 Critical and 0 High for the theme block.
- Ratings match the visible page. HiPP: markup says 4.91 / 107, and the page shows "4.91 · 107 reviews on Judge.me" with a star breakdown (102/2/2/0/1) that averages 4.906. Lebenswert: 4.57 / 7 matches "7 reviews". Holle: 4.85 / 55.
- The Organization has no self-serving `aggregateRating`. The Trustpilot score ("4,180 reviews") appears on the page but is correctly not marked up.
- The Organization/WebSite graph is clean. It has `@id`s, a logo `ImageObject`, `sameAs`, a `ContactPoint`, and `publisher` linked to the Organization.
- `Offer.availability` tracks stock. All four Lebenswert Stage 1 offers are `OutOfStock`, and the page shows "Sold Out".
- FAQPage markup mirrors visible text on all 4 pages checked (7/7, 7/7, 7/7 and 27/27 questions visible; the answer opening is found for 7/7, 7/7, 7/7 and 25/27).
- All URLs in the JSON-LD are absolute, except the Product `@id` (see finding 1).

## Findings

### 1. Two Product entities per product page with the same relative `@id` (Medium)
- **Evidence:** `products_hipp_formula_stage_1.html` has the theme Product at line 4690 (keys: name, url, image, description, sku, category, brand, offers) and the Judge.me Product at line 6110 (keys: name, aggregateRating, image, review only). Both use `"@id": "/products/hipp-formula-stage-1#product"`. The same pattern holds on Holle and Lebenswert.
  - Theme block `@context` is `http://schema.org/`; Judge.me's is `https://schema.org/`.
  - The images differ: the theme uses `formuland.com/cdn/shop/...9332888_1200x.jpg`, Judge.me uses `cdn.shopify.com/s/files/1/0647/7493/0651/files/...9332888.jpg`.
  - The validator marks the Judge.me block High `missing-product-offers`; the theme block alone passes.
- **Why it matters:** offers and reviews sit on separate Product nodes. A JSON-LD processor merges them because the relative `@id` resolves to one IRI, but Google documents each block as its own item. I could not run the Rich Results Test here, so how Google merges them is not measured.
- **Recommendation:**
  1. Preferred: turn off Judge.me's JSON-LD output (Judge.me settings, rich snippets / JSON-LD) and have the theme emit `aggregateRating` and `review` inside the single Product (template below). Check which Judge.me or Shopify review metafield your theme exposes before wiring it. I did not verify metafield names.
  2. Minimum: change the theme `@id` to an absolute URL (`{{ shop.url }}{{ product.url }}#product`).
- **Verify:** view source on a product page and count `application/ld+json` blocks with `"@type":"Product"` (target 1). Run Google's Rich Results Test on the URL.

### 2. Return policy on the product page contradicts the refund policy, so `hasMerchantReturnPolicy` cannot be written truthfully yet (High)
- **Evidence:**
  - `/policies/refund-policy` (loaded in Chromium): "We regretfully do not accept returns... Baby formula is regarded as being non-returnable."
  - HiPP product page, "Baby Satisfaction Guarantee": "covers one opened formula item per order with store credit. Unopened items of the same formula may also be returned for store credit. Contact Customer Care within 30 days of your order date."
  - No `hasMerchantReturnPolicy` exists in any JSON-LD (all 3 products plus the 73 nested products on the Holle collection page).
- **Why it matters:** Google requires return markup to match the visible policy, and shoppers (YMYL infant nutrition) see two answers. This is a business and content decision, not a markup one.
- **Recommendation:** reconcile the two texts first, then add exactly one of these two snippets to the Product (each passes the plugin's return-policy check):
  - Option A (policy page wins): `"hasMerchantReturnPolicy": {"@type":"MerchantReturnPolicy","applicableCountry":"US","returnPolicyCategory":"https://schema.org/MerchantReturnNotPermitted","url":"https://formuland.com/policies/refund-policy"}`
  - Option B (guarantee wins): `"hasMerchantReturnPolicy": {"@type":"MerchantReturnPolicy","applicableCountry":"US","returnPolicyCategory":"https://schema.org/MerchantReturnFiniteReturnWindow","merchantReturnDays":30,"refundType":"https://schema.org/StoreCreditRefund","url":"https://formuland.com/policies/refund-policy"}`
  - `returnFees` and `returnMethod` are not stated on any fetched page, so they are deliberately omitted.
- **Verify:** `schema_ecommerce_validate.py` no longer reports `missing-return-policy`, and Merchant Center return settings (not accessible here) say the same thing.

### 3. No `shippingDetails` on any Offer (Medium)
- **Evidence:** the theme block has no `shippingDetails` (validator: Medium `missing-shipping-details` on all 3 products). The shipping policy (`/policies/shipping-policy`, "Last updated: November 1, 2026") gives:
  - "free shipping on orders of $100 or more"
  - "Most orders are delivered within 2–7 business days after processing"
  - "processed and shipped as quickly as possible, often the same or next business day"
  - the refund policy adds a "maximum ship-out time... 24 hours"
- **Gap:** the shipping charge for US orders under $100 is not published on any page I fetched. The cart says "Spend $100 more for free shipping" and the product page says "Free shipping included" next to the one-time purchase, which is ambiguous. I did not guess a rate.
- **Recommendation:** the template below adds `shippingDetails` (rate 0 USD, destination US, handling 0–1 day, transit 2–7 days) to the offers priced at $100 or more (4, 6 and 12 cans for HiPP). Add a flat-rate `shippingDetails` to the 1-can and 2-can offers once the under-$100 charge is confirmed. The 2-can HiPP price is $99.98, just under the threshold.
- **Verify:** validator output. Expect it to keep flagging `missing-shipping-details` until the 1-can offer (`offers[0]`, the only one the script inspects) carries a rate.

### 4. No GTIN in the markup although valid barcodes exist, and multipack variants reuse the single-unit barcode (Medium)
- **Evidence:** the Shopify data embedded in each product page has `barcode` values that pass the EAN-13 checksum: HiPP Stage 1 `4062300460461`, Holle Stage 1 `7640230491068`, Lebenswert Stage 1 `7640230492317`. None of the three Product nodes has `gtin13`/`gtin`/`mpn`.
- **Data problem:** on HiPP, the 1-, 4-, 6- and 12-can variants all carry the 1-can barcode, and the 2-can variant has `null`. On Holle, all 4 variants share one barcode. A 4-pack is not the same trade item as a single can, so GTIN matching would misidentify the bundles.
- **Recommendation:** emit `gtin13` at Product level from the single-unit variant only (template below). In Shopify admin, clear the barcode on multipack variants or give them their own GTIN, if the manufacturer supplies one.
- **Verify:** `grep -o '"gtin13"' ` on the page source returns 1 hit, and the Shopify variant barcodes differ per pack size or are empty.

### 5. Article markup: `mainEntityOfPage` points at the homepage, and the author is a "Person" named Formuland with no credentials (Medium)
- **Evidence:** both sampled articles have `"mainEntityOfPage": {"@type":"WebPage","@id":"https://formuland.com"}` (the homepage, not the article URL), `"author": {"@type":"Person","name":"Formuland"}`, `publisher` with only a name, and `@context` `http://schema.org`.
- **Dates:**
  - "Best Organic Baby Formula": JSON `dateModified` is `2026-10-29T11:50:27Z`, but the visible text says "Updated May 27, 2026".
  - "What to Feed Your Infant?": JSON `dateModified` is 2025-04-21 and shows no visible update date.
  - `dateCreated` is not a Google-read property.
  - `articleBody` repeats 19,526 characters of visible text in the first article.
- **E-E-A-T:** the visible byline is "Posted by Formuland". There is no named author, reviewer or credential on either article, for infant-feeding content. Schema cannot fix this and I will not invent a name.
- **Recommendation:**
  1. Use the article template below (`BlogPosting`, correct `mainEntityOfPage`, `author` typed as the Organization it actually is, publisher with logo, ISO dates that match the visible date, no `articleBody`).
  2. When a real pediatric or dietitian author or reviewer exists, add `author` as a `Person` with `name`, `url` (an author page), `jobTitle` and `sameAs`. Put the reviewer on the page's `WebPage` node using `reviewedBy`. Only add this for people who actually reviewed the page.
  3. Make the visible "Updated" date and `dateModified` come from the same field.
- **Verify:** Rich Results Test shows `mainEntityOfPage` equal to the canonical URL; `dateModified` equals the visible date.
- **Not measured:** the other 109 articles. The 32 articles that returned 429 in the crawl are also not measured. Pattern is likely template-wide, since both samples match.

### 6. BreadcrumbList is missing from products, articles and 72 of 74 collections (Medium)
- **Evidence:** crawl `schema_types`: BreadcrumbList on 2/74 collections (`/collections/holle-formula`, `/collections/kendamil`), 0/249 products, 0/111 articles. The product page does show a visible breadcrumb, `Home / European Stage 1 Baby Formula – HiPP, Holle, Kendamil & Jovie Newborn Nutrition (/collections/stage-1-formula) / HiPP UK Organic Stage 1 from Birth (800g)`. Collections and articles have none.
- **Recommendation:** add BreadcrumbList to the product template, mirroring the visible trail (included in the paste-ready graph below). For collections, `Home > Collection name` (the pattern `/collections/holle-formula` already uses) is fine. For articles, `Home > Blog name > Article`.
- **Verify:** Rich Results Test, Breadcrumbs report in Search Console.

### 7. Collection `CollectionPage` markup exists on 2 of 74 collections and embeds 73 full Product nodes (Medium)
- **Evidence:** `/collections/holle-formula` has a 56,102-character block: a CollectionPage with an `ItemList` of 73 nested `Product`s, each with `offers`, `aggregateRating` and `priceValidUntil` of 2026-11-09 (the product pages say 2026-11-03). `availability` there uses `https://schema.org/...`, whereas the product page uses `http://schema.org/...`. Nested ratings are strings (`"5.0"`, `"3"`). The other 72 collections have no CollectionPage or ItemList.
- **Why it matters:** Google scopes Product and merchant-listing markup to a page about a single product, and carousel markup to a list of URLs. Per-product offer and rating data on a category page can disagree with the product pages (rolling dates already do). I did not compare ratings against the product pages.
- **Recommendation:** keep `CollectionPage` + `BreadcrumbList`, but reduce `ItemList` entries to `ListItem` with `position` and `url`, and drop nested `offers` and `aggregateRating`. Roll the pattern out to the other collections via the collection template, or leave collections to `BreadcrumbList` only.
- **Verify:** block size drops by about 90%; no `"@type":"Product"` inside collection pages.

### 8. Product `description` is the whole page body, truncated at 5,000 characters mid-sentence (Low)
- **Evidence:** HiPP theme description is exactly 5,000 characters and ends "...Pour the right amount of cooled water into a clean, sterilised b..." It includes the full nutrition table, double-escaped entities (`&lt;0.0050mg`, `&amp;`), 6 emoji and the dent disclaimer. Holle (4,108) and Lebenswert (3,690) are not truncated but are equally long.
- **Recommendation:** use a short plain-text description (first paragraph; see the template). In the theme Liquid: `{{ product.description | strip_html | strip_newlines | truncate: 300 | json }}`, or a metafield with a summary.
- **Verify:** description under about 300 characters, no `&lt;`.

### 9. `priceValidUntil` is a rolling date (Low)
- **Evidence:** all offers on the 3 product pages say `2026-11-03`, which is the fetch date (2026-11-02) plus one day. Collections use +7 days (`2026-11-09`).
- **Why it matters:** the date says the price expires tomorrow, on every render. If a cached copy is read after that date, the price looks expired. It is optional, and the template omits it.
- **Recommendation:** remove it unless a real sale end date exists.

### 10. Offers are an unlabeled array of pack sizes, with `http://` context and availability URLs (Low)
- **Evidence:** HiPP has 5 offers (1, 2, 4, 6, 12 cans: $49.99, $99.98, $195.96, $287.94, $551.88) with no `name`, so the offers differ only by `sku` and `?variant=` URL. `@context` is `http://schema.org/` and `availability` is `http://schema.org/InStock`. Ratings are strings (`"ratingValue": "4.91"`). `sku` values contain spaces and parentheses, e.g. `HIPP UK 1 (can) x 1 Unit`.
- **Recommendation:** template adds `name` from the on-page variant title ("4 Cans (UK)"), `itemCondition` NewCondition (the page says "brand-new, undamaged"), `seller`, and `https://` URLs; numeric ratings. I considered `ProductGroup` (the validator suggests it), but Google's `variesBy` list covers size/colour/material/pattern and so on, not pack quantity, so I kept the single Product with labeled Offers. I also tried `includesObject` for the pack quantity and dropped it: the nested `typeOfGood` Product stubs were flagged as extra Products missing `offers` and `image`.

### 11. About pages: duplicate, slightly conflicting Organization entities (Low)
- **Evidence:** `/pages/about-us` and `/pages/our-company` each emit a second Organization with the same `@id` (`https://formuland.com#organization`) as the global block, but with a shorter description and a `sameAs` that omits Trustpilot. Both AboutPages have the identical `name` "About Us – Formuland". The `our-company` description is cut off ("...ensuring."). The Facebook `sameAs` has a tracking parameter (`...61575078929814/?_rdr`). The claims differ between pages: "Founded in 2014 in California" (about-us); "over 500,000 orders" and "main office... in Europe" (our-company); "over 1 million orders" (product page) and "over 1 million families" (home).
- **Recommendation:** in the AboutPage graphs reference the Organization by `@id` only, give each AboutPage a unique `name`, clean the Facebook URL, and fix the conflicting claims on the visible pages. The Organization has no `address`, `telephone` or `legalName`: none is on the home page, so nothing can be added without the business supplying it.

### 12. FAQPage on 51 pages: no Google SERP benefit (Info)
- **Evidence:** 48 collections, `/pages/formuland-faqs` (27 Q) and 2 articles.
- **Status:** the brief says Google restricted FAQ rich results to authoritative government and health sites. That has since been superseded: Google retired FAQ rich results for all sites on 2026-05-07, so there is no SERP feature for this site either way. The markup is valid, matches visible text and is not harmful. Any AI/GEO visibility benefit is unconfirmed.
- **Recommendation:** no action needed. Optional: drop it from templates to save about 3.5–12 KB per page. Do not add FAQPage to new pages for SERP reasons. Use `QAPage` only for genuine user Q&A.

### 13. Reviews and WebSite details (Info)
- Judge.me `Review.comment` carries merchant replies with `"digitalSourceType": "https://schema.org/TrainedAlgorithmicMediaDigitalSource"` on 6/20 HiPP and 5/21 Holle reviews. That is Judge.me's AI-reply disclosure; benign, and Google ignores it.
- `author.name` is "Anonymous" on 2/20 HiPP and 6/21 Holle reviews. Google's review-snippet guidance asks for a valid author name; I did not confirm in the Rich Results Test whether these are discarded.
- Only 20 of 107 HiPP reviews are in the markup (Judge.me page size), which is normal.
- `WebSite.SearchAction` is valid (`https://formuland.com/search?q={search_term_string}`). Google retired the sitelinks search box in November 2024, so there is no SERP effect. Harmless; I did not test that the search URL works.
- The Organization is hard-coded in the layout (`https://formuland.com#organization`, no trailing slash) and identical on every template, including the `/en-intl/404` page. Fine for a single-locale store.

## Validator output (`schema_ecommerce_validate.py`, extracted Product JSON-LD)

| Product | Block | Critical | High | Medium | Info | Rules hit |
|---|---|---|---|---|---|---|
| HiPP Stage 1 | theme | 0 | 0 | 3 | 1 | missing-return-policy, missing-shipping-details, missing-member-program, no-product-group |
| HiPP Stage 1 | Judge.me | 0 | 1 | 4 | 1 | adds missing-product-offers (High), recommended-product-description |
| Holle Stage 1 | theme / Judge.me | 0 / 0 | 0 / 1 | 3 / 4 | 1 / 1 | same as HiPP |
| Lebenswert Stage 1 | theme / Judge.me | 0 / 0 | 0 / 1 | 3 / 4 | 1 / 1 | same as HiPP |
| Naive merge of both blocks | HiPP, Holle, Lebenswert | 0 | 0 | 3 | 1 | return policy, shipping, member program, product group |
| **Corrected template** (below) | HiPP, with Option A or B | 0 | 0 | 2 | 1 | missing-shipping-details (see finding 3), missing-member-program, no-product-group |

- `missing-member-program` is not applicable: the site's points programme is not member-tier pricing, and no tier prices are shown in the markup. Ignore it.
- `no-product-group` is Info only (see finding 10).
- The validator does not check GTIN, brand, `@id` form or the `http` context, so I checked those by hand.

## Paste-ready JSON-LD

Both examples use only values present on the pages I fetched. Lint check: every URL absolute and https, `@context` `https://schema.org`, dates ISO 8601, no placeholder text, EAN-13 checksum valid.

### Product template (HiPP UK Organic Stage 1, with BreadcrumbList)

Add the return policy from finding 2 (Option A or B) once the policy text is reconciled. `shippingDetails` is only on the offers at $100 or more; add a flat rate to the other two once the under-$100 charge is confirmed. The two `review` entries are the first two of Judge.me's 20; in production emit them from the app or metafield.

```json
{
  "@context": "https://schema.org",
  "@graph": [
    {
      "@type": "Product",
      "@id": "https://formuland.com/products/hipp-formula-stage-1#product",
      "url": "https://formuland.com/products/hipp-formula-stage-1",
      "name": "HiPP UK Organic Stage 1 from Birth (800g)",
      "description": "From day one, your little one deserves gentle, nourishing care — and HiPP Organic Stage 1 First Infant Milk is carefully designed to provide complete nutrition from birth onwards.",
      "image": [
        "https://formuland.com/cdn/shop/files/hipp-uk-organic-stage-1-from-birth-800g-9332888_1200x.jpg?v=1768382830"
      ],
      "sku": "HIPP UK 1 (can) x 1 Unit",
      "gtin13": "4062300460461",
      "brand": {
        "@type": "Brand",
        "name": "Hipp"
      },
      "category": "Baby Formula",
      "aggregateRating": {
        "@type": "AggregateRating",
        "ratingValue": 4.91,
        "reviewCount": 107,
        "bestRating": 5,
        "worstRating": 1
      },
      "review": [
        {
          "@type": "Review",
          "reviewRating": {
            "@type": "Rating",
            "ratingValue": 5,
            "bestRating": 5,
            "worstRating": 1
          },
          "author": {
            "@type": "Person",
            "name": "Nicole K."
          },
          "reviewBody": "My baby mood, sleep, and reflux has improved drastically from starting this formula. I was recommend by a family friend who used it on his two kids.",
          "datePublished": "2026-10-31T20:54:53Z"
        },
        {
          "@type": "Review",
          "reviewRating": {
            "@type": "Rating",
            "ratingValue": 5,
            "bestRating": 5,
            "worstRating": 1
          },
          "author": {
            "@type": "Person",
            "name": "Sii M."
          },
          "reviewBody": "My newborn has not refused this formula once! I can’t say the same for other brands. My husband and I are thrilled that our baby enjoys it and we trust it’s’ clean ingredients!",
          "datePublished": "2026-10-25T22:53:55Z"
        }
      ],
      "offers": [
        {
          "@type": "Offer",
          "name": "1 Can (UK)",
          "sku": "HIPP UK 1 (can) x 1 Unit",
          "url": "https://formuland.com/products/hipp-formula-stage-1?variant=48980561166555",
          "price": 49.99,
          "priceCurrency": "USD",
          "availability": "https://schema.org/InStock",
          "itemCondition": "https://schema.org/NewCondition",
          "seller": {
            "@type": "Organization",
            "name": "Formuland",
            "@id": "https://formuland.com#organization"
          }
        },
        {
          "@type": "Offer",
          "name": "2 Cans (UK)",
          "sku": "HIPP UK 1 (can) x 2 Units",
          "url": "https://formuland.com/products/hipp-formula-stage-1?variant=48980561199323",
          "price": 99.98,
          "priceCurrency": "USD",
          "availability": "https://schema.org/InStock",
          "itemCondition": "https://schema.org/NewCondition",
          "seller": {
            "@type": "Organization",
            "name": "Formuland",
            "@id": "https://formuland.com#organization"
          }
        },
        {
          "@type": "Offer",
          "name": "4 Cans (UK)",
          "sku": "HIPP UK 1 (can) x 4 Units",
          "url": "https://formuland.com/products/hipp-formula-stage-1?variant=42957344735451",
          "price": 195.96,
          "priceCurrency": "USD",
          "availability": "https://schema.org/InStock",
          "itemCondition": "https://schema.org/NewCondition",
          "seller": {
            "@type": "Organization",
            "name": "Formuland",
            "@id": "https://formuland.com#organization"
          },
          "shippingDetails": {
            "@type": "OfferShippingDetails",
            "shippingRate": {
              "@type": "MonetaryAmount",
              "value": 0,
              "currency": "USD"
            },
            "shippingDestination": {
              "@type": "DefinedRegion",
              "addressCountry": "US"
            },
            "deliveryTime": {
              "@type": "ShippingDeliveryTime",
              "handlingTime": {
                "@type": "QuantitativeValue",
                "minValue": 0,
                "maxValue": 1,
                "unitCode": "DAY"
              },
              "transitTime": {
                "@type": "QuantitativeValue",
                "minValue": 2,
                "maxValue": 7,
                "unitCode": "DAY"
              }
            }
          }
        },
        {
          "@type": "Offer",
          "name": "6 Cans (UK)",
          "sku": "HIPP UK 1 (can) x 6 Units",
          "url": "https://formuland.com/products/hipp-formula-stage-1?variant=42957344768219",
          "price": 287.94,
          "priceCurrency": "USD",
          "availability": "https://schema.org/InStock",
          "itemCondition": "https://schema.org/NewCondition",
          "seller": {
            "@type": "Organization",
            "name": "Formuland",
            "@id": "https://formuland.com#organization"
          },
          "shippingDetails": {
            "@type": "OfferShippingDetails",
            "shippingRate": {
              "@type": "MonetaryAmount",
              "value": 0,
              "currency": "USD"
            },
            "shippingDestination": {
              "@type": "DefinedRegion",
              "addressCountry": "US"
            },
            "deliveryTime": {
              "@type": "ShippingDeliveryTime",
              "handlingTime": {
                "@type": "QuantitativeValue",
                "minValue": 0,
                "maxValue": 1,
                "unitCode": "DAY"
              },
              "transitTime": {
                "@type": "QuantitativeValue",
                "minValue": 2,
                "maxValue": 7,
                "unitCode": "DAY"
              }
            }
          }
        },
        {
          "@type": "Offer",
          "name": "12 Cans (UK)",
          "sku": "HIPP UK 1 (can) x 12 Units",
          "url": "https://formuland.com/products/hipp-formula-stage-1?variant=42957344800987",
          "price": 551.88,
          "priceCurrency": "USD",
          "availability": "https://schema.org/InStock",
          "itemCondition": "https://schema.org/NewCondition",
          "seller": {
            "@type": "Organization",
            "name": "Formuland",
            "@id": "https://formuland.com#organization"
          },
          "shippingDetails": {
            "@type": "OfferShippingDetails",
            "shippingRate": {
              "@type": "MonetaryAmount",
              "value": 0,
              "currency": "USD"
            },
            "shippingDestination": {
              "@type": "DefinedRegion",
              "addressCountry": "US"
            },
            "deliveryTime": {
              "@type": "ShippingDeliveryTime",
              "handlingTime": {
                "@type": "QuantitativeValue",
                "minValue": 0,
                "maxValue": 1,
                "unitCode": "DAY"
              },
              "transitTime": {
                "@type": "QuantitativeValue",
                "minValue": 2,
                "maxValue": 7,
                "unitCode": "DAY"
              }
            }
          }
        }
      ]
    },
    {
      "@type": "BreadcrumbList",
      "@id": "https://formuland.com/products/hipp-formula-stage-1#breadcrumb",
      "itemListElement": [
        {
          "@type": "ListItem",
          "position": 1,
          "name": "Home",
          "item": "https://formuland.com/"
        },
        {
          "@type": "ListItem",
          "position": 2,
          "name": "European Stage 1 Baby Formula – HiPP, Holle, Kendamil & Jovie Newborn Nutrition",
          "item": "https://formuland.com/collections/stage-1-formula"
        },
        {
          "@type": "ListItem",
          "position": 3,
          "name": "HiPP UK Organic Stage 1 from Birth (800g)",
          "item": "https://formuland.com/products/hipp-formula-stage-1"
        }
      ]
    }
  ]
}
```

Liquid mapping for the offers loop (not run against Shopify; test in a theme preview):

```liquid
"offers": [
{%- for variant in product.variants -%}
  {
    "@type": "Offer",
    "name": {{ variant.title | split: ' - ' | first | json }},
    "sku": {{ variant.sku | json }},
    "url": {{ shop.url | append: variant.url | json }},
    "price": {{ variant.price | divided_by: 100.0 }},
    "priceCurrency": {{ cart.currency.iso_code | json }},
    "availability": {% if variant.available %}"https://schema.org/InStock"{% else %}"https://schema.org/OutOfStock"{% endif %},
    "itemCondition": "https://schema.org/NewCondition"
  }{% unless forloop.last %},{% endunless %}
{%- endfor -%}
]
```

Set `"gtin13"` from the single-unit variant only (`product.variants.first.barcode`), and add `shippingDetails` when the variant price is 10000 or more (cents).

For out-of-stock products such as Lebenswert Stage 1 the same template yields `OutOfStock` on all offers, which matches the page.

### Article template (BlogPosting, "Best Organic Baby Formula: Which Is Safest for Your Child?")

`dateModified` uses the visible "Updated May 27, 2026". If the 2026-10-29 CMS edit was a real content change, update the visible date to match instead. In Liquid, map `headline` to `article.title`, `datePublished` to `article.published_at`, `image` to `article.image`, and `@id` / `mainEntityOfPage` to `shop.url | append: article.url`.

```json
{
  "@context": "https://schema.org",
  "@type": "BlogPosting",
  "@id": "https://formuland.com/blogs/parenting-tips/best-organic-baby-formula-which-is-safest-for-your-child#article",
  "mainEntityOfPage": {
    "@type": "WebPage",
    "@id": "https://formuland.com/blogs/parenting-tips/best-organic-baby-formula-which-is-safest-for-your-child"
  },
  "headline": "Best Organic Baby Formula: Which Is Safest for Your Child?",
  "description": "Get a side-by-side comparison of different brands to identify the best organic formula for your infant from Formuland.",
  "image": [
    "https://formuland.com/cdn/shop/articles/best-organic-baby-formula-which-is-safest-for-your-child-1061050_1200x.jpg?v=1779982687"
  ],
  "datePublished": "2016-11-15T10:39:50Z",
  "dateModified": "2026-05-27",
  "author": {
    "@type": "Organization",
    "name": "Formuland",
    "url": "https://formuland.com/"
  },
  "publisher": {
    "@type": "Organization",
    "name": "Formuland",
    "url": "https://formuland.com/",
    "logo": {
      "@type": "ImageObject",
      "url": "https://formuland.com/cdn/shop/files/cropped-formuland-logo-center_698x.jpg?v=1653702734"
    }
  }
}
```

## Not checked / limitations

- **Google tooling:** I could not run the Rich Results Test or Search Console, so how Google merges the two Product blocks and which rich results are actually shown are not measured. Merchant Center (feed, shipping and return settings) was not accessible.
- **Sample size:** the product and article findings are verified on 3 products and 2 articles, plus crawl-level type counts for the rest. Per-page block counts for the other 246 products and 109 articles are not measured. Whether products with zero reviews still emit a Judge.me block is also not measured.
- **Unmeasured pages:** 32 blog articles returned 429 in the crawl. The saved blog index was a bot-challenge page, so I re-fetched it. curl was also challenged (403) on the policy and collection pages, so those four pages were loaded once each in plain headless Chromium without any evasion.
- **Not looked at:** `/en-intl/`, `/ja-intl/`, `/zh-intl/` and formulandcanada.com (my one guess at an `/en-intl/` product URL returned a 404). I also did not compare nested collection ratings against product pages, or check that the search URL works.
- **Possible bot-challenge impact:** the "Verifying your connection" challenge appeared for curl and for the crawler. Whether Googlebot ever sees it is unknown and not measured.
- **Content claims noticed along the way (for the content reviewers):**
  - The product page meta description says HiPP UK Stage 1 contains "prebiotics, probiotics", but the on-page ingredient list shows only galacto-oligosaccharides and no probiotic culture.
  - Order-count claims differ across pages (500,000 / 1 million).
  - I found no text addressed to AI agents in any fetched page.
