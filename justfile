set export
set dotenv-load := true

# Default command, runs when no arguments are given
_default:
    @just --list

# Install all project dependencies
init:
    pnpm install

# Starts the development server with watch mode
dev:
    pnpm --filter @formicary/api dev

# Builds the application for production
build:
    pnpm --filter @formicary/api build

# Starts the production build
start:
    pnpm --filter @formicary/api start

# Run linting and fix issues
tidy:
    pnpm run tidy

# Run linting and check issues
tidy-check:
    pnpm run tidy:check
