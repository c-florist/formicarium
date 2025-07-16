set export
set dotenv-load := true

# Default command, runs when no arguments are given
_default:
    @just --list

# Run the full stack in dev mode
dev:
    pnpm tauri dev

# Run linting and fix issues
tidy *flags:
    cargo fmt --manifest-path packages/core/Cargo.toml
    pnpm run tidy {{flags}}

# Run type checking
typecheck:
    pnpm run typecheck
    cargo check --manifest-path packages/core/Cargo.toml

# Run tests
test:
    cargo test --manifest-path packages/core/Cargo.toml
