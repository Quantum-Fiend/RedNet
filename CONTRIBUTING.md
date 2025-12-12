# Contributing to RedNet

Thank you for your interest in contributing to RedNet! This document provides guidelines and instructions for contributing.

## 🤝 How to Contribute

### Reporting Bugs

1. Check if the bug has already been reported in [Issues](https://github.com/yourusername/RedNet/issues)
2. If not, create a new issue with:
   - Clear title and description
   - Steps to reproduce
   - Expected vs actual behavior
   - Environment details (OS, versions)
   - Relevant logs or screenshots

### Suggesting Features

1. Check existing [Issues](https://github.com/yourusername/RedNet/issues) and [Discussions](https://github.com/yourusername/RedNet/discussions)
2. Create a new discussion or issue with:
   - Clear use case
   - Proposed solution
   - Alternatives considered
   - Impact on existing features

### Pull Requests

1. **Fork the repository**
2. **Create a feature branch**
   ```bash
   git checkout -b feature/amazing-feature
   ```
3. **Make your changes**
   - Follow coding standards (see below)
   - Add tests for new features
   - Update documentation
4. **Commit your changes**
   ```bash
   git commit -m 'Add amazing feature'
   ```
5. **Push to your fork**
   ```bash
   git push origin feature/amazing-feature
   ```
6. **Open a Pull Request**

## 📝 Coding Standards

### C Code
- Follow K&R style
- Use meaningful variable names
- Add comments for complex logic
- Check for memory leaks

### Rust Code
- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Add unit tests for new functions
- Use `#[must_use]` where appropriate

### Go Code
- Run `go fmt` before committing
- Run `go vet` and fix issues
- Follow [Effective Go](https://golang.org/doc/effective_go)
- Add godoc comments for public APIs

### Python Code
- Follow PEP 8
- Use type hints
- Add docstrings for functions
- Run `black` for formatting

### TypeScript Code
- Follow ESLint rules
- Use TypeScript strict mode
- Add JSDoc comments
- Run `npm run lint` before committing

## 🧪 Testing

- Add unit tests for new features
- Ensure all tests pass: `npm test`, `cargo test`, `go test ./...`
- Test Docker builds: `docker-compose build`
- Manual testing for UI changes

## 📚 Documentation

- Update README.md for new features
- Add inline code comments
- Update API documentation
- Include usage examples

## 🔐 Security

- Never commit secrets or credentials
- Report security vulnerabilities privately
- Follow secure coding practices
- Use dependency scanning tools

## 📄 License

By contributing, you agree that your contributions will be licensed under the MIT License.

## ❓ Questions?

- Open a [Discussion](https://github.com/yourusername/RedNet/discussions)
- Join our community chat
- Email: security@rednet.dev

Thank you for contributing! 🎉
