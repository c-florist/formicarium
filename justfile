set export
set dotenv-load := true

# Default recipe to list all available recipes
_default:
    @just --list

# Install application dependencies
init:
    pnpm install
