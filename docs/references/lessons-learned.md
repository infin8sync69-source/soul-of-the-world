# Lessons learned

Status: Accepted. Each lesson is something that actually happened in the reference repositories (see [reference-repos](reference-repos.md)). The right-hand column says where this project prevents it.

| ID | What happened | Rule here | Enforced by |
|---|---|---|---|
| L-01 | A server generated users' private keys, stored them, and returned them through an endpoint with no authentication. | Keys never leave the device. | REQ-ID-03, T-01, code review gate |
| L-02 | Live credentials were committed in several repositories, one in a package given to every user, one later pushed to a public repository. | Rotate first, scan always, no real values in examples. | [Secrets policy](../security/secrets-policy.md), REQ-SE-01 |
| L-03 | Documentation claimed Ethereum and MongoDB where neither existed, and "production ready" where core features were stubs. | Claims need tests; stubs fail loudly. | REQ-QA-04, ADR-0001, handbook |
| L-04 | Four unlinked identity schemes in one ecosystem. | One identity, specified once, implemented once. | ADR-0003, ADR-0005 |
| L-05 | The same helper code was copied across several repositories and diverged. | One core library. | ADR-0005 |
| L-06 | Large binaries and archives were committed repeatedly (94 MB three times, desktop archives, prebuilt binaries). | Binaries and models stay out of git. | REQ-QA-05, CI size check |
| L-07 | Broken submodule pointers, machine-specific paths, a symlink to another person's home directory, build junk. | No machine-specific content; CI check for these patterns. | Handbook, CI |
| L-08 | An "abstraction layer" was documented but bypassed by hundreds of direct calls to a vendor backend. | Abstraction at the boundary the code actually uses, with contract tests. | ADR-0005, ADR-0008 |
| L-09 | The default AI provider was a cloud service, and a cloud key shipped inside the mobile app. | Local-first, opt-in remote, no keys in apps. | ADR-0010, REQ-AI-02 |
| L-10 | Local tool servers accepted requests from any web page and could write anywhere in the home directory. | Per-launch token, origin checks, scoped access. | REQ-SE-02, T-14 |
| L-11 | The installer depended on a release pinned only on one laptop; hashes and content identifiers drifted across three places. | Releases pinned on at least two reachable nodes, signed, one automated manifest. | REQ-SE-03, REQ-QA-03 |
| L-12 | Custom cryptography and a custom chain were built without review; a word list could not even load; a "signature" used a secret key as an HMAC key so nobody could verify it. | Vetted libraries only; no custom primitives; independent review of protocols. | Handbook, spec acceptance rule |
| L-13 | No Android tests; the browser repository's CI was an unmodified template that always failed. | CI must run real tests on every change. | REQ-QA-01 |
| L-14 | A reviewed refactor was merged although the browser-extension bundle began importing shared chunks, which content scripts and page-injected scripts cannot load. Tests and type checks did not catch it. | Check the **build output**, not just the source, for packaging-sensitive code. | Handbook, CI build-output checks |
| L-15 | A text hash was used as a network admission rule, tying participation to agreement with one document. | Admission by signatures and community mechanisms. | OQ-05 |
| L-16 | A stub function returned "node started" while doing nothing. | Stubs return an explicit not-implemented error. | Handbook, review |

## How to use this list

In design review, ask which lessons apply. In code review, cite the L-number when you reject something for one of these reasons. Add a new lesson when a new failure of this kind occurs.
