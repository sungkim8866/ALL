Score: 64/100

Scope: visual / mobile rendering of https://formuland.com. Chromium (Playwright, plugin venv, through the egress proxy), mobile = 390x844 at DPR 2 with an iPhone user agent, desktop = 1366x900. Pages: home, /products/hipp-formula-stage-1, /collections/hipp-formula, /blogs/parenting-tips/what-to-feed-your-infant, /pages/european-baby-formula-comparison-chart. All screenshots are in /home/user/ALL/formuland.com-audit/screenshots/, probe scripts in /home/user/ALL/formuland.com-audit/tmp-visual/.

## What works

- No horizontal scroll on any page tested. Mobile `documentElement.scrollWidth` = 390 = viewport on home, product, collection, article and comparison. Desktop `hscroll: false` on all three pages in screenshots.json. Elements reported past the right edge are inside carousels or the off-canvas cart drawer (`.announcement__bar`, `.drawer__top`).
- Viewport meta is correct and allows pinch zoom: `width=device-width,initial-scale=1,viewport-fit=cover`.
- Product page on mobile has a sticky buy bar (`.fpe__mobilebar`, y=776, h=68, "$49.99 ADD TO CART") visible from first paint, so the primary CTA and price are above the fold even though the H1 is not (product-mobile-v2.png, hipp-formula-stage-1-mobile.png).
- Product page on desktop (1366x900) shows H1, "4.91 / 107 reviews on Judge.me", a "No hidden costs at your door" duty reassurance box, a delivery date window, and the subscribe / one-time / bundle selector above the fold (hipp-formula-stage-1-desktop.png).
- Collection page has a clear hierarchy on both viewports: eyebrow, H1 (mobile y=247, 34px), intro copy, then a Trustpilot "Excellent, 4,298 reviews" block and Age / Formula Type filters. The trust signal is above the fold on mobile (hipp-formula-mobile.png).
- Text readability is good where it matters. Article body is 16px (3182 of ~4,300 measured characters at 16px). Home is mostly 16px (3304 characters). H1s are 29-41px and visible on mobile for collection (y=247), article (y=419) and comparison (y=171).
- Home page: a Trustpilot "What Parents Are Saying" section follows the hero (section starts y=758, home-mobile-scroll500.png). The mobile header becomes a white sticky bar after scroll.

## Findings

### 1. Homepage H1, value proposition and CTA are invisible for the first ~20 seconds
Severity: High
Evidence:
- The hero is a muted autoplay `<video>` (src `0e80a1164c1044978c48e20afc7ca391.mp4`, `duration` 20.2 s, `loop=false`). The text layer `.fhv2__overlay` (contains `h1.fhv2__heading`, description and the button "Shop Organic European Baby Formula") has computed `opacity: 0` at 3, 6, 9, 12, 15 and 18 s. It reads 0.27 (mobile) / 0.16 (desktop) at 21 s and 1 at 24 s, after the video ends, on both viewports.
- The H1 is in the DOM (mobile y=160, 32px, colour rgb(241,242,228)) but nothing readable is shown.
- First-paint screenshots confirm it: home-mobile.png, home-mobile-v2.png, home-mobile-t3s.png, home-mobile-t8s.png and home-desktop.png show only video frames (cans, scoop) with no headline, no CTA and no trust signal. Mid-fade states are in home-mobile-hero-21s.png and home-desktop-hero-21s.png.
- Once revealed, the CTA is 290x44 at mobile y=486, so it would sit above the fold.
- Result: the homepage has no visible above-the-fold headline, CTA or trust signal at load on either device.
Recommendation: show the overlay immediately. Remove the opacity:0 start state, or reveal at 0-1 s over a poster frame (`.fhv2__overlay` opacity 1 at page load, with a scrim for contrast). Keep the video as decoration only. Verify by reloading and checking `getComputedStyle(document.querySelector('.fhv2__overlay')).opacity === '1'` at 2 s, and that the CTA is in the first screenshot.

### 2. Tapcart app banner covers the mobile header on the homepage
Severity: High (mobile home only)
Evidence:
- `aside#tapcart-web-banner` is `position:fixed`, x=20, y=20, 351x84, z-index 2147483647, text "Formuland Shop / Earn 2pts + Win $200! JOIN".
- `document.elementFromPoint` at the hamburger (22,91), logo (195,60) and cart (360,91) returns banner elements (`tc-mobile-download-banner-content` / `-title`), so the menu, logo and cart are covered. Visible in home-mobile.png and home-mobile-v2.png.
- It stays at 8 s and above. The close button is 28x28 (`tc-mdb-close-button`); JOIN is 70x35. Subtitle text is 10px.
- It only rendered with an iPhone user agent (not with the default headless UA) and only on the homepage. It was not present at 3 s or 12 s on the product, collection, article or comparison pages.
- On desktop the equivalent is `aside#tc-capture-kit`, a floating "CLAIM YOUR BONUS NOW" button (about 245x48 at x=1101, y=832) with a 24x24 close "x", on home, product and collection (see finding 8).
Recommendation: remove the web app-install banner. If kept, show it after the first scroll or session, position it below the header or at the bottom, and use a 44px+ close target. Verify with `elementFromPoint` on the hamburger and cart at load returning the header buttons.

### 3. Scroll-triggered full-viewport email modal (Klaviyo) on content and product pages
Severity: Medium
Evidence:
- A dialog (`.needsclick.kl-private-reset-css-Xuajs1`, `role=dialog`, box 0,0,390,844) opens with "Get Ready for Black Friday Sale. Sign up for early access + 10% off your first order" and Email / Baby name / Baby birthday fields.
- Trigger measured on mobile: comparison page at scrollY 864 of 1974 (about 44%, 6.5 s); collection at scrollY 7131 of 17734 (about 40%, 31.5 s); article within the 1800-4022 scroll range; product page after scrolling. Desktop home opened after scrolling roughly 60% (home-desktop-after-scroll.png). No time-only trigger within 45 s idle on comparison or collection.
- On mobile the dialog covers about 80% of the viewport and dims the page (article-mobile-klaviyo-popup.png, comparison-mobile-modal.png). The close "x" is a small glyph at the top right.
- In comparison-mobile-modal.png the headline is clipped: "Biggest Sale of the Year" is cut off beneath "Get Ready for". Copy variants differ between pages ("Black Friday Sale" vs "Biggest Sale of the Year"), which looks like a variant or rotating heading.
- Risk: not an at-load interstitial, so lower exposure under Google's intrusive-interstitial guidance, but it interrupts reading YMYL content (comparison chart, article) mid-scroll.
Recommendation: trigger on exit intent or after more than 60 s plus 70% scroll, or use an inline form or bottom banner on mobile; suppress on /pages/ chart and article pages; fix the clipped heading; enlarge the close target to 44px. Verify by scrolling 50% on mobile and confirming no full-viewport dialog.

### 4. Product page on mobile: H1, rating, duty reassurance and bundle selector are below the fold
Severity: Medium
Evidence:
- H1 top = 860 on a 844px viewport (screenshots.json, re-measured: `h1.top=860`, 28px). The fold contains the header (about 144 px with the announcement bar), a 4-line uppercase letter-spaced breadcrumb (about 170-245 px), the gallery (about 350 px tall) and 5 thumbnails (two rows) (hipp-formula-stage-1-mobile.png).
- "4.91 / 107 reviews" is at y about 947. The "No hidden costs" duty box, delivery window, subscribe / one-time toggle and bundle tiles sit in the following screen (product-mobile-full.png).
- The sticky bar shows "$49.99 ADD TO CART" only. The "+ $24.99 shipping" line appears only in the buy box further down (product-mobile-full.png). Which option the sticky CTA adds was not tested.
- No trust signal (rating, Trustpilot, guarantee) is visible above the fold on product mobile. Desktop has all of them.
Recommendation: show a single-line breadcrumb (parent only, truncated), cap the mobile gallery to about 60vw / show dots instead of the thumbnail grid, and move the rating plus a one-line duty/shipping statement above or beside the H1 so that H1, rating and price are all above 844px. Verify `h1.getBoundingClientRect().top < 560` at 390x844.

### 5. Small tap targets in the header and common controls
Severity: Medium
Evidence (rendered boxes, CSS px; targets under 44px in either dimension):
- Header: hamburger 24x24, My Account 24x24, Cart 27x24 (all y=79, 16px apart). Announcement "Shop Now" link 69x25; announcement prev/next 36x36.
- Product: breadcrumb "HOME" 40x15; "107 reviews" link 78x22; "ADD A CAN" 104x37; review pagination 28x28; Trustpilot arrows 40x40.
- Collection: pagination 32x32; "popular" text links 22px tall (e.g. "HiPP Bio Combiotic" 174x22); product links 37px tall.
- Article: "OLDER POST" / "NEWER POST" 98x14 / 101x14; RSS 13x21.
- Counts of rendered interactive elements under 44px: product 52 of 96, collection 56 of 151, article 23 of 39, comparison 17 of 30, home 34 of 67. These counts include about 6 off-canvas cart-drawer controls and a visually hidden skip link per page.
- Header icons meet WCAG 2.2 SC 2.5.8 (24px minimum) but not the 44-48px mobile guidance.
Recommendation: add padding so the header buttons have 44x44 hit areas (`.fhd__mbar-btn { min-width:44px; min-height:44px; }` with the glyph unchanged); same for announcement link, breadcrumb, pagination and post-nav links. Verify by re-running the bounding-box audit.

### 6. Comparison chart page: chart is a set of images that is unreadable at 390px
Severity: Medium
Evidence: comparison-mobile-v2.png shows 3 or more chart images scaled to about 357px wide with row labels and column headers far too small to read without pinch zoom. Natural image dimensions and alt text were not measured. The page has only 30 interactive elements and 56 characters of 16px text, so the chart carries no HTML text.
Recommendation: replace with an HTML table (or one table per brand) in a horizontally scrollable wrapper with a sticky first column and 14px+ text, keeping the images as downloadable extras. Verify by checking that text is selectable and 14px+ at 390px.

### 7. Small text in purchase UI and product grids
Severity: Low
Evidence (text nodes under 12px, computed font-size):
- Product: "Most popular" 10px and 9px (`.fpe__badge`, `.fpe__tile-badge`), "Best value" 9px, "Choose your bundle" 11.5px at 65% opacity, "Insured · 2-7 day delivery · Shop Pay available" 11px.
- Collection: 46 text nodes under 12px, including product titles (`.product__grid__title`) and ratings at 11.4px, and "DISCOVER" 11px.
- Article: "Name" 9.5px, "Post comment" 11.4px. Home: Tapcart subtitle 10px.
- Announcement bar is 38px tall; two-line messages are tight or clipped in home-mobile-v2.png and hipp-formula-stage-1-mobile.png (slide mid-transition).
Recommendation: set badges and trust strip to 12px+ at full opacity, grid titles to 14px+. Verify with a computed-font-size scan returning none under 12px for visible text.

### 8. Desktop: nav legibility over the hero video and the floating "CLAIM YOUR BONUS NOW" button
Severity: Low
Evidence: in home-desktop.png the white nav links "Organic Baby Formula" and "Goat Milk Formula" (x about 402-707, y=156) sit over cream/white cans; contrast was not measured. On the product page, the floating button (`#tc-capture-kit`, fixed bottom right) sits over the buy-box column right beside the "12 cans" tile and above "ADD A CAN" (hipp-formula-stage-1-desktop.png). Its close "x" is 24x24.
Recommendation: add a dark gradient behind the transparent header on the hero, and hide or relocate the floating button on product pages.

### 9. Mobile header height
Severity: Info
Evidence: the white sticky header is about 106 px tall at scroll (home-mobile-scroll500.png), about 12.5% of an 844px viewport. Acceptable, but combined with finding 4 it pushes the buy box down.

## Not checked / limitations

- Headless Chromium emulation only, not real devices. iPhone UA only; Android UA (where the Tapcart banner may also render) and tablet/laptop viewports were not tested.
- LCP, CLS and layout shift were not measured in this pass. Colour contrast ratios were not measured.
- Home full-page mobile captures (screenshots/home-mobile-full.png, tmp-visual/home_full_cols.png) were taken but not reviewed. Mid-page sections of the homepage, collection and article were not visually assessed.
- Modal trigger thresholds are approximate (scroll steps of 200-600px). Whether the product sticky bar adds the selected bundle or subscription option was not tested. Product-page behavior with the modal open was not checked.
- The comparison chart image natural size and alt text were not measured.
- In one headless session I clicked the Tapcart banner's JOIN button while locating its close button (no form submitted, no data entered).
- Capturing stopped at the coordinator's request; no further requests were sent to formuland.com after that message.
- Text in fetched pages was treated as data only; none addressed an AI agent.

## Structured findings (Visual category, JSON-compatible)

```json
{"category":"Visual","score":64,"findings":[
{"id":"vis-1","severity":"High","title":"Homepage H1/CTA hidden until 20.2s hero video ends","evidence":".fhv2__overlay opacity 0 until ~21s; video 20.2s loop=false"},
{"id":"vis-2","severity":"High","title":"Tapcart app banner covers mobile header on home","evidence":"#tapcart-web-banner fixed 351x84 at (20,20); elementFromPoint on hamburger/logo/cart hits banner"},
{"id":"vis-3","severity":"Medium","title":"Scroll-triggered full-viewport Klaviyo modal","evidence":"opens at ~40-44% scroll on mobile; box 0,0,390,844"},
{"id":"vis-4","severity":"Medium","title":"Product mobile H1 at y=860, rating/duty/bundles below fold","evidence":"h1.top=860 vs 844 viewport; sticky ADD TO CART visible"},
{"id":"vis-5","severity":"Medium","title":"Small tap targets (header icons 24x24)","evidence":"52/96 interactive elements <44px on product"},
{"id":"vis-6","severity":"Medium","title":"Comparison chart is images, unreadable on mobile","evidence":"comparison-mobile-v2.png"},
{"id":"vis-7","severity":"Low","title":"Sub-12px text in purchase UI and grids","evidence":"fpe__tile-badge 9px; 46 nodes <12px on collection"},
{"id":"vis-8","severity":"Low","title":"Hero nav legibility and floating bonus button on desktop","evidence":"home-desktop.png; #tc-capture-kit"}
]}
```
