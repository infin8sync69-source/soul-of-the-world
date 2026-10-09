# Open questions

Status: Living. Each item names who decides and what it blocks. Close an item by writing an ADR or editing the relevant document, then mark it **Resolved** with a link.

| ID | Question | Decider | Blocks | Notes |
|---|---|---|---|---|
| OQ-01 | Which licence? | Owner | Accepting outside contributions | **Resolved** by [ADR-0014](decisions/0014-licensing.md): MIT OR Apache-2.0 for code, CC BY 4.0 for docs. Revisit only before the first external contribution. |
| OQ-02 | Product and repository naming: "Soul of the World", "Bucks", or other? | Owner | Public naming, domain, app-store listings | **Resolved** by [ADR-0015](decisions/0015-naming.md): project Soul of the World, product Bucks, protocol identifiers frozen. |
| OQ-03 | Governance and legal entity: who owns the project, who can merge, who runs infrastructure? | Owner | Funding, hosting, store accounts | **Interim** rules in [ADR-0017](decisions/0017-solo-maintainer-review.md). A legal entity is still open. |
| OQ-04 | Launch market and regulation: India first? Payment, KYC, data-protection and content-liability requirements | Owner with counsel | Release planning (R-05) | [Guessing] The predecessor targets India (UPI, Bengaluru). No legal review has been done. |
| OQ-05 | What is the role of the "Soul" constitution? | Owner | Assistant design, network admission | **Resolved** by [ADR-0016](decisions/0016-soul-is-a-persona-not-a-gate.md): an optional, user-selectable persona; network trust never depends on it. The owner may reopen it. |
| OQ-06 | Policy for history signed by devices that were later removed | Spec working group | SPEC-002 acceptance | Options: re-attestation by a rotation key, removal operations that carry an explicit cutoff, or directory-attested time. |
| OQ-07 | Rotation-key override window, fork resolution, and the source of trusted time | Spec working group | REQ-ID-08, SPEC-001 acceptance | Directory-attested timestamps or external anchoring are candidates. |
| OQ-08 | Who runs the first nodes, and who pays? | Owner | M4 exit (two independent operators) | |
| OQ-09 | Security owner and contact address | Owner | SECURITY.md, M0 exit | **Interim** per [ADR-0017](decisions/0017-solo-maintainer-review.md): the repository owner, through GitHub private vulnerability reporting (the owner must enable it in the repository's security settings). |
| OQ-10 | Retention and deletion in an append-only log (privacy law, user wishes) | Owner with counsel | SPEC-002, export | Likely design: erasure by encrypting payloads and discarding keys; references remain. |
| OQ-11 | Add a domain-separation prefix to every signed object before first deployment? | Spec working group | M1 (vectors regenerate) | **Resolved** by [ADR-0013](decisions/0013-domain-separation.md): yes, for signatures and hashes. |
| OQ-12 | Event type registry design and the first domain schemas | Spec working group | M5 | Start from the predecessor's data model for rides, orders and listings. |
| OQ-13 | iOS push: acceptable privacy trade-off? | Owner | M5 | Apple's service is unavoidable on iOS. The design uses wake-only messages. |
| OQ-14 | Predecessor Go node and Solidity contracts: reuse anything? | Reviewer | ADR-0009 revisit | Not reviewed in the audit that produced this repository. |
