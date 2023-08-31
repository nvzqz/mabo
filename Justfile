_default:
  just --choose

# Build all crates and other parts of the project
build:
  cargo build --release
  cd book && just build
  cd vscode-extension && pnpm run package

# Run the benchmarks
bench:
  cargo bench -p stef-benches

# Run clippy over all crates, testing every feature combination
check:
  cargo hack clippy --workspace --feature-powerset --no-dev-deps

# Start up the local server for the book
@book:
  cd book && just dev

# Check all links of the crates and book
linkcheck:
  cd book && just build
  lychee --cache --max-cache-age 7d \
    'book/src/**/*.md' \
    'book/book/**/*.html' \
    'crates/**/*.rs'
