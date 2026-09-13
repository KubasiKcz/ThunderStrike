#!/usr/bin/env bash

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cd "$PROJECT_DIR" || {
  echo "Error: Failed to change directory to $PROJECT_DIR"
  exit 1
}

echo "Starting ThunderStrike on macOS from: $PROJECT_DIR"
npm run tauri dev