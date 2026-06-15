
# Sebuilder

Build Scratch Everywhere! from .sb3 files

## Installation

### Requirements

- Rust toolchain ([Download](https://rust-lang.org/tools/install/))
- Docker installation

### Install

```bash
cargo install sebuilder
```

# Examples

Basic building
```bash
sebuilder --name myproject -d "This is a description" --author you -s Project.sb3
```

Custom Platform
```bash
sebuilder --name myproject -d "This is a description" --author you -s Project.sb3 --platform wiiu
```

# Supported platforms
3ds, gamecube, libretro, linux, macos, nds, ps4, psp, switch, vita, wasm, webos, wii, wiiu, windows