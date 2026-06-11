# Story 141 — data-driven registry name tables via build-time codegen (#141)

Delegated brief for GitHub issue **#141**. The registry **data is already
vendored** (the orchestrator fetched it verbatim from TSDuck — do NOT fetch or
edit it, and do NOT invent any registry values). Your job is the `build.rs`
codegen + wiring only. Touch only the files listed. Do **not** commit.

## Scope (read carefully)

Convert exactly **two** living registries from hand-written `match` arms to
build-time-generated lookups sourced from the vendored data:

- **CA_system_id** → `dvb-si/src/descriptors/ca.rs::ca_system_name(u16) -> Option<&'static str>`
- **private_data_specifier** → `dvb-si/src/descriptors/private_data_specifier.rs::private_data_specifier_name(u32) -> Option<&'static str>`

**Do NOT touch `data_broadcast_id`** — it has no vendored data file; its existing
small hand-verified lookup stays exactly as-is.

## Vendored data (source of truth — read, never edit)

- `dvb-si/registries/tsCAS.names` — use section **`[CASystemId]`** (16-bit).
- `dvb-si/registries/tsPDS.names` — use section **`[PrivateDataSpecifier]`** (32-bit).
- `dvb-si/registries/README.md` — provenance + BSD-2-Clause attribution (already written).

### `.names` mini-format the build.rs must parse
Plain text, line-oriented:
- `# …` is a comment (full-line, and inline: strip from the first `#` to EOL).
- Blank lines ignored.
- `[SectionName]` starts a section; only extract the two sections named above.
- `Bits = N` — section metadata; ignore.
- Entry: `KEY = VALUE`, split on the **first** `=`.
  - `KEY` is either a single hex value `0xNNNN` **or** an inclusive range
    `0xLO-0xHI`.
  - `VALUE` is the name: the remainder of the line, comment-stripped and
    trimmed. Names contain spaces, `/`, `,`, etc. — preserve them verbatim.
    Escape `\` and `"` when emitting the Rust string literal.

## Implementation

1. **`dvb-si/build.rs`** (new, package root — Cargo auto-detects it; std only, no
   external crates, MSRV 1.75). Parse the two sections and write
   `${OUT_DIR}/registry_names.rs` containing two private functions, e.g.:
   ```rust
   pub(crate) fn ca_system_name_generated(id: u16) -> Option<&'static str> {
       match id {
           0x0001 => Some("IPDC SPP Open Security Framework Generic Roaming"),
           // …exact arms first…
           0x0100..=0x01FF => Some("MediaGuard"),
           // …range arms…
           _ => None,
       }
   }
   pub(crate) fn private_data_specifier_name_generated(v: u32) -> Option<&'static str> { … }
   ```
   Emit `cargo:rerun-if-changed=registries/tsCAS.names` and `…/tsPDS.names`.
   Read the data via a path relative to `CARGO_MANIFEST_DIR`.
   - Overlap policy: if an exact value also falls inside a range entry, the
     **exact** entry wins (emit exact arms before range arms; Rust matches
     top-to-bottom). If two entries collide identically, keep the first and
     emit a `// note` — do not silently drop.
2. **Wire it in:** a small module (e.g. `dvb-si/src/registry_names.rs`) that does
   `include!(concat!(env!("OUT_DIR"), "/registry_names.rs"));`, declared
   `#[path]`/`mod` in `lib.rs` (private to the crate). Then make the public
   `ca_system_name` / `private_data_specifier_name` delegate to the generated
   functions (keep their exact public signatures — non-breaking).
3. **Delete** the hand-written `match` arms now superseded.

## Tests (the source of truth is the data — derive expectations FROM it)

- The values now come from the vendored files, so the existing
  `ca_system_name_verified*` / `private_data_specifier_*` tests will assert the
  **old** hand-written strings (e.g. "Seca / Mediaguard") which no longer match
  the generated TSDuck names (e.g. "MediaGuard"). Update them to assert
  **exact (id → name) pairs that appear verbatim in the vendored `.names`
  files** — read the file to get the exact string; do not invent. Cover: one
  exact entry and one range entry for CA_system_id, one exact entry for
  private_data_specifier, and an unknown id → `None`.
- Keep/adjust the "unknown returns None" tests.

## CHANGELOG (required)
Under `dvb-si/CHANGELOG.md` → `## Unreleased`, add a `### Changed` bullet: the
`ca_system_name` / `private_data_specifier_name` lookups are now generated at
build time from vendored TSDuck `.names` data (#141) — same `Option<&'static str>`
signature, fuller + drift-free coverage, attribution in `registries/README.md`.

## Constraints
- `build.rs` uses **std only** (no `syn`/`quote`/regex crates) — keep MSRV 1.75
  and `--locked` builds untouched. The generated code has no runtime deps and no
  file I/O at runtime.
- `--no-default-features` must still build (the lookups are not feature-gated).
- No magic numbers outside `#[cfg(test)]`; the generated file is the data.
- Do not add an `include`/`exclude` change that would drop `registries/` or
  `build.rs` from the package (current `exclude` already leaves them in).

## Gate commands (run ALL; fix until every one is green before finishing)
```
cargo build  --workspace --all-features --locked
cargo test   --workspace --all-features --locked
cargo build  --workspace --no-default-features --locked
cargo clippy --workspace --all-features --all-targets --locked -- -D warnings
cargo fmt --all --check
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked
```

## Boundaries
- Touch only: `dvb-si/build.rs` (new), `dvb-si/src/registry_names.rs` (new),
  `dvb-si/src/lib.rs` (one `mod` line), `dvb-si/src/descriptors/ca.rs`,
  `dvb-si/src/descriptors/private_data_specifier.rs`, `dvb-si/CHANGELOG.md`.
- Do **not** edit anything under `dvb-si/registries/` (vendored data).
- Do **not** touch `data_broadcast_id.rs`. Do **not** commit.
