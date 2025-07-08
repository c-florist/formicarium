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
    pnpm dev:api

# Builds the application for production
build:
    pnpm build

# Starts the production build
start:
    pnpm start

# Run linting and fix issues
tidy:
    pnpm run tidy

# Run linting and check issues
tidy-check:
    pnpm run tidy:check

# Run type checking
typecheck:
    pnpm typecheck

# Run tests
test package='all':
    #!/usr/bin/env sh
    if [ "$package" = "all" ]; then
        pnpm --filter "@formicarium/*" test:unit
    else
        pnpm --filter "@formicarium/$package" test:unit
    fi

# Run tests in watch mode
tdd package:
    pnpm --filter {{package}} test:unit:watch
