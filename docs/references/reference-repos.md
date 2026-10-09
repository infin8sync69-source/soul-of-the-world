# Reference repositories

Status: Accepted (per [ADR-0002](../decisions/0002-fresh-start-references-read-only.md)). Audited 2026-10-09 by reading code; commits are those audited. All are read-only references. Anything reused is re-reviewed and re-committed with a provenance note naming the source repository and commit.

| Repository | Audited commit | What it is |
|---|---|---|
| `shafeeqduddiyanda/bucks.global` | `1f2dbc6` | The website, installer scripts, a data-sync job and the shipped desktop archive |
| `infin8sync69-source/bucks-Mobile` | `cc4767a` | Android (Kotlin) and iOS (Swift) apps, plus a hosted-backend schema with about 120 database functions |
| `infin8sync69-source/Bucks-browser` | `7510c28` | Tauri 2 desktop shell, abandoned Electron, Qt and Next.js shells, committed build output |
| `infin8sync69-source/Bucks-global` | `f8825e8` | FastAPI and Next.js social app, a forked desktop app, many planning documents |
| `infin8sync69-source/bucks` | `4b04bbe` | C++ blockchain prototype with a wallet and web UI |
| `infin8sync69-source/buckscore` | `b29b721` (main) | The author's desktop workspace: Electron application source, Go node and miner, Solidity contracts, wallet extension, soul corpus |

## What to take, what to leave

### bucks-Mobile (most valuable)
- **Take as reference:** domain model (providers, drivers, rides, orders, requests, votes, communities, jobs); the database rules in the schema as the starting point for service modules; the confirmation gate and biometric policy; the deterministic intent grammar; trust "lenses"; design language; CI patterns (builds, SQL tests); the parity and launch-readiness documents.
- **Leave:** direct coupling to a hosted backend and identity provider, the embedded cloud model key, the demo sign-in code, repeated fallback credentials in workflows.
- **Known defects:** no Android unit tests; the repository abstraction was bypassed by about 280 direct backend calls; push for iOS incomplete; no moderation or privacy policy; regulatory items open.

### Bucks-browser
- **Take:** the Tauri shell concept; managed local content node; the phased plan's signed-event design ("local first, then signed event, then peer-to-peer, then relay").
- **Leave:** the abandoned shells and build output, root scripts for a removed runtime, two 94 MB archives.
- **Known defects:** keys and mnemonic in browser storage in plaintext; no content security policy; a stub identity; a hard-coded cloud API key in the legacy tree; an unverified binary download.

### Bucks-global
- **Take:** UX ideas (feed, messages, QR, recovery flow), and the idea of guardian-based recovery, to be redone with shares made on the client.
- **Leave:** the whole backend and frontend.
- **Known defects:** server-generated and server-stored user private keys; an unauthenticated endpoint that returns them; request-signing schemes that do not match between client and server; read endpoints that trust unauthenticated headers; planning documents that contradict each other and the code.

### bucks (C++ chain)
- **Take:** lessons only (ADR-0009).
- **Known defects:** unvalidated block rewards, duplicate-input acceptance, height-based fork choice, no transaction relay, unauthenticated encryption and signing endpoints, a wallet word list that fails to load.

### buckscore (desktop workspace)
- **Take as reference:** the feature set of the Electron app (agent browser control, app launcher, end-to-end messaging design with post-quantum key exchange, device linking, cluster membership by invitation, signed update design); the soul and persona concept ([OQ-05](../open-questions.md)).
- **Leave:** macOS-only binaries and databases in git, unauthenticated local services, default cloud inference.
- **Known defects:** a live database credential in public history on two branches (rotate first); local tool server with wildcard cross-origin access and broad file write; a shell allow-list bypass; cluster secrets once published; a merged pull request changed content and injected scripts to import shared bundle chunks, which would likely break a browser extension (see [lessons learned](lessons-learned.md), L-14).
- **Not reviewed:** the Go node, miner and Solidity contracts ([OQ-14](../open-questions.md)).

## Provenance rule

When code or data is carried over, its first lines and the commit message state: source repository, source commit, what was changed, and who re-reviewed it. Without that, a reviewer should reject the change.
