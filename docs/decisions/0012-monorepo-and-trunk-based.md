# ADR-0012: One repository, trunk-based development

- Status: Accepted
- Date: 2026-10-09
- Related: [engineering handbook](../engineering/handbook.md)

## Context
The predecessors were split across six repositories with duplicated code, broken gitlinks and large committed binaries. The mobile repository's own contributing guide already preferred a single trunk and shared contract.

## Decision
One repository holds core, apps, node, specs, docs and infrastructure. Trunk-based development with short-lived branches, required review and CI, and squash merges. Large binaries and models stay out of git.

## Alternatives considered
Multiple repositories per component: more coordination for cross-cutting protocol changes, which are frequent early on.

## Consequences
CI must be path-aware to stay fast. Access control is repository-wide.

## Revisit when
Build times or contributor structure make a split worthwhile, or a component needs a different licence or audience.
