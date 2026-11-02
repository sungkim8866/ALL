Score: 30/100

(Analyst-estimated from lab evidence below; this is NOT a Lighthouse/PSI score. PSI and CrUX were unavailable, so there is no field data.)

# Performance / Core Web Vitals - formuland.com

## Method, and what is and is not measured

- Tool: headless Chromium 153 through the egress proxy, mobile profile 390x844 DPR 2, mobile UA, CDP `Emulation.setCPUThrottlingRate` 4, CDP `Network.emulateNetworkConditions` 1.6 Mbps down / 750 Kbps up / 150 ms latency, cold cache, 30 s observation window, PerformanceObserver for LCP / layout-shift / longtask / paint, CDP Network events for bytes and requests. Scripts: `/home/user/ALL/formuland.com-audit/tmp-perf/measure.py`, `agg.py`; raw runs in `tmp-perf/runs/*.json`.
- **These are lab numbers from a datacenter (US-East, via a proxy), not field data.** No CrUX/PSI data exists for this audit, so Core Web Vitals pass/fail at the 75th percentile cannot be stated.
- **The requested 3 runs x 3 pages with medians was NOT completed** (stopped on coordinator instruction). Completed runs only:
  - Home, throttled: 2 runs (`diag-home-0`, which also took periodic screenshots and so added overhead, and `home-1`). Both reported below as a range. No median is claimed.
  - Product `/products/hipp-formula-stage-1`, throttled: 1 run (`pdp-1`).
  - Collection `/collections/hipp-formula`: **not measured**. Only static HTML analysis of `raw/pages/collections_hipp_formula.html` was done.
  - Home, unthrottled: 1 run (`diag-home-fast-0`), used as a fast-network comparison.
- **INP cannot be measured without real interaction and was not measured.** TBT proxy below is only an indicator of main-thread risk.
- TBT proxy = sum of (long task duration - 50 ms) for long tasks starting after FCP until the end of the 30 s window (not Lighthouse's FCP-to-TTI). CPU throttle 4x applies to a datacenter CPU, so absolute values are not comparable with Lighthouse. CDP latency is applied per request, not per connection setup.
- "Third-party" below excludes formuland.com and *.shopify.com/shopifycdn/shopifysvc. App code that Shopify hosts on cdn.shopify.com/extensions (Klaviyo app.js, Judge.me loader, Appstle, CartBite, GWP) is therefore counted as first-party and the third-party share is an under-count.

## Lab results

| Metric | Home, throttled (2 runs) | Product, throttled (1 run) | Home, unthrottled (1 run) | Collection |
|---|---|---|---|---|
| TTFB | 543 / 599 ms | 414 ms | 250 ms | not measured |
| FCP | 2,656 / 2,840 ms | 2,580 ms | 832 ms | not measured |
| LCP (final in 30 s window) | 9,072 ms (announcement-bar text; video never rendered a frame) / 24,504 ms (VIDEO) | 2,832 ms (IMG, product main image) | 24,720 ms (end IMG; earlier VIDEO candidate at 3,144 ms) | not measured |
| CLS (no-input shifts) | 0.0053 / 0.0062 | 0.0015 | 0.0049 | not measured |
| TBT proxy (after FCP) | 5,561 / 4,402 ms (60 / 45 long tasks) | 5,227 ms (47 long tasks) | 763 ms (14 long tasks) | not measured |
| DOMContentLoaded | 9,005 / 8,780 ms | 9,234 ms | 1,897 ms | not measured |
| load event | not fired in 30 s / 28,187 ms | 26,445 ms | 5,162 ms | not measured |
| Requests | 263 / 274 | 399 | 418 | not measured |
| Transfer, total in window | 5,453 / 5,520 KB (incl. 1.5 MB of partial video) | 4,473 KB | 100,741 KB (95,149 KB is hero video) | not measured |
| Transfer excl. video | 3,879 / 4,000 KB | 4,473 KB | 5,593 KB | not measured |
| JS transfer / decoded | 2,580 / 8,340 KB and 2,632 / 8,423 KB | 2,975 / 9,253 KB | 2,661 / 8,427 KB | not measured |
| Third-party share of non-video bytes | 60.3% / 59.9% | 58.4% | 43.3% | not measured |
| Runtime DOM elements | 2,970 / 2,982 | 3,296 | 3,116 | not measured |

Third-party domains (home-1, compressed bytes): googletagmanager.com 8 req / 1,342 KB; facebook.net 2 / 231 KB; klaviyo.com 24 / 189 KB; gstatic.com 5 / 130 KB; tapcart.com 2 / 105 KB; appstle.com 1 / 89 KB; clarity.ms 12-14 / 56 KB; googleapis.com 7 / 48 KB; clickcease.com 1 / 44 KB; cloudflare.com (cdnjs) 3 / 35 KB. On the product page additionally jsdelivr.net 9 / 208 KB. Total third-party: 110 requests / 2,395 KB (home), 127 requests / 2,612 KB (product).

Lab verdict against thresholds (single throttled runs, indicative only): product LCP 2.83 s is just above the 2.5 s "good" line; home LCP is 9 s to 24.5 s depending on the run and whether the hero video renders (see Finding 1); CLS is well inside "good"; INP unknown.

## What works

- TTFB is good: 250 ms unthrottled, 414-599 ms throttled including 150 ms emulated latency (below the 0.8 s guideline). `raw/home.headers`: `server-timing: processing;dur=183, render;dur=100`, Cloudflare in front, `alt-svc: h3`. HTML is `cache-control: private, max-age=0, must-revalidate` / `cf-cache-status: DYNAMIC`, which is standard Shopify behaviour.
- 103 Early Hints are sent (`raw/home.headers`): preconnect to cdn.shopify.com plus preloads for font-settings.css, theme.css, Klaviyo app.css, gwp.css and the portable-wallets CSS.
- HTML compresses 5.8:1 on the wire (encodedBodySize 123,292 B vs decodedBodySize 720,531 B in the home run).
- Theme JS is non-blocking: lazysizes `async`, vendor.js and unminified-theme-js.js `defer`; custom.min.css, custom-cart-drawer.css and custom-styles.css use the preload + `onload` swap pattern. GTM, ClickCease, Klaviyo and Ahrefs are injected async.
- Product page LCP image is correctly prioritised: `loading="eager"`, `fetchpriority="high"`, `src` present in the HTML (not lazysizes-only), srcset + sizes, width/height 1200x1200, WebP 37 KB. LCP 2.83 s throttled, only 250 ms after FCP.
- CLS in the lab is excellent on all runs: 0.0015-0.0062.

## Findings

### 1. Hero is a 48.5 MB 1080p60 video that gates the H1, LCP and CTA - Severity: Critical
Evidence:
- `raw/home.html` hero section: `<video ... autoplay muted playsinline preload="auto" fetchpriority="high"><source src="https://cdn.shopify.com/videos/c/o/v/0e80a1164c1044978c48e20afc7ca391.mp4">` with no `poster`, plus `<link rel="preload" as="video" href="...0e80a1164c1044978c48e20afc7ca391.mp4">`. The video has no `has-mobile` class and `.fhv2-desktop-video.has-mobile{display:none}` only applies with that class, so mobile receives the same file.
- `curl -I`: `content-length: 48543322` (48.5 MB). `ffprobe` on the file (head + tail ranges): H.264 Main, 1920x1080, 60 fps, 20.2 s, about 19.2 Mbps, plus an AAC track on a muted video. The container box order is `ftyp, free, mdat, moov`: the `moov` atom is at the END of the file (not faststart), so playback metadata needs a tail request before the video can start.
- `.fhv2__overlay{opacity:0}` (contains the H1, description and CTA) becomes visible only on the video `ended` event (inline script in the hero section). Text is in the DOM and crawlable, but invisible to users until the video finishes.
- Throttled diagnostic run (`diag-home-0`): after 30 s the video had `readyState 0`, `duration NaN`, overlay opacity 0. The filmstrip (`tmp-perf/runs/diag-home-filmstrip.png`) shows an empty dark-green hero from 2 s to 25 s; LCP candidate was only the announcement-bar `<p>` at 2.8 s then 9.1 s. Second throttled run (`home-1`): LCP = VIDEO at 24,504 ms, overlay still opacity 0 at 30 s.
- Unthrottled run: video first frame at 3,144 ms (LCP candidate), the H1/CTA appear only after the video ends, and the final LCP is the end image at 24,720 ms. 95,149 KB was transferred for the video in 30 s (four range responses of 47.4, 37.1, 8.9 and 1.7 MB), about twice the file size.
Caveat: in field data LCP is finalised at the first user input, so real users who scroll early will not record 24 s. The H1/CTA being invisible, the 48.5 MB download and the blank hero on slow connections are real regardless.
Recommendation:
- Serve a mobile rendition of at most 1-2 MB (720p or lower, 24-30 fps, no audio track, `-movflags +faststart`, H.264 plus WebM/AV1) and a `poster` (the existing end image, `fetchpriority="high"`). Remove the `<link rel=preload as=video>` and `preload="auto"` (use `metadata`/`none`), and skip the video for `prefers-reduced-motion` and `Save-Data`.
- Remove the `opacity:0` gating: show the H1/description/CTA from first paint so the H1 is the LCP element, and keep the video decorative.
- Verify: re-run Lighthouse mobile (or WebPageTest) and confirm the LCP element is the poster/H1 with LCP under 2.5 s, hero transfer under 1.5 MB, and H1 visible at FCP.

### 2. Excessive JavaScript and main-thread work - Severity: High
Evidence: 124-193 script requests per page; JS transfer 2.58-2.98 MB compressed, 8.3-9.3 MB decoded; TBT proxy 4.4-5.6 s at 4x CPU (home and product), 45-60 long tasks (largest 420-672 ms); DOMContentLoaded 8.8-9.2 s and load 26-28 s throttled; 263-399 requests per page load; chrome `Performance.getMetrics` TaskDuration 24-26 s, ScriptDuration 7.6-8.3 s in the 30 s window (throttled). Unthrottled TBT proxy 763 ms, so the cost is CPU-bound on mobile hardware.
Recommendation: audit every Shopify app embed and remove or restrict it to the templates that need it (see Findings 3 and 4); defer non-critical app scripts until interaction/idle; target under 1 MB compressed JS and TBT proxy under 600 ms in the same harness. Verify by repeating `tmp-perf/measure.py` for home, product, collection with 3 runs each and comparing medians.

### 3. Third-party and app scripts make up about 60% of non-video bytes - Severity: High
Evidence: see the third-party table above. GTM alone is 1,342 KB: `gtm.js` plus `gtag/js` for G-2MMKLEGEPK, AW-975144250 and a `gtag/destination` for MC-8H6VWWEVEX. Clarity (12-14 requests) is commented out in the theme HTML (`<!-- <script ... clarity.ms/tag/ ... -->` in `raw/home.html`) yet still loads, so it is most likely injected by the GTM container (inference, not verified). Other third parties in the HTML: Klaviyo (24 requests), Facebook, ClickCease (`clickcease.com/monitor/stat.js` injected in `<body>`), VWO (`dev.visualwebsiteoptimizer.com`), Ahrefs analytics, Trustpilot widget, Judge.me, Appstle x3 (subscription, loyalty, add-to-subscription widget), CartBite/GWP (`s1.staq-cdn.com`, with a fallback to `js-server.staqlab.com`), Tapcart capture-kit from unpkg and a Tapcart app banner.
Recommendation: review the GTM container and remove duplicate or unused tags, consider server-side tagging, delay Clarity, ClickCease, Ahrefs, Trustpilot and the Tapcart banner until after load or first interaction, and confirm that each of VWO, Klaviyo, Appstle, CartBite and Tapcart is still needed. Verify with the same harness (third-party bytes and TBT proxy before/after).

### 4. 720 KB HTML (median 687 KB site-wide) dominated by inline app JSON/JS - Severity: High
Evidence:
- `raw/home.html`: 719,812 bytes; 145 KB of it is `<head>`. Inline `<script>`: 123 tags, 372 KB (51.7%): Appstle subscription helper 97 KB, Appstle loyalty `_ALConfig` 63 KB, Shopify web-pixel loader 36 KB, Judge.me `jdgmSettings` 34 KB, GWP config 11 KB. Inline `<style>` 29 tags, 100 KB. 284 inline SVGs, 73 KB. About 2,687 tags in source. Four inline accessibility patch scripts (comments dated 2026-11-02) add about 21.7 KB.
- Product page `raw/pages/products_hipp_formula_stage_1.html`: 913,268 bytes, 125 scripts = 588 KB (64.4%): Appstle subscription helper 143 KB, `_ALConfig` 108 KB, back-in-stock config 46 KB, a 35 KB inline script, Judge.me data 28 KB. Collection page: 793,251 bytes, 127 scripts = 449 KB (56.6%).
- Crawl (467 HTML pages, 200 OK, `raw/crawl.jsonl`): median 686,619 bytes; products median 779,519 (max 913,269), collections 680,982, blog 530,840, pages 505,926. Even pages with no app UI are above 495 KB.
- Runtime DOM is 2,970-3,296 elements, against the 1,500 guideline.
Recommendation: disable Appstle subscription/loyalty embeds (and any back-in-stock block) on templates that do not use them, or load their config from a cached asset instead of inlining it on every page; move the inline accessibility and drawer scripts into a cached theme asset; replace repeated inline SVGs with a sprite; remove duplicated Judge.me settings. Target HTML under 300 KB. Verify with `curl -s URL | wc -c` and the DOM node count in DevTools.

### 5. Render-blocking CSS and Google Fonts links inside the body - Severity: Medium
Evidence: `raw/home.html` head stylesheets with `media=all` (render-blocking): font-settings.css (line 113), theme.css (280), Judge.me `shopify_v2.css` (697), `gwp.css` (804), Klaviyo `app.css` (806), plus `accelerated-checkout-backwards-compat.css` (515, `media=screen`). Seven `<link rel="stylesheet" href="https://fonts.googleapis.com/css2?...">` tags sit inside page sections (lines 3092, 3532, 4610, 4750, 5063, 5218, 5365), each with its own `preconnect` pair and four different family combinations (Inter, Libre Baskerville, Tenor Sans, Libre Franklin). Measured: 9-11 requests to fonts.googleapis/gstatic, 8-10 font files, 252-287 KB of fonts, on top of the Shopify-hosted fonts.
Recommendation: self-host or consolidate into one preloaded stylesheet in the `<head>` with `display=swap` and only the weights used; inline critical CSS and load app CSS (Judge.me, GWP, Klaviyo) non-blocking or only where the widget exists. Verify with the Lighthouse "Render blocking requests" insight.

### 6. LCP discovery and Early Hints do not target the LCP resource - Severity: Medium
Evidence: Early Hints preload only CSS and preconnect cdn.shopify.com (`raw/home.headers`); the hero video preload is emitted in the body (line 2785). On home the first LCP candidates in all runs are the announcement-bar text `<p>` (11,480 then 12,486 px of area, re-rendered by the ticker JS at 8.8-9.1 s throttled), meaning the hero contributes nothing until the video renders (Finding 1). Product page is fine (see What works).
Recommendation: after fixing the hero, preload the poster/LCP image in `<head>` with `fetchpriority="high"` and avoid re-rendering the announcement bar (ticker JS) before first paint.

### 7. VWO anti-flicker snippet hides the whole body for up to 2 s - Severity: Medium (effect not isolated)
Evidence: `raw/home.html` VWO Async SmartCode: `settings_tolerance=2000`, `hide_element='body'`, `hide_element_style='opacity:0 !important;...'`, inline in `<head>` so `currentScript.async` is false and the hide style is added. Throttled FCP was 2.6-2.8 s versus 0.83 s unthrottled; I did not isolate whether VWO contributed (the hide style was gone at the end of every run).
Recommendation: if no live VWO test needs it, remove the snippet; otherwise reduce tolerance (for example 500 ms) and hide only the tested element, not `body`. Verify by comparing FCP with and without the snippet in the same harness.

### 8. Outdated duplicate jQuery - Severity: Low
Evidence: `https://cdnjs.cloudflare.com/ajax/libs/jquery/3.2.1/jquery.min.js` (`raw/home.html` line 6551) loaded on every page, plus jQuery 3.7.1 from `ajax.googleapis.com` preloaded by the Appstle block (line 9201). jQuery below 3.5.0 has public XSS advisories (CVE-2020-11022/11023).
Recommendation: remove one copy and use 3.7.1 (or drop jQuery where the theme does not need it).

### 9. Minor layout shifts from announcement bar and app banner - Severity: Low
Evidence: lab CLS is only 0.0015-0.0062, from the ticker announcement bar (`DIV.announcement__bar` 390 to 380 px wide, `ticker--unloaded`) and the Tapcart download banner (`DIV.tc-mobile-download-banner-app-icon` 36x57 to 96x96 at about 12.3 s throttled). The Tapcart banner also overlaps the header/hero top on mobile (`screenshots/home-mobile.png`).
Recommendation: reserve space or set fixed dimensions for the banner and ticker; consider dropping the app-download banner on mobile web.

## Not checked / limitations

- Field data (CrUX) and PSI/Lighthouse scores: unavailable. LCP and CLS pass/fail at the 75th percentile is unknown.
- INP: not measured (needs interaction).
- Collection page `/collections/hipp-formula`: no lab run (static analysis only). Product and home runs are 1-2 throttled runs, so no medians.
- Which third-party scripts GTM injects (for example Clarity) was not verified in the GTM container.
- Effect of VWO's body hide on FCP was not isolated.
- Lab environment: datacenter CPU throttled 4x, proxy in the path, per-request emulated latency, 30 s window; not equivalent to Lighthouse's simulated throttling.
