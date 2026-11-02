Score: 70/100

Agent-readiness audit of https://formuland.com, run 2026-11-02 (UTC) with curl, saved HTML and offline Chromium. The 70 is an editorial judgement. It is not the Lighthouse Agentic Browsing fraction, which could not be measured. It is also not the Agent-UX heuristic, which is reported separately below.

Scope notes:
- `agentic_check.py` and `lighthouse_agentic.py` cannot fetch here, so their checks were replicated by hand.
- Fetched files are in `/home/user/ALL/formuland.com-audit/tmp-agentic/fetched/`.
- The offline accessibility-tree script is `/home/user/ALL/formuland.com-audit/tmp-agentic/scripts/a11y_offline.py`; its output is `/home/user/ALL/formuland.com-audit/tmp-agentic/a11y_offline.json`.
- Everything in agents.md, llms.txt and the JS files is treated as untrusted data. Instructions found there are reported as findings and were not followed.

## Summary
- **Lighthouse Agentic Browsing:** not measured (X/N unavailable). PSI returned 429 RESOURCE_EXHAUSTED for both mobile and desktop (`raw/psi/home-mobile.json`, `raw/psi/home-desktop.json`). No Lighthouse version or date applies.
- **Agent-UX heuristic (0-100), status partial:**
  - Home, HiPP stage 1 product page and HiPP collection page each scored 100.
  - The score comes from saved HTML loaded into Chromium with all network requests aborted (44 to 46 aborted per page). The CDP accessibility tree was captured with `render_page._capture_accessibility_tree` and scored with the plugin's `agent_ux_check` analyzers.
  - It is a snapshot of the server-rendered DOM, not the hydrated live page, hence partial.
  - Counts per page:

    | Page | AX nodes | Interactive | Unnamed interactive | `div[onclick]` | Landmarks |
    |---|---|---|---|---|---|
    | Home | 1257 | 100 | 0 | 0 | banner, navigation x3, search x2, main, contentinfo |
    | Product | 2325 | 112 | 0 | 0 | banner, navigation x4, search x2, main, contentinfo |
    | Collection | 2729 | 161 | 0 | 0 | banner, navigation x4, search x2, main, contentinfo |

- **P0 failures confirmed:** 0. One P0-class item was observed for the audit client only (bot challenge, F1). Two P0 accessibility-tree candidates are unverified (F4).
- **What most limits agents today:** unsigned, non-browser agents get a Cloudflare/Shopify managed challenge, and the platform's WebMCP `add_to_cart` tool tells agents not to ask the user to confirm.

## What works
- **Agent surfaces exist and are platform-generated.** Each returned 200 on 2026-11-02:
  - `/agents.md`: `text/markdown`, 5,731 B, served by `AgentsMdController`, and listed in `raw/sitemaps/sitemap_agentic_discovery.xml`.
  - `/llms.txt`: 237,834 B.
  - `/llms-full.txt`: 237,435 B.
  - `/.well-known/ucp`: `application/json`, UCP version 2026-08-25, `Link: </.well-known/ucp>; rel="ucp"; version="2026-08-25"` on HTML, agents.md and llms.txt responses.
- **UCP profile content.**
  - Capabilities: checkout, cart, fulfillment, discount, buyer_consent, order, catalog search and lookup, identity_linking, permalink (`https://formuland.com/buy`).
  - Payment handlers: Google Pay, `dev.shopify.card`, Shop Pay.
  - Older profile `/.well-known/ucp/2026-04-08` also returns 200 (4,338 B).
- **OAuth metadata is published** (RFC 8414 and 9728 are published standards).
  - `/.well-known/oauth-authorization-server` and `/.well-known/openid-configuration` are byte-identical (1,549 B).
  - Issuer is `https://shopify.com/authentication/64774930651`; endpoints are on `account.formuland.com`.
  - Scopes include `customer-account-mcp-api:full`, `dev.ucp.shopping.checkout:manage` and `dev.ucp.shopping.catalog.search:read`.
  - `/.well-known/oauth-protected-resource` returns resource `https://formuland.com` (155 B).
- **WebMCP adapter on every saved page.**
  - All 18 pages in `raw/pages/*.html`, plus `raw/home.html`, contain the inline loader (`shopify:webmcp_adapter_loaded`).
  - It feature-detects `document.modelContext || navigator.modelContext` and loads `//formuland.com/cdn/shopifycloud/storefront/assets/storefront/webmcp-23c12384.js` with SRI.
  - The script registers 12 imperative tools (see F2).
  - No declarative `toolname` or `tooldescription` attributes exist on any form.
- **Commerce controls on the product page use native, labelled elements** (`raw/pages/products_hipp_formula_stage_1.html`, offline Chromium DOM check).
  - Add to cart is a native `<button type=submit name=add>` inside `<form action="/cart/add">`, with the text "Add to cart — $49.99".
  - The price appears as server-rendered text (`$49.99/can`, "Save 11%", "+ $24.99 shipping").
  - The subscription picker is a `<select aria-label="Delivery frequency">` with 8 options.
  - The 3 search inputs are `type=search` with `aria-label`, inside `role=search` forms, and appear in the tree as `searchbox`.
  - The page has 0 duplicate ids and 0 positive tabindex values.
  - It has 3 JSON-LD blocks (home has 1).
  - Variant radios: 0 (this single-product page exposes no option fieldset).
- **404 behaviour is correct.**
  - Unknown path `/this-page-does-not-exist-xyz123` returns a real 404.
  - All probed well-known paths return a real 404 (HTML, 4,264 B), not a catch-all 200.
  - The Lighthouse `llms-txt` and `ard-schema` soft-404 hazard therefore does not apply.
- **robots.txt is reachable and has deliberate AI-retrieval groups** (see Access policy).

## Findings

### F1. Managed bot challenge served to the audit client (High; skill P0 class "WAF and CAPTCHA on content")
- **Evidence (this session).**
  - After about 25 requests in about 2 minutes, with the honest UA `Mozilla/5.0 (compatible; ClaudeSEO-Audit/2.5; +https://claude-seo.md)` at at most about 1.6 requests/second, every path returned HTTP 403 with `cf-mitigated: challenge` and the body "Verifying your connection...". This included `/`, `/collections/hipp-formula`, `/products/hipp-formula-stage-1`, `/agents.md` and `/robots.txt`.
  - The 403s started at 18:59 UTC; the earlier fetches at 18:58 UTC had returned 200.
- **Evidence (earlier crawl).** `raw/crawl.jsonl` has 468 pages at 200 and 32 at HTTP 429 with title "Verifying your connection...". The first 10 listed are all `/blogs/parenting-tips/` articles. `raw/crawl-retry.jsonl` shows the same 32 still challenged and 10 other pages returning 200.
- **Caveat.** This is behaviour toward an unverified, proxied, non-browser client. It is not proof that real ChatGPT, Claude or Gemini agents are blocked. `--ua-matrix` was not run because it was not authorised.
- **Consequence.** Markdown negotiation and the live accessibility tree could not be measured (see Not checked).
- **Recommendation.**
  - Check Shopify/Cloudflare analytics for challenged agent traffic.
  - Ask Shopify Support whether verified (Web Bot Auth signed) agents and verified search bots are exempt, since this edge is platform-managed.
  - Keep the challenge off informational pages (policies, products, articles) where Shopify allows it.
  - Verify by fetching `/robots.txt`, `/llms.txt` and one product URL from a signed or allow-listed agent and getting 200 without `cf-mitigated`.
- I did not attempt to circumvent the challenge. One retest with a browser user agent was declined by the permission layer as bot-check circumvention and was not retried.

### F2. WebMCP `add_to_cart` description tells agents to skip user confirmation (Medium; skill P1 tool-safety)
- **Evidence.**
  - Source: `https://cdn.shopify.com/shopifycloud/storefront/assets/storefront/webmcp-23c12384.js`, 86,494 B. The same file is referenced from formuland.com with SRI `sha256-r8Pww6V2udgtSdaVbcRtdZ4Kg+gXP7aLPZOtQSJPrbM=`.
  - Quoted text: "That request IS the authorization: do NOT ask the user to confirm before calling, and do NOT stop after search_catalog or get_product merely to report the variant you found".
  - `references/webmcp.md` rule 8 treats such wording as a P1 safety finding.
- **Tool list (12 imperative tools):** add_to_cart, browse_store, cancel_cart, get_cart, get_product, get_ucp_profile, manage_orders, proceed_to_checkout, search_catalog, search_shop_policies_and_faqs, show_variant, update_cart_lines.
- **Annotations.**
  - Only `readOnlyHint` and `untrustedContentHint` appear in the extracted annotations.
  - `readOnlyHint:true` on 4 tools: get_cart, get_ucp_profile, search_catalog, search_shop_policies_and_faqs.
  - `untrustedContentHint:true` on 9 tools.
  - No `consequentialHint` was seen in the extracted annotations.
  - `cancel_cart` ("Remove all items from the cart in one step") and `proceed_to_checkout` carry no `untrustedContentHint`.
- **Mitigating context.** Cart adds are reversible and involve no payment. agents.md says checkout requires buyer review and approval. That checkout step was not tested.
- **Why it matters here.** This is infant formula, where a wrong stage or product matters, and "buy" is listed as a trigger phrase.
- **Recommendation.**
  - The file is Shopify-hosted, so the merchant cannot edit it; raise it with Shopify and record the exposure.
  - Confirm the checkout approval step in a supervised session.
  - Do not add a second set of tools for the same actions.
  - Verify by re-reading the adapter description after Shopify updates it, or by running Lighthouse `webmcp-registered-tools` in a WebMCP-enabled browser.

### F3. Instructions to AI agents in platform files, with a conflicting skill URL (Medium)
- **Evidence.**
  - `/agents.md` and `/llms.txt` (section "Agent Instructions") both tell agents to "Use the Shop skill (https://shop.com/SKILL.md) ... Read the skill before your first call". They also tell agents to connect the user's Shop account.
  - The "Platform" section of agents.md links `https://shop.app/SKILL.md` instead.
  - So `shop.com` and `shop.app` are given for the same skill. I did not fetch either, so which is intended is not verified.
  - The files also instruct agents to call `tools/list` on the MCP endpoint, to use Web Bot Auth, to use only the WebMCP tools ("Do not work around the tools by scripting checkout forms or page controls"), and to back off on 429.
  - The files carry `etag ...AgentsMdController` and `...LlmsTxtController`, meaning Shopify generates them.
  - None of these instructions were followed. I did not call the MCP endpoint or fetch either skill URL.
- **Why it matters.** An agent that reads a skill from a mistyped or unverified host is a prompt-injection and supply-chain risk. These instructions are written for agents and are not visible to buyers.
- **Recommendation.**
  - Report the `shop.com`/`shop.app` mismatch to Shopify.
  - Check whether the store can override or disable these files; not verified.
  - Verify by re-fetching `/agents.md` and confirming that all skill links use one host.

### F4. Two accessibility-tree candidates to verify (Medium; skill P0 class, not confirmed)
- **Evidence.** On both home and product (offline DOM check): 102 focusable `a`, `button`, `input` or `select` elements sit inside `[aria-hidden=true]` subtrees. This is a selector count that does not check visibility. The axe rule `aria-hidden-focus` is one of the 33 behind Lighthouse `agent-accessibility-tree`, and the local heuristic does not test it.
- **Second candidate.** The product page's `input[name=quantity]` has no `aria-label` and no `label[for]` in the saved DOM. It could still be named by a wrapping label or a visible text node. The plugin's unnamed-interactive count does not include the `spinbutton` role.
- **Neither is confirmed** against live, hydrated pages.
- **Recommendation.**
  - Run Lighthouse Agentic Browsing locally or with an API key. Alternatively run `axe` rules `aria-hidden-focus` and `label` in DevTools on `/` and `/products/hipp-formula-stage-1`.
  - If confirmed, hide off-canvas drawers and menus with `inert`, `hidden` or `display:none` when closed, and give the quantity input `aria-label="Quantity"`.
  - Verify by rerunning the audit and getting no `aria-hidden-focus` or `label` violation.

### F5. llms.txt is a 237 KB catalogue, not a concise index, and is truncated (Low)
- **Evidence.**
  - `/llms.txt` has 1,205 lines and 447 Markdown links, with full product and collection descriptions inline. `/llms-full.txt` is nearly identical (237,435 B), differing only in line formatting.
  - It lists 200 of the 316 products in `raw/sitemaps/sitemap_products_1.xml` (116 missing, including Aptamil entries), 73 of 74 collections, 25 of 32 pages and all 142 blog URLs.
  - It includes the "Delivery Guarantee" add-on product at 1.00 USD.
- **Passes the Lighthouse rule.** The file has an H1 (`# Formuland`), Markdown links and 50+ characters. This is derived from the rules and was not run.
- **Recommendation.**
  - Do not hand-edit it, because it is platform-generated.
  - Treat it as incomplete and rely on `/sitemap.xml` for full coverage.
  - Re-check after Shopify updates.

### F6. robots.txt: no Content-Signal, and the AI group drops the `*` rules (Low)
- **Evidence (`raw/robots.txt`).**
  - No `Content-Signal` line and no `Agentmap:` line.
  - One shared group names `ChatGPT-User`, `OAI-SearchBot`, `PerplexityBot`, `Perplexity-User`, `Claude-User` and `Claude-SearchBot`. It allows `/collections/`, `/products/`, `/blogs/`, `/pages/` and the policy pages, and disallows `/cart`, `/checkout`, `/account`, `/search` and `/*?sort_by=`.
  - Under RFC 9309 group selection, those six bots ignore the `*` group. The `*` rules that do not apply to them include:
    - `Disallow: /collections/*sort_by*`
    - `Disallow: /collections/*+*`
    - `Disallow: /collections/*filter*&*filter*`
    - `Disallow: /*?*preview_theme_id=*`
  - `/*?sort_by=` blocks only a first-parameter `sort_by`.
  - The group mixes search tokens and user-triggered tokens.
- **Recommendation.**
  - Decide the training policy deliberately; this is a business decision and I do not recommend `ai-train=no` or `yes`.
  - Repeat the facet and sort rules in the AI group if faceted URLs should stay out of AI indexes.
  - If a signal is wanted, add `Content-Signal` in each named group (Cloudflare policy, not confirmed to affect Google).
  - Verify with a robots parser per user-agent token.

### F7. WebMCP origin trial token expires 2026-11-17 (Info)
- **Evidence.**
  - Shopify injects 5 third-party origin-trial tokens, scoped to `https://cdn.shopify.com:443`, from `https://cdn.shopify.com/shopifycloud/storefront/assets/storefront/origin_trials-318ab40a.js`. The decoded WebMCP token expires 2026-11-17, 15 days after this audit.
  - The adapter loads only when the browser exposes `modelContext`.
  - Not verified: whether a token scoped to `cdn.shopify.com` covers the adapter served from the `formuland.com` path.
- **Recommendation.** Nothing to change on the store. After 2026-11-17, check whether ordinary Chrome still exposes the tools. ChatGPT desktop does not need a token (vendor matrix, checked 2026-09-23).

### F8. UCP endpoint host mismatch and an empty field (Low/Info; depth belongs to seo-ecommerce)
- **Evidence.**
  - agents.md says the MCP endpoint is `https://formuland.com/api/ucp/mcp`.
  - `/.well-known/ucp` advertises `https://formuland.myshopify.com/api/ucp/mcp`, and its `supported_versions` URLs are also on `formuland.myshopify.com`.
  - The Google Pay handler has `merchant_info.auth_jwt: ""`.
  - Reachability of either endpoint was not tested. `ucp_check.py` was not run.
- **Recommendation.** Run `ucp_check.py` via `seo-ecommerce`. Ask Shopify whether the empty `auth_jwt` is expected.

### F9. Duplicate "Add to cart" controls and a heavy 404 page (Low)
- **Evidence.**
  - The product page contains two controls named "Add to cart": a form submit button and a `type=button`. Both appear in the saved HTML, and the empty-text buttons all resolved to named nodes in the AX tree.
  - A top-level unknown path returns a 404 of 490,121 B, against 4,264 B for a well-known 404.
- **Recommendation.** Give the second control a distinct accessible name, and lighten the 404 template.

### F10. Opportunities (Info, not defects)
- No `ai-catalog.json` (`/.well-known/ai-catalog.json` is a real 404). It would add a counted Lighthouse audit (`ard-schema`) only if the store has agent resources to list.
- No MCP Server Card, API catalog, A2A card or `ai-plugin.json`; all return a real 404.
- No `Link: rel="ai-catalog"` or `rel="alternate"; type="text/markdown"` on the home response. The home response does carry `Vary: Accept`, which is consistent with, but not proof of, negotiation.
- No declarative WebMCP form attributes. P3, and adding them partially could create a failing schema audit; annotate fully or not at all.

## Lighthouse Agentic Browsing
Not measured, so there is no X/N, no version and no date.

Rule-derived expectations, not results:
- `llms-txt` should pass (H1, link, over 50 characters).
- `ard-schema` is N/A, because no catalog is signalled and `/.well-known/ai-catalog.json` returns 404.
- `webmcp-registered-tools` is informative and never counted.
- `webmcp-form-coverage` is informative while forms lack `toolname`.
- `webmcp-schema-validity`, `agent-accessibility-tree` and `cumulative-layout-shift` are not measured.

Paths that add a counted audit, stated as options:
- A valid `ai-catalog.json` adds `ard-schema`.
- Annotating every form adds `webmcp-form-coverage`.

To get the real fraction, run `npx lighthouse@latest <url> --only-categories=agentic-browsing --output=json` on a machine with direct network access, or use a Google API key with PSI, then `lighthouse_agentic.py --from-json`.

## Access policy (robots.txt, `raw/robots.txt`)
- **Training (GPTBot, ClaudeBot, Google-Extended, CCBot and others):** no named group, so the `*` group governs them. It has `Allow: /` plus the Shopify disallows and no training opt-out. No Content-Signal. This is a policy gap to decide, not a defect.
- **Search (OAI-SearchBot, Claude-SearchBot, PerplexityBot):** named in the shared AI group. Allowed on `/collections/`, `/products/`, `/blogs/`, `/pages/`. `/cart`, `/checkout`, `/account`, `/search` and `/*?sort_by=` are disallowed.
- **User-triggered (ChatGPT-User, Claude-User, Perplexity-User):** same shared group. Per `access-policy.md`, ChatGPT-User "may not apply" and Perplexity-User is generally ignored, so robots.txt is a preference only. `Google-Agent` is not named and generally ignores robots.txt.
- **Private paths** (`/cart`, `/checkout`, `/account`) are listed in robots.txt, but robots.txt is not access control. Checkout and accounts are protected by Shopify sessions (not tested).
- **WAF:** see F1. The user-agent matrix was not run (not authorised).

## Standards status (dated)
Dates come from `references/vendor-matrix.md`, last full check 2026-09-23, 40 days before the audit and inside the 60-day window.
- **WebMCP:** W3C Web Machine Learning CG draft. Chrome origin trial M149 to M156. The decoded Shopify token expires 2026-11-17, and the matrix lists the same date as secondary-sourced. Safari opposes and Firefox is neutral. No Gemini or Claude consumption is documented. ChatGPT desktop calls imperative tools.
- **Content-Signal:** Cloudflare CC0 policy (2025-09-24). The IETF individual draft expired 2026-04-04. No Google statement was found. Absent here.
- **ai-catalog.json (ARD 1.0):** proposal. Absent here.
- **MCP Server Card:** SEP-2127 is open and unmerged (as of 2026-09-12). Absent here.
- **Web Bot Auth:** `draft-ietf-webbotauth-httpsig-protocol-00` (2026-09-01). agents.md tells agents to use it. The store publishes no key directory, which is normal for site owners.
- **UCP:** profile version 2026-08-25, presence flagged only (P3).
- **OAuth metadata (RFC 8414, 9728):** published standards.

## Not checked / limitations
- **Lighthouse Agentic X/N:** PSI quota exhausted (429).
- **Live accessibility tree:** not captured. The offline snapshot was taken from the saved HTML, with every network request aborted and no theme or app JS running. Cart drawer state, predictive search and hydrated variant selectors were not exercised.
- **Variant selectors on multi-option products:** only HiPP Stage 1 was inspected.
- **Markdown delivery:** `Accept: text/markdown` and `.md` probes on the product, home and collection URLs all hit the challenge (403). The result is undetermined. Only `/agents.md` and `/llms.txt` are served as `text/markdown`; `/llms.txt.md` was a 404 before the challenge began.
- **CLS and other lab metrics:** not measured by this agent.
- **WebMCP tool registration:** not observed in a live WebMCP-capable browser. The tool list comes from static reading of the adapter file.
- **Not called:** the MCP endpoint (`tools/list`) was deliberately not called, because it is an instruction from agents.md. No cart, checkout or account action was performed. The `shop.com` and `shop.app` URLs were not fetched.
- **`--ua-matrix` and WAF logs:** not run.
- **Commerce protocol depth:** `ucp_check.py` was not run.
