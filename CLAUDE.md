# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

A single-crate Rust library that extracts page metadata (`<title>`, OpenGraph, Twitter Card, favicon, `<html lang>`) from an HTML **string**. It does **not** fetch URLs — callers supply the HTML. Built on top of the `scraper` crate (CSS selectors over an `html5ever` parse tree).

## Layout

- `src/lib.rs` — the entire library. There is no `tests/` directory; unit tests live in `#[cfg(test)] mod test` at the bottom of `lib.rs`, and doctests live in the rustdoc on each public method.
- `terraform/github/` — Terraform that manages this GitHub repo itself (labels, rulesets, dependabot etc.) via the `github` provider. Not part of the published crate. Dependabot updates these too.
- `.github/workflows/unit-test.yml` — CI: runs `cargo test` on every push/PR plus a nightly cron.
- `.github/workflows/publish.yml` — release: tag-triggered publish to crates.io + GitHub Release (see "Releasing").

## Commands

```bash
cargo test                          # unit tests + doctests
cargo test <name>                   # run a single test (substring match)
cargo test --doc                    # doctests only
cargo test --lib                    # unit tests only
cargo clippy --all-targets          # lint
cargo fmt                           # format
```

When you change a public method's docstring, run `cargo test --doc` — the examples are real doctests, not decoration.

## Architecture

The public API on `MetaScraper` has a two-tier shape that matters when adding or modifying behavior:

- **Per-source `extract_*` methods** (`extract_og_title`, `extract_twitter_image`, `extract_title`, …) — each binds to exactly one HTML source and returns `Option<String>`.
- **Aggregate accessors** (`title`, `description`, `image`) — call the `extract_*` methods in a fixed priority order and return the first match. Don't reimplement selector logic at this layer; just chain `.or_else(...)`.

Three behavioral invariants run through every `extract_*` method, established as bug fixes — preserve them when editing:

1. **Both attribute variants are accepted.** OG selectors match `meta[property='og:X']` AND `meta[name='og:X']`. Twitter selectors match `meta[name='twitter:X']` AND `meta[property='twitter:X']`. Real-world CMSes emit either.
2. **Empty `content=""` is treated as absent** — `.filter(|c| !c.is_empty())` after the attribute lookup, so consumers can rely on `Some(...)` meaning a non-empty value.
3. **`favicon()` uses `link[rel~='icon']`** (whitespace-token match), so `rel="shortcut icon"` and `rel="icon shortcut"` both match. Exact-match `[rel='icon']` was a bug.

`extract_title()` additionally trims whitespace and returns `None` for empty/whitespace-only `<title>`.

## Releasing

Versioning convention: pre-1.0, behavior changes go in a **minor** bump (Cargo treats `0.x → 0.(x+1)` as incompatible). Don't ship behavior shifts under a patch bump — downstream auto-upgrades will silently change results.

Release flow:

1. Bump `version` in `Cargo.toml`, commit.
2. `git tag vX.Y.Z && git push --tags`.
3. `.github/workflows/publish.yml` fires: verifies the tag matches `Cargo.toml`, runs `cargo test`, exchanges the GitHub OIDC token for a short-lived crates.io token via `rust-lang/crates-io-auth-action@v1`, publishes, then creates a GitHub Release with auto-generated notes.

Trusted publishing is configured on the crates.io side (no `CARGO_REGISTRY_TOKEN` secret in this repo). If the action ever fails auth, check `crates.io/crates/html-meta-scraper/settings` → Trusted Publishing for: repo `46ki75/html-meta-scraper`, workflow filename `publish.yml`, environment blank.
