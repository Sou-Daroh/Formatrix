# Changelog

All notable changes to Formatrix Desktop are documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).  
Versioning follows [Semantic Versioning](https://semver.org/).

---

## [1.2.1] — 2026-04-27

### Added
- Inline preview for image results (JPEG, PNG, WebP) in the result view
- Inline preview for text/JSON results in a scrollable code block
- Open file explorer with saved file selected after Save As
- Copy to clipboard support for JSON output (previously text/plain only)

### Fixed
- File sizes showing 0 B — added missing `fs:allow-stat` permission
- File size not readable outside Downloads/Desktop/Documents — widened fs scope to `$HOME`
- Back button crash — replaced blocked browser `confirm()` with Tauri dialog plugin `confirm()`
- Added `dialog:allow-confirm` capability

### Changed
- Result card widened from 400px to 560px to accommodate previews
- Asset protocol enabled for serving local images in webview

---

## [1.2.0] — 2026-04-27

### Added
- Batch image processing — drop multiple images, process all with shared settings, get a ZIP
- Toast notification system replacing native `alert()` dialogs
- Elapsed time display during processing
- Dynamic window title per operation
- Copy to clipboard for PDF text extraction results
- Input filename display in processing view
- Clear All button on file list for multi-file operations
- Escape key to navigate back
- Frontend validation for PDF split page ranges
- Image dimension ceiling (10,000px max)
- Image no-op detection (skip when nothing would change)
- Global unhandled error boundary
- ARIA labels for accessibility
- Responsive breakpoint for narrow windows
- SVG icons replacing platform-dependent emoji
- Edge case tests for image, CSV, and PDF split processors
- `parse_page_ranges` unit tests
- Temp cleanup integration test
- GitHub Actions CI with badge in README
- CHANGELOG.md and CONTRIBUTING.md

### Changed
- `thiserror` `ProcessError` enum replaces raw `String` errors across all processors
- PDF split moved to dedicated `pdf_split.rs` module
- `store.svelte.ts` renamed to `types.ts`
- Duplicate `formatSize` extracted to shared `src/lib/utils.ts`
- PDF merge output named after first input file instead of `merged.pdf`
- Quality slider hidden for lossless formats (PNG, WebP)

### Fixed
- Memory bomb on large PDFs — added 200 MB size guard with `BufReader`
- Temp file leak on processing errors — orphaned directories now cleaned up
- Double-click on Process button spawning duplicate tasks
- Back button clickable during active processing
- Dead Tauri v1 drag-drop event listeners consuming memory
- Hardcoded `v1.0.0` footer replaced with dynamic version
- Triple compile time from unused `staticlib`/`cdylib` crate types
- Empty CSV files now rejected with validation error
- Empty PDF split page ranges now rejected with validation error
- Temp cleanup test race condition with parallel tests

### Removed
- 3 dead drag-drop event listeners (`tauri://drag-drop`, `tauri://drop`, `tauri://file-drop`)
- `staticlib` and `cdylib` crate types from `Cargo.toml`

---

## [1.0.1] — 2026-04-10

### Fixed
- Minor bug fixes and stability improvements

---

## [1.0.0] — 2026-04-09

Initial release of Formatrix Desktop.

### Added
- Image resize and compress (JPEG, PNG, WebP output via Lanczos3 resampling)
- CSV to JSON conversion
- PDF text extraction
- PDF merge and split
- Svelte 5 frontend with three-step flow (choose, configure, result)
- Native file open and save dialogs via Tauri
- Drag-and-drop file input
- Inline image and text/JSON preview
- Progress bar with Tauri event updates
- Dark industrial UI theme with IBM Plex Mono and DM Sans
- Windows, macOS, and Linux support
