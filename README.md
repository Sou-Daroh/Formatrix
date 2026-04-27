# Formatrix

[![CI](https://github.com/Sou-Daroh/formatrix/actions/workflows/ci.yml/badge.svg)](https://github.com/Sou-Daroh/formatrix/actions/workflows/ci.yml)

A native desktop application for file conversion and processing. Built with Tauri, Rust, and Svelte.

Convert images, transform data, and manipulate PDFs — all processed locally on your machine. No internet connection required. No files leave your device.

**Platform:** Windows, macOS, Linux  
**Repository:** https://github.com/Sou-Daroh/formatrix

---

## Operations

| Operation | Input | Output | Description |
|---|---|---|---|
| Image Resize / Compress | JPG, PNG, WebP, GIF, BMP, TIFF | JPG, PNG, WebP | Resize by dimensions and re-encode with quality control |
| Batch Image Processing | Multiple images | ZIP | Process multiple images with shared settings in one pass |
| CSV → JSON | CSV | JSON | Convert tabular CSV data to a JSON array of objects |
| PDF → Text | PDF | TXT | Extract all selectable text content from a PDF |
| PDF Merge / Split | PDF(s) | PDF or ZIP | Combine multiple PDFs or extract page ranges |

---

## Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (stable, 1.77+)
- [Bun](https://bun.sh/) (1.x+)

### Run in development

```bash
git clone https://github.com/Sou-Daroh/formatrix
cd formatrix
bun install
bun tauri dev
```

### Build for production

```bash
bun tauri build
```

Output installers are in `src-tauri/target/release/bundle/`.

---

## Stack

- **Shell:** Tauri 2 — native webview, file system access, IPC
- **Backend:** Rust — all file processing logic, zero system dependencies
- **Frontend:** Svelte 5 + Vite — compiled UI, no runtime framework overhead
- **Styling:** Custom CSS — dark industrial theme, IBM Plex Mono + DM Sans

---

## Project Structure

```
formatrix/
├── src/                    # Svelte frontend
│   ├── lib/
│   │   ├── components/     # Svelte UI components
│   │   ├── api.ts          # Tauri IPC calls
│   │   ├── utils.ts        # Shared utilities
│   │   └── types.ts        # TypeScript type definitions
│   └── app.css             # Global styles
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── lib.rs          # Tauri entry point
│   │   ├── commands/       # IPC command handlers
│   │   └── processor/      # One module per operation
│   ├── tests/              # Integration tests
│   ├── Cargo.toml
│   └── tauri.conf.json
├── docs/                   # Project documentation
└── package.json
```

---

## Relationship to Formatrix Web

Formatrix Desktop is the first phase of the Formatrix project. The web application (phase 2) will expose the same four operations via a browser interface with a Go backend. The operation specifications, UI design language, and feature set are shared between the two phases.

---

## Known Limitations

- PDF text extraction works on selectable-text PDFs only. Scanned image-based PDFs return empty output.
- No operation history between sessions.

---

## Documentation

- [CHANGELOG](docs/CHANGELOG.md) — release history
- [CONTRIBUTING](docs/CONTRIBUTING.md) — how to contribute

---

## License

MIT. See [LICENSE](LICENSE).
