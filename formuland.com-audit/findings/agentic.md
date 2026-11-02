Score: 62/100

PARTIAL (first pass, 2026-11-02). Final version overwrites this file.

## What works (so far)
- /agents.md, /llms.txt, /llms-full.txt, /.well-known/ucp (UCP 2026-08-25) all return 200 with correct content types (Shopify-generated).
- Shopify WebMCP adapter loads on every page (12 tools); OAuth metadata published at /.well-known/oauth-*.
- Real 404s on unknown paths and unused well-known paths.

## Findings (so far)
- [High] Cloudflare managed challenge (cf-mitigated: challenge, 403 "Verifying your connection...") served to the audit client on every path incl. robots.txt and agents.md after ~25 requests.
- [Medium] WebMCP add_to_cart description tells agents not to ask the user to confirm.
- [Medium] agents.md/llms.txt link the Shop skill at two different hosts (shop.com vs shop.app).
- [Info] No ai-catalog.json, no Content-Signal, no Markdown negotiation measured.

## Not checked / limitations
- Lighthouse Agentic fraction: PSI 429; Agent-UX and Markdown probes pending.
