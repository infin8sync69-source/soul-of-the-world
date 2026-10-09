## What and why

<!-- One paragraph. Link the issue, ADR or spec. -->

## Checklist

- [ ] Behaviour is specified; the spec or ADR is updated
- [ ] Tests added or updated; vectors regenerated if signed bytes changed
- [ ] If signed bytes or IDs changed: titled `spec!:` and marked as a protocol break
- [ ] Threat model consulted; new threats recorded
- [ ] New dependency added to the register (or none)
- [ ] No secrets, machine-specific paths or large binaries
- [ ] Claims in docs are backed by a test or a measurement
- [ ] `python3 scripts/check_docs.py` passes
