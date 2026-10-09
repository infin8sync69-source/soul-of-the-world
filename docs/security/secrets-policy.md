# Secrets policy

Status: Accepted. Applies to all repositories, issue trackers, chat and documentation of this project. Origin: three separate incidents in the predecessor repositories in which live credentials ended up in public git history.

## Rules

1. **No secret in git, ever.** Passwords, API keys, tokens, private keys, mnemonics, cluster secrets, signing keys, `.env` files with real values. Use CI secret stores, local environment variables, or a secret manager.
2. **Examples use obvious placeholders.** `EXAMPLE_API_KEY`, never a real value with a few characters changed.
3. **Scan on every change.** A secret scanner runs as a pre-commit hook and in CI on every pull request. A finding blocks the merge.
4. **Test keys are labelled.** Keys in test vectors are derived from public labels and are documented as unusable for real purposes.
5. **Private keys of users are not "secrets we manage".** The system never receives them ([T-01](threat-model.md)).

## If a secret is exposed

Treat as compromised from the moment it was pushed anywhere that was or could have been public, even briefly.

1. **Rotate first.** Issue a new credential and revoke the old one at the provider. This is the only step that removes risk.
2. **Check for misuse.** Review the provider's access logs for the exposure window.
3. **Remove from history** after rotation (history rewrite, force-push, invalidate caches and forks where possible). Understand that copies may remain.
4. **Record** what happened, the exposure window, and what changed, in the project's incident log without repeating the value.
5. **Fix the cause** (missing scanner, unsafe default, copy-paste habit).

## Release and signing keys

- Release signing keys are kept offline or in hardware, never in CI environment variables.
- CI signs through a short-lived mechanism or a signing service that holds the key; a compromised CI cannot export it.
- The public key is pinned in installers. The rotation procedure (two keys valid during a window, announced through a signed manifest) is a milestone M3 deliverable.

## Node operators

Operators keep their node's key and configuration private. The project provides a template that excludes secret files from backups pushed to public locations and from container images.

## Ownership

Until an owner is named ([OQ-09](../open-questions.md)), the repository owner is responsible for rotation decisions.
