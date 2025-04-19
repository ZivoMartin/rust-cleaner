# rust-cleaner

**rust-cleaner** is a simple command-line tool designed to reclaim disk space by removing `target` directories from Rust projects across your filesystem. It's perfect for Rust developers who want to quickly clean up after compilation-heavy work or simply want to keep their machines tidy.

## 🧹 What it does

- Scans directories recursively for Rust projects (`Cargo.toml`)
- Identifies and removes their `target` build directories
- Can display how much disk space is saved
- Offers safety mechanisms like dry-run and path exclusions

## 🚀 Why use rust-cleaner?

If you're working on multiple Rust projects, your disk can easily fill up with gigabytes of build artifacts. `rust-cleaner` automates the boring part of cleanup, so you can focus on what matters: writing awesome Rust code.

## 📦 Installation

Clone this repository and build with Cargo:

```bash
git clone https://github.com/ZivoMartin/rust-cleaner
cd rust-cleaner
cargo build --release
```

You can then run it with:

```bash
./target/release/rust-cleaner [options]
```

## 📄 Example usage

```bash
rust-cleaner --path ~/projects --dry-run
rust-cleaner --confirm --verbose
rust-cleaner --path /mnt/data --exclude my-important-project
```

## ⚠️ Warning

Use with care. This tool *will* delete build artifacts unless you use `--dry-run`. Make sure you know what you're cleaning.

---

## 🛠️ TODO
- [ ] Implement dry-run mode
- [ ] Display total disk space reclaimed
- [ ] Add `--confirm` option for safe deletion
- [ ] Add support for excluding specific paths
- [ ] Add `--clean-only-debug` / `--release-only` options
- [ ] Add logging for deletions
- [ ] Handle symlinks safely
- [ ] Add CLI options (`--help`, `--verbose`, etc.)
- [ ] Support multiple root paths
- [ ] Optional config file (`.rust-cleanerrc`)
- [ ] Progress bar or animation
- [ ] Aggressive mode (also clean `.cargo`, `Cargo.lock`, etc.)
