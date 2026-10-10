## Sorolint Findings

| # | Severity | File | Line | Check | Function | Description | Suggestion |
|---|----------|------|------|-------|----------|-------------|------------|
| 1 | **HIGH** | src/lib.rs | 15 | [missing-require-auth](https://github.com/Sorobscan/Sorolint/blob/main/docs/checks.md#missing-require-auth-high) | bump | Method `bump` writes to `env.storage()` but does not call `env.require_auth()`. Callers may mutate contract state without proving they are authorized. | Add `env.require_auth();` as the first statement in the function body. |
| 2 | MEDIUM | src/lib.rs | 15 | [missing-event-emission](https://github.com/Sorobscan/Sorolint/blob/main/docs/checks.md#missing-event-emission-medium) | bump | Method `bump` writes to storage but does not emit an event. Consider using `env.events().publish(…)` to allow off-chain indexers to track contract activity. | Add `env.events().publish((symbol_short!("bump"), …), &payload)` after the storage write so off-chain indexers can track this state change. |

**2 finding(s): 1 High, 1 Medium, 0 Low**
