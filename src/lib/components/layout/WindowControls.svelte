<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';

  export function minimizeWindow() {
    getCurrentWindow().minimize();
  }

  export function toggleMaximizeWindow() {
    getCurrentWindow().toggleMaximize();
  }

  export function closeWindow() {
    getCurrentWindow().close();
  }
</script>

<div class="window-controls">
  <button
    class="win-btn"
    onclick={minimizeWindow}
    aria-label="Minimize"
    title="Minimize"
  >
  <img src="/img/icons/minimize.svg" alt="Minimize" class="svg">
  </button>
  <button
    class="win-btn"
    onclick={toggleMaximizeWindow}
    aria-label="Maximize"
    title="Maximize"
  >
  <img src="/img/icons/maximize.svg" alt="Maximize" class="svg">
  </button>
  <button
    class="win-btn close"
    onclick={closeWindow}
    aria-label="Close"
    title="Close"
  >
  <img src="/img/icons/close.svg" alt="Close" class="svg">
  </button>
</div>

<style>
.window-controls {
  display: flex;
  -webkit-app-region: no-drag;
}

.win-btn {
  width: 45px;
  background: transparent;
  border: none;
  cursor: pointer;
  color: var(--icon-color);
  display: flex;
  justify-content: center;
  align-items: center;
  transition: background 0.2s, color 0.2s;
  outline: none;
}

.win-btn:hover {
  background-color: var(--hover-bg);
  color: var(--hover-text);
}

.win-btn.close:hover {
  background-color: #e81123;
  color: white;
}
</style>
