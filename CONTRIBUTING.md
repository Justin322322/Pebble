# Contributing to Pebble

Thank you for your interest in contributing to Pebble! This is an educational project designed to help understand ORM concepts in Rust.

## Getting Started

1. Fork the repository
2. Clone your fork
   ```bash
   git clone https://github.com/yourusername/pebble.git
   cd pebble
   ```
3. Install Rust if you haven't already: https://rustup.rs/
4. Build the project
   ```bash
   cargo build
   ```
5. Run the tests
   ```bash
   cargo test
   ```

## Development Workflow

1. Create a new branch for your feature or bugfix
   ```bash
   git checkout -b feature/my-new-feature
   ```

2. Make your changes
   - Write clean, idiomatic Rust code
   - Add tests for new functionality
   - Update documentation as needed

3. Run the tests
   ```bash
   cargo test
   ```

4. Check formatting
   ```bash
   cargo fmt --check
   ```

5. Run clippy for linting
   ```bash
   cargo clippy -- -D warnings
   ```

6. Commit your changes
   ```bash
   git add .
   git commit -m "Add feature: description"
   ```

7. Push to your fork
   ```bash
   git push origin feature/my-new-feature
   ```

8. Create a Pull Request on GitHub

## Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Use meaningful variable and function names
- Add doc comments for public APIs
- Keep functions focused and small
- Write comprehensive tests

## Testing

- All new features should include tests
- Run `cargo test` before submitting
- Aim for high test coverage
- Test edge cases and error conditions

## Documentation

- Update README.md for user-facing changes
- Add doc comments (`///`) for public APIs
- Include examples in doc comments when helpful

## Ideas for Contributions

Here are some areas where contributions would be welcome:

### Features
- Better type mapping (INTEGER, REAL, BLOB types)
- Support for NULL values
- Transaction support
- Derive macro for Model trait
- Support for composite primary keys
- JOIN operations
- Aggregation functions (COUNT, SUM, AVG, etc.)
- Migration system

### Improvements
- Better error messages
- Performance optimizations
- More comprehensive examples
- Benchmark suite
- Connection pooling

### Documentation
- Video tutorials
- Blog posts
- More examples
- API reference improvements

## Questions?

Feel free to open an issue for:
- Bug reports
- Feature requests
- Questions about the codebase
- Documentation improvements

## Code of Conduct

Be respectful, constructive, and collaborative. This is a learning project - everyone is welcome!

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
