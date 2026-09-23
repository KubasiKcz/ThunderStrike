![GitHub Repo Banner](https://ghrb.waren.build/banner?header=%21%5Bthunderstrike%5D+ThunderStrike&subheader=A+portable%2C+open-source+mod+manager+for+War+Thunder.&bg=00000000&color=FFFFFF&headerfont=JetBrains+Mono&subheaderfont=JetBrains+Mono&support=false)

---

## ✨ Current Features

### System
| Feature | Description |
|---|---|
| 🔍 **Auto Game Detection** | On first launch, automatically scans common Steam library locations on Windows and Linux |
| 🔒 **Safe & Atomic Writes** | Strict path sanitization and atomic writes — game files and configs are never corrupted |
| ⚡ **Lightweight** | Native Tauri binary with minimal resource usage — no heavy Electron runtime |
| 🌐 **Multilingual UI** | App interface in **English** & **Czech** |

### Localization Editor
| Feature | Description |
|---|---|
| 🎯 **Delta Patcher** | Saves your edits as clean JSON diffs — original vanilla CSVs are never modified directly |
| 🔄 **Auto-Update Detection** | Reads the game's `version` file; one-click re-apply of all your changes after a WT patch |
| ⭐ **Starred Keys** | Curated quick-access list of common strings (kill messages, HUD alerts, crew calls, artillery prompts…) |
| ↺ **Per-Row Revert** | Restore any single string back to vanilla with one click — no need to redo the whole file |
| 📥 **Datamine Fetch** | Download pristine vanilla CSVs from [gszabi99/War-Thunder-Datamine](https://github.com/gszabi99/War-Thunder-Datamine) — no game launch needed |

---

## 🗺️ Planned Features

*(Not in strict order)*

- [ ] **Fast game launch** — launch War Thunder directly from ThunderStrike
- [ ] **Skin manager** — browse, install and toggle custom vehicle skins & decals
- [ ] **Sight manager** — manage custom gunsight packs
- [ ] **Custom sounds manager** — install and switch sound mods
- [ ] **Hangar manager** — custom hangar environments
- [ ] **User missions manager** — organize and run custom missions (which can also contain custom vehicles)
- [ ] **Discord Rich Presence** — show current activity in Discord
- [ ] **Replay archiver** — auto-save and tag game replays before they expire
- [ ] **macOS support**
- [ ] **More in-app language support**

---

## ⌨️ Shortcuts

| Context | Shortcut | Action |
|---|---|---|
| **Localization editor** | `Ctrl + S` | Save pending edits and apply them to game files immediately |
| **Localization editor** | `Ctrl + F` | Jump to the search / filter bar |

---

## 📦 Installation

> [!NOTE]
> **ThunderStrike is fully portable.** There is no installer and nothing is written to the registry — just drop it in a folder and run.  
> All data (`config.json`, diffs, backups) is stored locally next to the executable.

> **System Compatibility & Testing:**  
> - Developed on **Arch Linux** with [CachyOS](https://cachyos.org/)  
> - Tested on **Windows 11**

### 🪟 Windows

1. Download **`ThunderStrike_x64.exe`** from the [Releases page](https://github.com/KubasiKcz/ThenderStrike/releases/latest).
2. Place it in any folder you prefer (e.g. `C:\Users\You\Apps\ThunderStrike\`).
3. Double-click to run — that's it, it setups itself automatically.

### 🐧 Linux (AppImage) — recommended

1. Download **`ThunderStrike_x86_64.AppImage`** from the [Releases page](https://github.com/KubasiKcz/ThenderStrike/releases/latest).
2. Place it in a dedicated folder, for example:
   ```bash
   ~/Apps/ThunderStrike/
   ```
3. Make it executable and run:
   ```bash
   chmod +x ThunderStrike_*.AppImage
   ./ThunderStrike_*.AppImage
   ```

<details>
<summary><b>🐧 Linux (Arch / Manjaro PKGBUILD)</b></summary>

A `PKGBUILD` is included in [`packaging/arch/`](packaging/arch/):

```bash
cd packaging/arch
makepkg -si
```

</details>

<details>
<summary><b>❌️🍎 macOS (not yet supported)</b></summary>

macOS builds are not available **yet** — planned for a **future release**.

</details>

---

## 📂 Portable Directory Layout

When running ThunderStrike as an end-user, this is the clean folder structure created next to your executable:

```
ThunderStrike/
├── ThunderStrike.exe (or .AppImage)
├── config.json              # Local config & detected WT path (auto-generated)
└── mods/                    # Mod storage
    └── localization/         # Localization mods storage
        ├── starred.txt          # Starred keys list
        ├── diffs/               # Your JSON diffs (<file>.json)
        └── backups/             # Pristine vanilla CSV backups (<file>.csv)
```

---

## 🔧 Building from Source

### Prerequisites

- [Node.js](https://nodejs.org/) v18+
- [Rust](https://rust-lang.org/) (latest stable via [rustup](https://rustup.rs/))
- On Linux: `gtk3`, `webkit2gtk-4.1`, `libsoup3`, `pkg-config`

### Steps

```bash
# 1. Clone
git clone https://github.com/KubasiKcz/ThunderStrike.git
cd ThunderStrike

# 2. Install frontend deps
npm install

# 3. Dev build (hot-reload)
npm run tauri dev

# 4. Production build
npm run bundle
```

---

## 🎁 Special Thanks

- [gszabi99](https://github.com/gszabi99) for [War-Thunder-Datamine](https://github.com/gszabi99/War-Thunder-Datamine)
- [Lucide](https://lucide.dev/) for the open-source icon library
- [Waren Gonzaga](https://github.com/warengonzaga) for [GitHub Repo Banner Creator](https://ghrb.waren.build)

---

## ⚖️ Legal

### Disclaimer

**THIS IS NOT AN OFFICIAL GAIJIN ENTERTAINMENT OR WAR THUNDER PRODUCT.**  
ThunderStrike is an independent community tool with no affiliation with, endorsement by, or sponsorship from Gaijin Entertainment or the War Thunder franchise. Use at your own risk.

### License

This project is licensed under the **[GNU General Public License v3.0](LICENSE)** — free to use, modify, and distribute under the same terms.

### AI Assistance

Parts of this project were developed with the assistance of AI coding tools (Google Antigravity IDE / Gemini & Claude). All generated code was reviewed, tested, and modified by the project maintainer. Im still learning so please be nice :(
