# Contributing to This Project

Thank you for your interest in contributing! Whether you're fixing bugs, adding features, improving documentation, or just asking a question — you're helping make this project better for everyone.

## 🛠️ How to Contribute

### 1. Fork & Clone

```bash
git clone https://github.com/kevinletchford/open-crawler.git
cd open-crawler
```

### 2. Create a Branch

Use a descriptive branch name:

```bash
git checkout -b fix/dead-links
```

### 3. Make Your Changes

Please:
- Follow the existing code style.
- Write comments where helpful.
- Prefer small, focused commits.

### 4. Run Tests & Format Code

Make sure everything is clean before opening a PR:

```bash
cargo fmt        # Format code
cargo clippy     # Lint
cargo test       # Run tests
```

### 5. Open a Pull Request

Push your branch and open a PR against `main` with a clear description of your changes.

---

## 📐 Code Style

- 4-space indents (use `cargo fmt`)
- Use `Result<T, E>` for fallible functions
- Prefer meaningful variable names over short ones like `x`
- Public functions and structs should have `///` doc comments

---

## 🧪 Tests

If you're fixing a bug or adding a feature, try to include a test case.

### Add:
- Unit tests in `src/`
- Integration tests in `tests/` (for end-to-end scenarios)

---

## 📝 Issues

- Found a bug? [Open an issue](http://github.com/kevinletchford/open-crawler/issues/new)
- Want to request a feature? Same process!
- Check for existing issues before filing a new one.

Use labels like:
- `good first issue` – great for new contributors
- `help wanted` – needs community help

---

## 🙏 Code of Conduct

We follow the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). Be respectful and kind in all interactions.

---

## ❤️ Thanks

Your contributions — big or small — are always appreciated. Thanks for helping improve this project!
