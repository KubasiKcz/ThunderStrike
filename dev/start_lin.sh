#!/usr/bin/env bash

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cd "$PROJECT_DIR" || {
  echo "Error: Failed to change directory to $PROJECT_DIR"
  exit 1
}

echo "Starting ThunderStrike for Linux from: $PROJECT_DIR on GDK_BACKEND $GDK_BACKEND and WEBKIT_DISABLE_DMABUF_RENDERER $WEBKIT_DISABLE_DMABUF_RENDERER"

export GDK_BACKEND=x11
export WEBKIT_DISABLE_DMABUF_RENDERER=1

npm run tauri dev