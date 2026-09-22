# ThunderStrike ⚡

A modern, reliable localization mod manager for **War Thunder**, built with **Tauri v2**, **Rust**, and **SvelteKit**.

ThunderStrike solves the classic problem War Thunder modders face: whenever Gaijin updates the game, modified localization CSV files cause missing text (`#hud_target_destroyed`, missing vehicle names, broken menus).

ThunderStrike uses a **Delta Patcher** architecture to keep your custom texts preserved across game updates without breaking new in-game vehicles or features.

---

## Key Features

- 🎯 **Delta Patcher Engine**: Saves changes as clean JSON diffs rather than modifying vanilla CSVs directly. Original game files are backed up automatically.
- 🔄 **Game Version Tracking**: Automatically reads the game's `version` file. When an update is detected, it guides you through safely regenerating fresh game files and re-applying your customizations with one click.
- ⭐ **Curated Starred View**: Instant access to the most frequently modified combat text (kill messages, HUD warnings, artillery prompts, crew knockouts) without digging through 30,000+ CSV entries.
- ↺ **Per-Row Revert**: Easily restore any customized string back to its vanilla text with a single click.
- 🔒 **Safe & Atomic Operations**: All writes are performed atomically with strict path sanitization to guarantee game files and configs are never corrupted.
- 🌐 **Multilingual Support**: Fully bilingual application interface (English & Čeština) with support for all War Thunder languages.
- ⚡ **Lightweight & Fast**: Built on Tauri v2 and optimized for smooth performance even with large localization files.

---

## Installation

### Windows

1. Download the latest `.exe` installer from the [Releases page](https://github.com/KubasiKcz/ThenderStrike/releases).
2. Run the installer and choose your preferred installation folder.
3. Launch **ThunderStrike.exe** — that's it. No extra setup needed.

> `config.json` and the `mods/` folder are created automatically on first launch.

### Linux (AppImage)

1. Download the latest `.AppImage` from the [Releases page](https://github.com/KubasiKcz/ThenderStrike/releases).
2. Make it executable:
   ```bash
   chmod +x ThunderStrike_*.AppImage
   ```
3. Place the AppImage in any folder you like and run it. ThunderStrike stores all its data (config, mods, backups) next to the AppImage file.

### Linux (Arch / Manjaro)

A `PKGBUILD` is available in [`packaging/arch/`](packaging/arch/). Install with:

```bash
cd packaging/arch
makepkg -si
```

---

## Building from Source

### Prerequisites
- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://rust-lang.org/) (latest stable)

### Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/KubasiKcz/ThenderStrike.git
   cd ThenderStrike
   ```

2. Install frontend dependencies:
   ```bash
   npm install
   ```

3. Run the development build:
   ```bash
   npm run tauri dev
   ```

> No manual `config.json` setup is required — the app creates and auto-populates it on first launch, including an automatic scan for your War Thunder installation.

---

## Directory Layout

```
ThunderStrike/
├── config.json              # Local configuration (game directory path, toggle)
├── mods/
│   └── localization/
│       ├── starred.txt      # Curated list of starred keys
│       ├── diffs/           # User JSON diffs (<file>.json)
│       └── backups/         # Pristine vanilla CSV backups (<file>.csv)
├── src/                     # SvelteKit frontend & design system
└── src-tauri/               # Rust backend (Tauri commands, CSV patcher, BLK parser)
```

---

## Shortcuts

- `Ctrl + S`: Save pending modifications and apply them directly to game files.
- `Ctrl + F`: Focus the key/text search filter.

---

## License

This project is licensed under the [GNU General Public License v3.0](LICENSE).

---

## Disclaimer

THIS IS NOT AN OFFICIAL ITEM OR PRODUCT BY WAR THUNDER OR GAIJIN ENTERTAINMENT.
This project is an independent community tool and is not affiliated with, endorsed by, or sponsored by Gaijin Entertainment or War Thunder in any way.
