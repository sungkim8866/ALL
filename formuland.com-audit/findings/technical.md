Score: PARTIAL (in progress, scored in final version)

# Technical SEO - formuland.com (PARTIAL, first-pass written 2026-11-02 ~19:05 UTC)

Status: first analysis pass from local data (raw/, crawl.jsonl, 15 intl sitemap fetches). At ~18:59:40 UTC the
egress IP started receiving Cloudflare managed-challenge responses (HTTP 403, `cf-mitigated: challenge`) on every
URL including /robots.txt, so product-head diffs are pending a challenge expiry. No bypass will be attempted.

## Interim facts (will be reorganised into Findings)
- Hreflang: product pages carry up to two independent hreflang blocks. Block A (theme, 3 tags: en-US, en-CA ->
  formulandcanada.com, x-default). Block B (Shopify Markets, 29 tags: x-default, en, 9 countries x en/ja/zh-Hant).
  Block B appears on 61/61 crawled products that are listed in the /en-intl/ product sitemap (64 products) and on 0 of the
  188 that are not. Counts in crawl: 142 products have 0 tags, 46 have 3, 57 have 32, 4 have 29.
- The 32 "0 hreflang" article/blog rows in the crawl are the HTTP 429 challenge pages, not real article templates.
- 429/403 "Verifying your connection..." is a Cloudflare managed challenge in front of Shopify.
- Primary + en-intl product sitemap lastmod is a single bulk timestamp; ja-intl/zh-intl lastmod equals the request time.
