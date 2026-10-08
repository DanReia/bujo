# bujo

A personal bullet journal system: a Rust API on a VPS (behind Tailscale), a PWA for mobile capture, and a local-first macOS desktop app built with iced. The database is a plain folder of markdown files.

The full product and format spec is in `docs/spec.md`. Read it before changing the data model, the storage format, or sync.

The old CLI prototype (clap 2, edition 2018) is being replaced; don't build on it.

## Hard Rules

- **Markdown on disk is the source of truth.** No other database. Files must stay human-readable and open cleanly as an Obsidian vault.
- **The parser must be lossless.** Any content it doesn't understand must be written back unchanged. Editing one entry must not reformat the rest of the file.
- **One parser.** All markdown parsing and serialization lives in `crates/core`. The API, desktop app and PWA (via WASM) all use it; never write a second one.
- **Desktop is local-first.** Write to disk first and sync in a background worker. The UI never blocks on the network.
- **Sync is per-file last-writer-wins.** The losing version is kept as a conflict copy and never silently discarded (see the spec).
- **Entries nest to any depth,** and children can be of any kind.

## Storage Format

| Signifier       | Markdown        |
|-----------------|-----------------|
| Task `•`        | `- [ ] text`    |
| Completed `×`   | `- [x] text`    |
| Migrated `>`    | `- [>] text`    |
| Scheduled `<`   | `- [<] text`    |
| Event `○`       | `- [o] text`    |
| Note `–`        | `- text`        |

- Nest entries with tabs.
- Write links as `[[wikilinks]]`; also read standard `[text](path.md)` links.
- File layout: `index.md`, `future-log.md`, `monthly/YYYY-MM.md`, `daily/YYYY-MM-DD.md`, `collections/<name>.md`.

## Layout

```
crates/core     # data model, lossless markdown parse/serialize, dates
crates/api      # REST API + sync endpoint, serves the PWA
crates/desktop  # macOS app (iced)
web/            # PWA
docs/spec.md    # product + format + sync spec
```

## Stack

These are decided. Don't swap them or add alternatives without asking.

| Area | Choice |
|---|---|
| Language | Rust edition 2024, as a Cargo workspace |
| `core` parser | Custom line-based parser: bujo entry lines become `Entry`, everything else is kept as `Raw` text. No AST round-tripping through a markdown library. |
| Markdown display | pulldown-cmark, for rendering only; never used to write files |
| `api` | axum + tokio; serves the PWA build with tower-http `ServeDir` |
| `desktop` | iced (tokio feature), notify for watching the folder, reqwest for sync |
| PWA | SvelteKit with `adapter-static` (client-only SPA, `ssr = false`), TypeScript, `@vite-pwa/sveltekit`, `idb` for the IndexedDB outbox |
| PWA ↔ core | `crates/core` compiled to WASM with wasm-pack/wasm-bindgen |
| JS package manager | pnpm |
| Sync | JSON over HTTP; clients poll, no websockets. BLAKE3 content hashes. |
| Hosting | HTTPS via `tailscale serve`, which the service worker requires. No app-level auth; Tailscale is the security boundary. |

Don't build UI until the frontend design step (see the spec's Build Order) has been signed off.

## Commands

```sh
cargo build --workspace
cargo test --workspace
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings

# PWA (in web/)
pnpm install
pnpm dev
pnpm build
pnpm check
```

Run `fmt`, `clippy` and `test` (and `pnpm check` for web changes) before considering work done.

## Testing

- `core` uses golden-file tests: markdown in, model, identical markdown out. Every new syntax feature needs a round-trip fixture.
- Include fixtures with content the parser doesn't recognize, to prove it is preserved.
