set export
set dotenv-load := true

# Default command, runs when no arguments are given
_default:
    @just --list

# Run the dev server
dev:
    pnpm --filter web dev

# Run linting and fix issues
tidy *flags:
    cargo fmt --manifest-path packages/simulation/Cargo.toml
    cargo fmt --manifest-path packages/wasm-client/Cargo.toml
    pnpm run tidy {{flags}}

# Run type checking
typecheck:
    pnpm run typecheck
    cargo check --manifest-path packages/simulation/Cargo.toml
    cargo check --manifest-path packages/wasm-client/Cargo.toml

# Run tests
test:
    cargo test --manifest-path packages/simulation/Cargo.toml
    cargo test --manifest-path packages/wasm-client/Cargo.toml
