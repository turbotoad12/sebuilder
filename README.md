

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
| Platform | Status |
|--|--|
| [3ds](ca://s?q=Edit_3ds_platform_status) | Full |
| [gamecube](ca://s?q=Edit_gamecube_platform_status) | ⚠️ Untested |
| [libretro](ca://s?q=Edit_libretro_platform_status) | ⚠️ Untested |
| [linux](ca://s?q=Edit_linux_platform_status) | ⚠️ Untested |
| [macOS](ca://s?q=Edit_macOS_platform_status) | ⚠️ Untested |
| [nds](ca://s?q=Edit_nds_platform_status) | ⚠️ Untested |
| [ps4](ca://s?q=Edit_ps4_platform_status) | ⚠️ Untested |
| [psp](ca://s?q=Edit_psp_platform_status) | ⚠️ Untested |
| [switch](ca://s?q=Edit_switch_platform_status) | ⚠️ Untested |
| [vita](ca://s?q=Edit_vita_platform_status) | ⚠️ Untested |
| [wasm](ca://s?q=Edit_wasm_platform_status) | ⚠️ Untested |
| [webos](ca://s?q=Edit_webos_platform_status) | ⚠️ Untested |
| [wii](ca://s?q=Edit_wii_platform_status) | ⚠️ Untested |
| [wiiu](ca://s?q=Edit_wiiu_platform_status) | ⚠️ Untested |
| [windows](ca://s?q=Edit_windows_platform_status) | ⚠️ Untested |
