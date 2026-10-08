# bujo Specification

A personal bullet journal. Single user, self-hosted on a VPS behind Tailscale.

## Collections

### Index
- Lists custom collections with links to each.
- Generated automatically from the `collections/` folder.
- Can also hold hand-written entries, which are preserved when the index is regenerated.

### Future Log
- The next 6 months, one section per month.
- Holds tasks and events scheduled beyond the current month.

### Monthly Log
- Calendar page: one line per day of the month (date and weekday) for events.
- Task page: goals and tasks for the month.

### Daily Log
- One file per day, with entries written as signifiers (see below).
- Any entry can have nested children of any kind, to any depth.

### Custom Collections
- Free-form pages that accept every signifier plus general markdown.
- Links are clickable and can be followed, as in Obsidian.

## Storage Format

The database is a plain folder of markdown files. It must stay readable and editable by a human in a text editor, and should open cleanly as an Obsidian vault.

### File layout

```
<data_dir>/
  index.md
  future-log.md
  monthly/2026-10.md
  daily/2026-10-08.md
  collections/<name>.md
```

### Signifiers

| Signifier       | Markdown        |
|-----------------|-----------------|
| Task `•`        | `- [ ] text`    |
| Completed `×`   | `- [x] text`    |
| Migrated `>`    | `- [>] text`    |
| Scheduled `<`   | `- [<] text`    |
| Event `○`       | `- [o] text`    |
| Note `–`        | `- text`        |

- `[>]` and `[<]` follow the Obsidian Tasks plugin and Minimal theme conventions.
- Plain Obsidian renders any non-space checkbox character as checked, so `[>]`, `[<]` and `[o]` show as ticked boxes there. That is acceptable.

### Nesting
- Child entries are indented by one tab per level (Obsidian's default).

### Links
- Write links as `[[wikilinks]]` (Obsidian's default).
- Read and follow standard `[text](path.md)` links as well.

### Migration and scheduling
- Migrating (`>`) copies the task to the target log and marks the original `[>]`.
- Scheduling (`<`) copies the task into the Future Log or a Monthly Log and marks the original `[<]`.
- The copy contains a wikilink back to the source day.

### Fidelity
- The parser must be lossless. Any content it does not understand (headings, prose, frontmatter, unknown checkbox characters) must be written back byte for byte.
- Editing one entry must not reformat the rest of the file.
- Implementation: a custom line-based parser. A file is a sequence of blocks: `Entry` (signifier line + nested children) or `Raw` (any other text, kept verbatim). `serialize(parse(s)) == s` must hold for every input. pulldown-cmark is used only to render markdown for display, never to write files.

## Architecture

```
crates/core     # shared: data model, markdown parse/serialize, date logic
crates/api      # REST API on the VPS, serves the data folder and the PWA
crates/desktop  # macOS app (iced), local-first
web/            # PWA
```

### API (VPS)
- Rust API on the VPS. The data folder on disk is the database.
- Reached only over Tailscale. Served over HTTPS via `tailscale serve` or `tailscale cert`; service workers need HTTPS.
- No authentication in the app itself; Tailscale is the security boundary.

### Desktop (macOS, iced)
- Keeps a full local copy of the data folder.
- Writes to disk first; a background worker syncs with the API. The UI never waits on the network.
- Watches the folder for outside edits (e.g. from Obsidian) and reloads changed files.

### PWA
- Primary capture tool on mobile; should also be usable on desktop.
- Works offline. New entries go into an IndexedDB outbox and sync when online.
- iOS Safari does not support Background Sync, so sync runs when the app opens, regains focus, or comes back online.
- Stack: SvelteKit (adapter-static, client-only) + TypeScript + `@vite-pwa/sveltekit`, managed with pnpm. It uses `crates/core` compiled to WASM, so there is still only one parser. The IndexedDB outbox uses `idb`.

## Sync

Per-file, last-writer-wins, over JSON/HTTP. Clients poll; there are no websockets. Content hashes use BLAKE3.

- Each client tracks the content hash of every file as of its last sync (the "base").
- When pushing, the client sends the new content together with its base hash.
- If the server's current hash matches the base, the server accepts the push.
- If it doesn't match (a conflict), the latest write wins. The losing version is saved next to it as `<name>.conflict-<device>-<YYYYMMDDTHHMMSS>.md`, so no data is lost.
- Deletions are synced as tombstones, so a deleted file isn't brought back by a stale client.

## Build Order

1. `core`: data model plus lossless parse/serialize, with golden-file tests.
2. `api`: read and write files, plus the sync endpoint.
3. Frontend design, covering both the PWA and the desktop app:
   - Choose a shared visual language: typography, spacing, colors (light and dark), and how each signifier looks.
   - Design the mobile capture flow first: open the app, add an entry with a signifier, nest it. This should take as few taps as possible.
   - Lay out each collection (Index, Future Log, Monthly calendar + tasks, Daily, Custom) at phone and desktop widths.
   - Define the interactions: completing, migrating and scheduling tasks, following links, and showing sync and conflict state.
   - Produce mockups or a clickable prototype and get sign-off before building.
4. PWA: capture to the daily log, then browse the other collections.
5. Desktop app.
6. Desktop background sync.
