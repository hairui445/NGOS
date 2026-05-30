 Contributing to NGOS

Welcome to the NGOS community! We appreciate your interest in contributing. To maintain a clean, high-performance architecture, please read the following guidelines before submitting your code.

## Workflow

1. **Fork the repository**: Fork the project to your personal account.
2. **Create a branch**: Create a feature or fix branch in your forked repository (e.g., `feat/xxx` or `fix/xxx`).
3. **Commit your changes**: Ensure your changes adhere to the project's coding style.
4. **Submit a Pull Request (PR)**: Open a PR against the `main` branch of the upstream repository.

## Engineering Standards

### Code Style
Before submitting, please ensure your code meets industrial standards by running:
```bash
cargo fmt --all

```

### Testing

All submitted code must pass existing tests and maintain or improve test coverage:

```bash
cargo test --workspace

```

### ABI Compatibility

If you modify or add plugin-related code, please verify it using the certification tool:

```bash
cargo run -p ngos-cert -- <path_to_your_plugin>

```

## Communication & Collaboration

* **Issue First**: For major bugs or significant architectural changes, please open an Issue to discuss the proposal before submitting a PR.
* **Commit Messages**: Please follow the [Conventional Commits](https://www.conventionalcommits.org/) specification (e.g., `feat:`, `fix:`, `docs:`, `refactor:`).

---

*Thank you for your contributions! Together, we can make NGOS even better.*

