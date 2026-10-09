# Open questions

Status: Living. Each item names who decides and what it blocks. Close an item by writing an ADR or editing the relevant document, then mark it **Resolved** with a link.

| ID | Question | Decider | Blocks | Notes |
|---|---|---|---|---|
| OQ-01 | Which licence? | Owner | Accepting outside contributions | The predecessor website repository used the MIT licence. Options: MIT or Apache-2.0 for code, CC BY 4.0 for documentation, and a patent grant consideration for protocols. No licence file exists yet, so all rights are reserved by default. |
| OQ-02 | Product and repository naming: "Soul of the World", "Bucks", or other? | Owner | Public naming, domain, app-store listings | The identifiers `did:bucks:` and `bucks.` event kinds are in the draft specs and would change with the name. |
| OQ-03 | Governance and legal entity: who owns the project, who can merge, who runs infrastructure? | Owner | Funding, hosting, store accounts | Needed before node operators or contributors are invited. |
| OQ-04 | Launch market and regulation: India first? Payment, KYC, data-protection and content-liability requirements | Owner with counsel | Release planning (R-05) | [Guessing] The predecessor targets India (UPI, Bengaluru). No legal review has been done. |
| OQ-05 | What is the role of the "Soul" constitution? | Owner | Assistant design, network admission | The predecessor used a text derived from a religious source as a prefix to model prompts and as a hash that peers must match to be trusted. Proposal in this project: an optional, user-selectable persona layer; network trust never depends on it. Needs the owner's explicit decision and care for the people who hold it dear. |
| OQ-06 | Policy for history signed by devices that were later removed | Spec working group | SPEC-002 acceptance | Options: re-attestation by a rotation key, removal operations that carry an explicit cutoff, or directory-attested time. |
| OQ-07 | Rotation-key override window, fork resolution, and the source of trusted time | Spec working group | REQ-ID-08, SPEC-001 acceptance | Directory-attested timestamps or external anchoring are candidates. |
| OQ-08 | Who runs the first nodes, and who pays? | Owner | M4 exit (two independent operators) | |
| OQ-09 | Security owner and contact address | Owner | SECURITY.md, M0 exit | Until set, report through private repository reporting. |
| OQ-10 | Retention and deletion in an append-only log (privacy law, user wishes) | Owner with counsel | SPEC-002, export | Likely design: erasure by encrypting payloads and discarding keys; references remain. |
| OQ-11 | Add a domain-separation prefix to every signed object before first deployment? | Spec working group | M1 (vectors regenerate) | Recommendation: yes. A protocol break relative to the current vectors, cheap now and expensive later. |
| OQ-12 | Event type registry design and the first domain schemas | Spec working group | M5 | Start from the predecessor's data model for rides, orders and listings. |
| OQ-13 | iOS push: acceptable privacy trade-off? | Owner | M5 | Apple's service is unavoidable on iOS. The design uses wake-only messages. |
| OQ-14 | Predecessor Go node and Solidity contracts: reuse anything? | Reviewer | ADR-0009 revisit | Not reviewed in the audit that produced this repository. |
