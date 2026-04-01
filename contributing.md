# Contributing

Thanks for improving deadlocked.
Performance, stability, and code quality fixes are always welcome.
For larger feature work, open an issue first so the scope is aligned before implementation.

## Project overview

- `src/cs2`: game-specific logic (entities, features, offsets, targeting, input)
- `src/ui`: GUI + overlay rendering and window lifecycle
- `src/parser`: map parsing and BVH visibility data
- `src/os`: low-level process/mouse/crash handling
- `src/game.rs` + `src/router.rs`: game loop and message flow between game/UI threads

## Contribution guidelines

- Keep pull requests focused. Small PRs are easier to review and safer to merge.
- Prefer behavior-preserving refactors before functional changes.
- Do not mix unrelated changes (for example: feature + style-only cleanup).
- Preserve existing architecture unless a refactor is explicitly discussed first.
- Add or update tests when behavior changes.

## Pull request checklist

- Build passes locally (`cargo check`).
- Tests pass (`cargo test`).
- Lint is clean or intentionally documented (`cargo clippy --all-targets --all-features`).
- Config/backward compatibility is considered for new fields.
- User-facing UI changes are localized (EN/RU) when applicable.
