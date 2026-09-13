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

## Getting Started

### Prerequisites
- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://rust-lang.org/) (latest stable)

### Setup
1. Clone the repository:
   ```bash
   git clone https://github.com/KubasiKcz/ThenderStrike.git
   cd ThenderStrike
   ```

2. Copy the example configuration:
   ```bash
   cp config.example.json config.json
   ```
   Set `"war_thunder_files"` in `config.json` to your War Thunder directory (e.g. `C:\WarThunder` or `~/.steam/steam/steamapps/common/War Thunder`).

3. Install frontend dependencies:
   ```bash
   npm install
   ```

4. Run the development build:
   ```bash
   npm run tauri dev
   ```

---

## Shortcuts

- `Ctrl + S`: Save pending modifications and apply them directly to game files.
- `Ctrl + F`: Focus the key/text search filter.

---

## License

This project is licensed under the [MIT License](LICENSE).
