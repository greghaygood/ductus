---
section: "Acquisition"
---

# Fetch-archive-reads-its-proxy-once

## Context

`fetch-archive` builds a new `reqwest` client on every call and never turns off reqwest's system proxy. Each build therefore reads `HTTP_PROXY`, `HTTPS_PROXY`, `ALL_PROXY` and `NO_PROXY` (and their lowercase forms), plus `REQUEST_METHOD`, from the environment, through `hyper-util`'s proxy matcher. 062's review found two consequences on 2026-09-29:

- The long-lived MCP server re-reads those variables on every fetch, which `CFG-ENV-001` forbids (read once at startup and cache). 062 moved every variable the runtime itself reads to a one-time capture in `main`, including this primitive's `DUCTUS_FETCH_ALLOW_INSECURE_HOSTS`. The proxy variables are read by the dependency, so 062's review waived them there and routed them here.
- With a proxy set, the connection is tunnelled through the proxy, which resolves the target itself. The `resolve_to_addrs` pin that `validate_fetch_url` sets — the guard against DNS rebinding between screening an address and connecting to it — then no longer holds.

## Behavior

`fetch-archive` reads its proxy configuration once, at startup, like every other variable the runtime reads, and `docs/runtime.md`'s inventory lists it. The SSRF screen's guarantee is stated for the proxied case: either it still holds, or the operator's proxy is the documented trust boundary.

## Edge Cases

- An adopter behind a corporate proxy must still be able to fetch. Turning proxies off (`.no_proxy()`) would stop the per-fetch reads and restore the pin, but it would break acquisition for them.
- With `REQUEST_METHOD` set (a CGI environment), the matcher ignores `HTTP_PROXY`. A captured configuration has to keep that guard.

## Open Questions

- Should fetch-archive honor proxies from a startup capture (an explicit reqwest proxy configuration built from the variables read once in main), or turn system proxies off?
- When a fetch goes through a proxy that resolves the host itself, how does the SSRF screen hold — or is the operator's proxy the documented trust boundary?

## Resolved Questions

*None yet.*
