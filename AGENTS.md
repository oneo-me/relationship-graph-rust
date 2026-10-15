# oneo-gpui-graph

A standalone GPUI Kit component for interactive relationship graphs. Read [ARCHITECTURE.md](ARCHITECTURE.md) before changing code, then the relevant subjects.

## Principles

- Keep the crate independent of any host application; IDs, labels, clusters, and colors stay opaque.
- Keep `model.rs` and `layout.rs` free of GPUI, pixels, and themes; rendering belongs in `paint.rs` and `view.rs`.
- Preserve the public API and behavior. This is a 1:1 extraction for external consumers.
- Keep the force layout deterministic and frame-rate independent; avoid untested physics changes.

## Working with the user

- Do not commit, push, tag, or publish without explicit authorization.
- Ask before renaming the crate, changing the public API, or relaxing validation rules.
- Keep changes focused; do not add features the user did not request.

## Verification

- Run `cargo test -p oneo-gpui-graph` and `cargo doc -p oneo-gpui-graph` before finishing.
- Review visual or interface changes in `cargo run -p oneo-gpui-graph --example relationships`.
- Run `git diff --check` from the repository root.

## Documentation

- Update a document only when a fact its reader needs changes.
- Keep each document within its budget and written for its reader.
- Record completed user-visible outcomes as short `Unreleased` bullets, merging related ones.
- Repair documents you touch instead of only appending to them.
- Keep `README.md` and `README.zh-CN.md` equivalent; everything else, including code comments, is English.
- Keep documentation on the code branch and reconcile it during merges.
