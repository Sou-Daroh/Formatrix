# Contributing

Formatrix Desktop is a portfolio project with a narrow, intentional scope. External contributions are welcome within those boundaries.

---

## What Is Welcome

- **Bug fixes** — incorrect output from a processor, UI crashes, platform-specific rendering issues
- **Test additions** — Rust unit tests with new fixture files, especially edge cases
- **Documentation corrections** — typos, inaccurate descriptions, outdated steps
- **Platform-specific fixes** — issues specific to a particular OS or webview engine
- **Security issues** — see the reporting process below

---

## What Is Not Welcome

- New operations — the four operations in v1 are the complete scope
- Framework migrations — the Tauri + Rust + Svelte stack is final
- UI component library additions — the UI is hand-written, no component libraries
- Runtime JavaScript dependencies beyond `@tauri-apps/*` and `svelte`
- Rust dependencies that use FFI, CGo, or system library bindings
- Network requests of any kind
- AI or ML features

---

## Reporting a Bug

Open a GitHub issue using the bug report template. Include:

1. Operating system and version
2. Steps to reproduce
3. Expected behaviour
4. Actual behaviour (error message, screenshot if applicable)
5. Input file details (type, approximate size) — do not attach files with personal data

---

## Submitting a Pull Request

1. Fork the repository
2. Create a branch: `git checkout -b fix/describe-the-fix`
3. Make the change
4. Run `cargo test` from `src-tauri/` and confirm all tests pass
5. Run `cargo clippy -- -D warnings` and fix all warnings
6. Run `bun check` and fix all TypeScript errors
7. Run `cargo fmt` and `bun format` to format the code
8. Commit with a clear message: `fix: handle corrupt JPEG input gracefully`
9. Open a pull request against `main`

One issue per PR. Do not bundle unrelated fixes.

---

## Code Style

### Rust

- Format with `cargo fmt`
- No `unwrap()` or `expect()` in non-test code — use `?` and return `Result`
- No `panic!()` in non-test code
- All public functions have doc comments (`///`)
- Error messages are lowercase, no trailing punctuation: `"could not open file"` not `"Could not open file."`

### TypeScript / Svelte

- Format with Prettier (`bun format`)
- No `any` — all types are explicit
- No `// @ts-ignore` — fix the type error instead
- Use Svelte 5 runes (`$state`, `$derived`, `$effect`) — no Svelte 4 patterns
- No `<input type="file">` — use Tauri file dialog commands

### General

- No emoji in code, comments, or documentation
- Commit messages follow conventional commits: `fix:`, `feat:`, `docs:`, `chore:`, `test:`

---

## Security Issues

Do not open a public GitHub issue for security vulnerabilities. Email the repository owner directly with:

- A description of the vulnerability
- Steps to reproduce
- Potential impact

Given that Formatrix is a personal-use desktop tool with no network access and strict Tauri capability grants, the attack surface is limited. However, path traversal, arbitrary file write outside the temp directory, or capability bypass issues should be reported privately.

---

## License

By contributing, you agree that your contributions will be licensed under the MIT licence that covers this project.
