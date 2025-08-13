# Git Guidelines for SynthMob

## ✅ What TO Commit

### Source Code
- `src/` - React frontend source
- `src-tauri/src/` - Rust backend source
- `src-tauri/Cargo.toml` - Rust dependencies
- `src-tauri/build.rs` - Build script
- `src-tauri/tauri.conf.json` - Tauri configuration

### Frontend Configuration
- `package.json` - Node.js dependencies and scripts
- `tsconfig.json` - TypeScript configuration
- `vite.config.ts` - Vite bundler configuration
- `index.html` - HTML entry point

### Project Files
- `README.md` - Project documentation
- `ROADMAP.md` - Development plan
- `dev.sh` - Development scripts
- `.gitignore` - Git exclusion rules

### Assets
- `src-tauri/icons/` - App icons (the RGBA PNGs you created)

## ❌ What NOT to Commit

### Build Artifacts
- `src-tauri/target/` - Rust build output
- `src-tauri/gen/` - **Generated Tauri files**
- `dist/` - Frontend build output
- `target/` - Any Rust target directories

### Dependencies
- `node_modules/` - Node.js packages
- `src-tauri/Cargo.lock` - Usually excluded, but can be committed for reproducible builds

### Development Files
- `.vscode/` - VS Code settings (optional - some teams commit these)
- IDE temp files
- Log files
- OS-specific files

## Why `src-tauri/gen/` Should Not Be Committed

The `src-tauri/gen/` folder contains:
- Auto-generated schema files
- Build-time generated code
- Platform-specific generated files
- These are recreated during build process

## Current Git Status

Your `.gitignore` now properly excludes:
```
src-tauri/gen/
src-tauri/target/
node_modules/
dist/
```

## Recommended Git Workflow

```bash
# Add source files
git add src/ src-tauri/src/ src-tauri/Cargo.toml src-tauri/tauri.conf.json
git add package.json tsconfig.json vite.config.ts index.html
git add README.md ROADMAP.md dev.sh
git add src-tauri/icons/

# Commit
git commit -m "Initial SynthMob Tauri app setup"
```

The build artifacts in `src-tauri/gen/` and `src-tauri/target/` will be automatically recreated when someone clones and builds your project.
