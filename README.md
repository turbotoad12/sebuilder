
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

# Usage

Create a project.

```bash
sebuilder new myproject
```

Build the project. (defaults to 3ds)

```bash
sebuilder build
```

Build with custom platform.

```bash
sebuilder build --platform wiiu
```

## Customizing Projects

Inside the assets folder, you’ll find all the platform-specific icons and graphics.
For example, the icon.png file is used as the app icon on consoles like the 3DS.

- You can replace these images with your own graphics.
- ⚠️ Do not change the image sizes, names, or extensions.
The images must keep their exact dimensions and same filenames (e.g. icon.png).
- GIMP is great for these kinds of things, but you can use any image editing software you want.

To modify banners or special icons, open the subfolder with the name of your target console (e.g., 3ds/, wiiu/, etc.).

For more information reference the [Scratch Everywhere! Documentation](https://scratcheverywhere.github.io/docker#step-5-customize-icons-and-graphics)
# Supported platforms
3ds, gamecube, libretro, linux, macOS, nds, ps4, psp, switch, vita, wasm, webos, wii, wiiu, windows.