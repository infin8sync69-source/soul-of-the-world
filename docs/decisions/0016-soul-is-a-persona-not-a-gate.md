# ADR-0016: "Soul" is the assistant's persona layer, never an admission rule

- Status: Accepted
- Date: 2026-10-09
- Related: OQ-05, [vision non-goals](../vision.md), REQ-AI-01..04, T-13

## Context
In the predecessor desktop app, "Soul" was three things at once: a per-install agent key, a constitution text derived from a religious source that was prefixed to every model prompt, and a hash of that text that peers had to match before they were trusted. The third use made agreement with one document a precondition for joining the network. The project's goal is one identity for every person, globally, and the owner delegated this decision with the instruction to proceed.

## Decision
- **Soul** is the name of the user-facing AI assistant and of its configurable persona and values layer.
- A persona is a versioned document the user selects. The project may ship a default persona and the founder's constitution as one selectable option; a user may choose none.
- Network admission, peer trust and identity verification **never** depend on a persona or on any text hash. They depend on signatures (SPEC-001), community vouching and operator policy only.
- Persona text is never sent to a node or peer as a trust credential.
- The assistant must disclose which persona and which model are active (REQ-AI-04).

## Alternatives considered
- Keep the hash-match admission rule: partitions the network on every text edit and excludes people on grounds unrelated to trustworthiness. Rejected.
- Drop the constitution entirely: discards something the founder values and that some users may want. Not necessary; it fits as an option.

## Consequences
Removes one coupling between the agent and the protocol. The founder's constitution lives on as content, not as infrastructure. Documentation should treat the religious source with respect and without implying the project endorses or requires it.

## Revisit when
The owner objects to this reading of their intent, in which case the alternative must still satisfy the non-goal "not gated by belief or affiliation" in the vision, or the vision must change by ADR.
