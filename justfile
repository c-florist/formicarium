set export
set dotenv-load := true

# Default command, runs when no arguments are given
_default:
    @just --list

# Run linting and fix issues
tidy *flags:
    cargo fmt --manifest-path packages/core-rs/Cargo.toml
    pnpm run tidy {{flags}}

# Run type checking
typecheck:
    pnpm run typecheck
    cargo check --manifest-path packages/core-rs/Cargo.toml

# Run tests
test:
    pnpm --filter "@formicarium/*" test:unit
    cargo test --manifest-path packages/core-rs/Cargo.toml
