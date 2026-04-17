# Raw Sources

Dump zone for unprocessed inputs before they get synthesized into the wiki.

## What goes here

- YouTube video transcripts (copy/paste from "Show transcript" or use `yt-dlp --write-subs`)
- Web articles (via **Obsidian Web Clipper** — set default folder to `raw/`)
- PDF exports, book chapters, course notes, lecture slides
- Code snippets with context (lessons, tutorials, gists)
- Personal notes / draft thoughts before integration

## Naming convention

```
YYYY-MM-DD_short-title.md
```

Example: `2026-04-17_karpathy-llm-wiki.md`

## Workflow

1. Drop the source file here
2. Tell Claude: *"Ingest `raw/<filename>` into the wiki"*
3. Claude reads the source, asks which domain(s) it belongs to and the focus angle, then proposes wiki updates (new page, edits to existing pages, cross-references)
4. You review and validate before any wiki page is touched

## Not to confuse

- `raw/` = inputs (messy, verbatim, unstructured)
- `<domain>/` folders (rust, ros2, mathematics…) = processed, synthesized, validated learning notes

Raw files are kept for traceability — you can always re-ingest them later if conventions change.
