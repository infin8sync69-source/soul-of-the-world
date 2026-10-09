# Threat model

Status: Draft. Owner: security reviewer (unassigned, see [OQ-09](../open-questions.md)). Method: asset and actor analysis with STRIDE-style prompts. Reviewed against: SPEC-001, SPEC-002, the architecture overview.

This is a living document. Every protocol or architecture change MUST be reviewed against it ([REQ-SE-05](../requirements.md)).

## 1. Assets

| ID | Asset | Why it matters |
|---|---|---|
| A1 | Device private keys | Control of identity and signing. Loss or theft is the worst case. |
| A2 | Rotation (root) keys and recovery material | Can replace all other keys. |
| A3 | Identity logs | Integrity of who controls which ID. |
| A4 | Private user content (messages, order details, location) | Privacy and safety. |
| A5 | Public content and listings | Integrity and availability. |
| A6 | Node infrastructure and operator credentials | Availability, federation trust. |
| A7 | Release artifacts and signing keys | Supply chain. |
| A8 | Source repositories and CI | Supply chain, secret exposure. |
| A9 | The assistant's tools and local services | Local code execution and data access. |

## 2. Actors

| Actor | Capabilities |
|---|---|
| Malicious user | Valid identity; abuses protocols, spams, impersonates in the UI. |
| Malicious or compromised node operator | Controls a node: withholds, delays, censors, inspects metadata, serves stale data. |
| Network attacker | Observes or alters traffic; can block endpoints. |
| Thief or device attacker | Physical access to an unlocked or locked device; malware on the device. |
| Supply-chain attacker | Compromises a dependency, build, release channel or repository. |
| Insider | A maintainer or operator acting maliciously or carelessly. |
| State or ISP censor | Blocks domains, gateways, and push services; legal pressure on operators. |

## 3. Trust boundaries

Defined in the [architecture overview](../architecture/overview.md): device, client to node, node to node, local services to web, build to user.

## 4. Threats and mitigations

Status: **Designed** means the mitigation is specified; **Open** means it is not yet; **Built** means code and tests exist in the predecessor reference implementation.

| ID | Threat | Actor | Mitigation | Status |
|---|---|---|---|---|
| T-01 | A server holds or can reconstruct user private keys | Insider, node | Keys are generated on device and never exported. No key escrow API exists. Reviews reject any code that transmits a private key. | Designed |
| T-02 | Stolen device is used to impersonate | Thief | Hardware-backed device keys; biometric gate on sensitive actions; remove the device from another device. | Designed |
| T-03 | Stolen or malicious rotation key takes over the identity | Thief, insider | Offline backup key; override window with priority (REQ-ID-08); social recovery. **Gap:** v0.1 lets any rotation key rotate all. | Open |
| T-04 | Forged operation or event | Network, node | ECDSA signatures, canonical encoding, chain checks, low-S. | Built |
| T-05 | Signature malleability changes an ID or hash | Network | ID excludes the signature; low-S required; canonical decoding. | Built |
| T-06 | Cross-protocol signature reuse | Malicious user, node | Domain-separation prefix. **Gap:** not in v0.1. | Open |
| T-07 | Backdated events from a removed device | Thief | Events from later-removed devices stop verifying. Cost: history needs a policy. | Built (policy open) |
| T-08 | Directory withholds a revocation | Node | Clients consult several directories and take the longest valid chain; anchor directory roots to a public timestamp. | Designed |
| T-09 | Node reads private messages | Node | End-to-end encryption inside event payloads; nodes see only metadata. | Designed |
| T-10 | Metadata analysis by nodes or network | Node, network | Minimise metadata; coarse location; padding and relay options later. Full metadata privacy is a non-goal. | Open |
| T-11 | Node censors or drops a user's data | Node, censor | Home-node list in identity; replicate to multiple nodes; change nodes. | Designed |
| T-12 | Spam and abuse flood a node | Malicious user | Rate limits, per-identity quotas, proof of vouching, operator policy. | Open |
| T-13 | Sybil identities inflate trust | Malicious user | Trust counts only votes tied to completed transactions, per lens; community vouching with locality checks. | Designed |
| T-14 | Local service abused by a web page or local process | Network, malware | Per-launch token; origin checks; scoped file access; no wildcard CORS. | Designed |
| T-15 | The assistant is tricked into harmful actions | Malicious content | The assistant only proposes; deterministic validation; confirmation gate; scoped tools. | Designed |
| T-16 | Malicious update or installer | Supply chain | Signed releases; pinned key; reproducible builds; transparency log later. | Designed |
| T-17 | Compromised dependency | Supply chain | Locked dependencies, review of new dependencies, vulnerability scanning, minimal dependency policy. | Open |
| T-18 | Secrets committed to repositories | Insider | Secret scanning in pre-commit and CI; rotation procedure; no real credentials in examples. | Open |
| T-19 | Push notification provider sees activity | Platform | Wake-only pushes with no content; user chooses the Android push path. | Designed |
| T-20 | Phone-number onboarding links identity to a real-world identifier | Node, platform | Phone verification is an optional attestation with a salted hash, never the identity root. | Designed |
| T-21 | Location tracking | Node, insider | Coarse location by default; precise only during an active ride or delivery; retention limits. | Designed |
| T-22 | Rollback of the identity log by a node | Node | Clients remember the highest seen head and refuse to accept a shorter valid chain. | Designed |
| T-23 | Denial of service through large or malformed input | Network | Size limits and bounded parsing before deep decode; fuzz tests. | Partly built |

## 5. Lessons from the predecessor audit (real incidents)

These happened in the reference repositories and are encoded as rules above and in [lessons learned](../references/lessons-learned.md):

- User private keys were generated on a server and stored in a database, with an unauthenticated endpoint that returned them. (T-01)
- A live database credential was committed and later pushed to a public repository. (T-18)
- Local tool servers had wildcard cross-origin access and could write files anywhere in the home directory. (T-14)
- A custom chain accepted unlimited minting and exposed an unauthenticated signing endpoint. (Out of scope by [ADR-0009](../decisions/0009-no-custom-chain.md).)

## 6. Residual risks accepted for now

- Metadata exposure to nodes (T-10).
- Dependence on Apple for iOS push (T-19).
- Social-layer moderation can be abused to censor (R-06 in the [risk register](../risks.md)).

## 7. Review checklist for a change

1. Does it move or expose a private key? Reject.
2. Does it add a way for a server to act as a user? Reject.
3. Does it add trust in a node for correctness (not only availability)? Justify.
4. Does it add an external dependency? Update the [register](../architecture/sovereignty.md).
5. Does it add a parser? Add size limits and fuzz tests.
6. Does it change signed bytes? Update vectors and mark a protocol break.
