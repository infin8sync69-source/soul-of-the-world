# Risk register

Status: Draft. Likelihood and impact are L (low), M (medium), H (high) [Guessing] until reviewed. Owner is TBD until a team exists ([OQ-03](open-questions.md)).

| ID | Risk | L | I | Mitigation | Trigger to escalate |
|---|---|---|---|---|---|
| R-01 | Key loss with no recovery locks people out permanently | M | H | Mandatory backup prompt; social recovery design; recovery drills in M2 | Any user-reported unrecoverable loss |
| R-02 | Scope: identity, nodes, mobile, desktop, assistant is too much for the team | H | H | Strict milestone order; M1 and M2 deliver value alone; defer federation and economy | Two missed exit criteria in a row |
| R-03 | A protocol flaw is found after deployment | M | H | Spec review before deployment; vectors; version field; independent review; conservative gaps listed | Any failed vector in a second implementation |
| R-04 | Third-party dependency changes or fails (iroh, mobile bindings, model runtimes) | M | M | Dependency register; abstraction at the transport and signer boundary; pin versions; verify claims independently | Release notes break an exit criterion |
| R-05 | Legal and regulatory exposure (payments, KYC, data protection, content liability, app-store policy) | M | H | Counsel before launch ([OQ-04](open-questions.md)); no custody; moderation tooling; operator policies | Before any public release |
| R-06 | Operator moderation conflicts with censorship resistance; or abuse of moderation to censor | M | M | Publish policies; allow node switching; replicate identity logs; avoid claiming absolute resistance | A credible takedown incident |
| R-07 | Nodes are hard to run, so few exist and centralise | M | H | One-binary design, tests on small machines, documentation, at least two independent operators before M5 | Fewer than two independent operators at M4 exit |
| R-08 | Supply-chain compromise of a dependency or release | L | H | Locked dependencies; review; signed releases; reproducible builds | Any advisory on a direct dependency |
| R-09 | Secrets leak again | M | H | Secrets policy; scanners; no real credentials in examples | Any scanner hit on a protected branch |
| R-10 | Documentation drifts from reality | M | M | Docs checked in CI; claims need tests; ADRs | Review finds an unverified claim |
| R-11 | Metadata exposure to nodes reveals social graph and movement | H | M | Minimise metadata; coarse location; clear user messaging; research on private relays | Evidence of misuse |
| R-12 | Network effects: people will not leave existing apps | H | H | Start with a concrete local use (a community, a city) and a clear benefit; do not rely on ideology | No pilot community by end of M4 |
| R-13 | Dependence on a single maintainer's knowledge | H | M | Write things down; handbook; ADRs; second reviewer for specs | Bus-factor review each milestone |
