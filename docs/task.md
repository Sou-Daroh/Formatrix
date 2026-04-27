# Formatrix v1.2 Sprint — Task Tracker

## Phase 1: Architecture & Correctness (~12 commits)
- [x] 1.3 Remove dead `staticlib`/`cdylib` crate-types from Cargo.toml
- [x] 1.6 Fix hardcoded `v1.0.0` footer → dynamic version via Vite define
- [x] 1.8 Rename `store.svelte.ts` → `types.ts`
- [x] 1.9 Extract duplicate `formatSize` to `src/lib/utils.ts`
- [x] 1.4 Remove 3 dead drag-drop event listeners from `api.ts`
- [x] ~~1.10 Fix case-sensitive file extension filtering in DropZone~~ (verified already correct)
- [x] 1.12 PDF merge output uses first input stem instead of `merged.pdf`
- [x] 1.7 Move `split()` from `pdf_merge.rs` to new `pdf_split.rs`
- [x] 1.5 Fix temp file leak on processing errors
- [x] 1.11 Fix quality slider: hide for lossless formats (PNG/WebP), document encoder limitations
- [x] 1.1 Introduce `thiserror` and `ProcessError` enum
- [x] 1.2 Fix memory bomb on large PDFs (BufReader/Size Guard)

## Phase 2: Safety & Robustness (~6 commits)
- [x] 2.1 Double-click guard on Process button (`isProcessing`)
- [x] 2.2 Disable Back button during processing
- [x] 2.3 Frontend validation for PDF split page ranges
- [x] 2.4 Image dimension ceiling (10000px max)
- [x] 2.5 Detect image no-op case (0×0, same format)
- [x] 2.6 Global unhandled error boundary
- [x] 3.6 Escape key to go back

## Phase 3: UX & Accessibility (~10 commits)
- [x] 3.1 Toast notification component (replace `alert()`)
- [x] 3.2 Responsive breakpoint for configure layout
- [x] 3.3 Elapsed time display during processing
- [x] 3.4 Dynamic window title per operation
- [x] 3.5 Replace emoji icons with SVG icons
- [x] 3.7 Copy to Clipboard on text extraction results
- [x] 3.8 Show input filename in processing view
- [x] 3.9 Add ARIA labels for accessibility
- [x] 3.10 Clear All button on file list

## Phase 4: New Feature — Batch Image Processing (~8 commits)
- [x] Backend: `image_batch.rs` processor
- [x] Backend: `process_image_batch` command
- [x] Backend: register command in `lib.rs`
- [x] Frontend: `processImageBatch()` API call
- [x] Frontend: image operation → `multiple: true`
- [x] Frontend: batch detection logic in App.svelte
- [x] Test: batch processing integration test
- [x] Update progress reporting for per-file batch updates

## Phase 5: Testing (~6 commits)
- [x] 5.1 Edge case tests for image processor
- [x] 5.2 Edge case tests for PDF split
- [x] 5.3 Edge case tests for CSV processor
- [x] 5.4 `parse_page_ranges` unit tests
- [x] 5.5 Temp cleanup integration test
- [x] 5.6 CI badge in README

## Phase 6: Documentation & Release (~5 commits)
- [x] 6.1 Screenshots in README
- [x] 6.2 CHANGELOG.md
- [x] 6.3 CONTRIBUTING.md
- [x] 6.4 Update README for v1.2
- [x] 6.5 Bump to v1.2.0 and release
