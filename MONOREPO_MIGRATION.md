# Monorepo Migration Complete

This repository has been successfully restructured into a monorepo format.

## Structure

```
.
├── crates/
│   ├── takeoff_core/      # Core types and utilities
│   └── takeoff_tools/     # Tools that depend on core
├── packages/
│   └── bindings/          # napi-rs Node.js bindings
├── Cargo.toml             # Rust workspace config
├── package.json           # Root workspace package.json
├── pnpm-workspace.yaml    # pnpm workspace config
└── turbo.json             # Turborepo config
```

## Installation

There appears to be a corepack signature verification issue with your Node.js installation. To resolve this and install dependencies:

### Option 1: Install pnpm globally via npm
```bash
npm install -g pnpm@10.24.0
pnpm install
```

### Option 2: Use npx
```bash
npx pnpm@10.24.0 install
```

### Option 3: Update corepack
```bash
npm install -g corepack@latest
corepack enable
pnpm install
```

### Option 4: Disable corepack temporarily
```bash
export COREPACK_ENABLE_STRICT=0
pnpm install
```

## Building

Once dependencies are installed:

```bash
# Build everything
pnpm build

# Build specific package
pnpm --filter bindings build

# Run tests
pnpm test

# Run linting
pnpm lint

# Format code
pnpm format
```

## Rust Workspace

The Rust workspace includes three members:
- `crates/takeoff_core` - Core functionality
- `crates/takeoff_tools` - Tools using the core
- `packages/bindings` - Node.js bindings

To work with Rust:

```bash
# Build all Rust packages
cargo build

# Test specific crate
cargo test -p takeoff_tools

# Run tests for bindings
cd packages/bindings && cargo test
```

## What Changed

1. **Rust Code Split**: The monolithic Rust crate has been split into:
   - `takeoff_core`: Core types, utilities, and measurements
   - `takeoff_tools`: Tool implementations that use the core

2. **Bindings Isolated**: The napi-rs bindings are now in `packages/bindings`, making it easier to add other packages in the future.

3. **Workspace Dependencies**: Shared dependencies are defined at the workspace level in the root `Cargo.toml`.

4. **Turborepo**: Task orchestration is now handled by Turbo for better caching and parallelization.

5. **Shared Configs**: Prettier, rustfmt, and other tools now have shared configurations at the root.

## Adding New Packages

### Adding a Rust Crate
1. Create `crates/new-crate/`
2. Add it to workspace members in root `Cargo.toml`
3. Create its `Cargo.toml` and reference workspace dependencies

### Adding a Node Package
1. Create `packages/new-package/`
2. Add its `package.json`
3. It will automatically be part of the pnpm workspace

## Migration Notes

- Old `src/takeoff_core` → `crates/takeoff_core/src`
- Old `src/takeoff_tools` → `crates/takeoff_tools/src`
- Tests have been split between crates and the bindings package
- Original files have been backed up as `*.backup`

