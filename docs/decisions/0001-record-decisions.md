# ADR-0001: Record decisions as ADRs

- Status: Accepted
- Date: 2026-10-09
- Related: [engineering handbook](../engineering/handbook.md)

## Context
The predecessor projects contained several planning documents that contradicted each other and the code (one described Ethereum and MongoDB, which did not exist). Nobody could tell which statement was current.

## Decision
Every architecture, protocol or dependency decision is a numbered ADR in this directory. Prose documents describe the current design and link to the ADRs that justify it.

## Alternatives considered
Free-form design docs: drift and contradiction, as observed. Issue discussions only: not discoverable.

## Consequences
Small overhead per decision. A reviewer can ask "where is the ADR" for any significant change.

## Revisit when
The number of ADRs makes the index unusable.
