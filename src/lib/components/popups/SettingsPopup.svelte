<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { toastStore } from "../../stores/toastStore";
  import { i18n, AVAILABLE_LANGUAGES } from "../../stores/i18n.svelte";
  
  let { isOpen = $bindable(false), isDark, toggleTheme } = $props();
  let gameLocation = $state("");
  let GameLang = $state("");
  let Username = $state("");
  let steamUsername = $state("");
  let language = $state("en");
  let foundLanguages: string[] = $state([]);
  let targetWtLang = i18n.currentLanguage === "cs" ? "Czech" : "English";
  let foundMatch = foundLanguages.find(
          (l) => l.toLowerCase() === targetWtLang.toLowerCase(),
        );

        if (foundMatch) {
          GameLang = foundMatch;
        } else if (foundLanguages.length > 0) {
          GameLang = foundLanguages[0];
        }

  async function saveSettings() {
    try {
      await invoke("save_setting", {
        key: "gameLocation",
        value: gameLocation,
      });
      await invoke("save_setting", {
        key: "lang",
        value: i18n.currentLanguage,
      });
      await invoke("save_setting", { key: "gameLang", value: GameLang });
      await invoke("save_setting", { key: "wtUsername", value: Username });
      await invoke("save_setting", {
        key: "steamUsername",
        value: steamUsername,
      });
      await invoke("save_setting", { key: "firstLaunch", value: "false" });

      const { emit } = await import("@tauri-apps/api/event");
      await emit("onboarding-finished");

      isOpen = false;
    } catch (e) {
      console.error("Failed to save location:", e);
      toastStore.add(String(e), "error");
    }
  }

  async function loadSettings() {
    try {
      gameLocation = await invoke("get_setting", { key: "gameLocation" }) || "";
      GameLang = await invoke("get_setting", { key: "gameLang" }) || "en";
      Username = await invoke("get_setting", { key: "wtUsername" }) || "";
      steamUsername = await invoke("get_setting", { key: "steamUsername" }) || "";
      language = await invoke("get_setting", { key: "lang" }) || "en";

    } catch (e) {
      console.error("Chyba při načítání:", e);
    }
  }

  $effect(() => {
    if (isOpen) {
      loadSettings();
    }
  });

  function closePopup() {
    isOpen = false;
  }

  function toast() {
    toastStore.add("Test info popup", "info");
    toastStore.add("Test success popup", "success");
    toastStore.add("Test warning popup", "warning");
    toastStore.add("Test error popup", "error");

    const loadingId = toastStore.add("Test loading popup...", "loading", 0);
    setTimeout(() => {
      toastStore.updateToast(loadingId, "Test loading completed!", "success", 3000);
    }, 3000);
  }

  async function factoryReset() {
    if (
      confirm(
        "Are you sure you want to completely reset the program? (It will delete settings, but not your modified texts from the game)",
      )
    ) {
      try {
        await invoke("reset_settings");
        toastStore.add("Reset successful", "success");
        localStorage.clear();
        window.location.reload();
      } catch (e) {
        toastStore.add("Reset failed: " + e, "error");
      }
    }
  }
</script>

{#if isOpen}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    class="popup-overlay"
    onclick={closePopup}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
  >
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="popup-content" onclick={(e) => e.stopPropagation()}>
      <div class="popup-header">
        <h2>{i18n.t("Settings")}</h2>
        <button
          class="close-btn"
          onclick={closePopup}
          aria-label="Close"
          title="Close"
        >
          <img src="/img/icons/close.svg" alt="Close" class="svg" />
        </button>
      </div>
      <div class="popup-body">
        <h3>{i18n.t("General")}</h3>
        <div class="setting-item">
          <span>{i18n.t("Dark Mode")}</span>
          <button
            class="toggle-btn {isDark ? 'active' : ''}"
            onclick={toggleTheme}
            aria-label="Toggle dark mode">
            <div class="toggle-circle"></div>
          </button>
        </div>
        <div class="setting-item">
          <span>{i18n.t("App Language")}</span>
          <select bind:value={language}>
            {#each AVAILABLE_LANGUAGES as lang}
              <option value={lang.code}>{i18n.t(lang.name)}</option>
            {/each}
          </select>
        </div>
        <div class="setting-item">
          <span>{i18n.t("Game language")}</span>
          <select bind:value={GameLang}>
            {#each foundLanguages as lang}
              <option value={lang}>{lang}</option>
            {/each}
          </select>
        </div>
        <div class="setting-item">
          <span>{i18n.t("WarThunder username")}</span>
          <input type="text" bind:value={Username} />
        </div>
        <div class="setting-item">
          <span>{i18n.t("Steam username")}</span>
          <input type="text" bind:value={steamUsername} />
        </div>
        <div class="setting-item">
          <span>{i18n.t("Game location")}</span>
          <input type="text" bind:value={gameLocation} />
        </div>
        <div class="setting-item">
          <span>{i18n.t("Save settings")}</span>
          <button onclick={saveSettings} class="save-button">{i18n.t("Save Settings")}</button>
        </div>
        <h3>{i18n.t("Dev Testing")}</h3>
        <div class="setting-item">
          <span>{i18n.t("Toast Test")}</span>
          <button class="button" onclick={toast} aria-label="Toast Test">{i18n.t("Test Toasts")}</button>
        </div>

        <div class="setting-item" style="margin-top: 20px;">
          <span style="color: #ff4c4c;">{i18n.t("Factory Reset")}</span>
          <button
            class="button"
            style="background-color: #c0392b; color: white;"
            onclick={factoryReset}
          >
            {i18n.t("Reset program (Hard Reset)")}
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
.popup-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
  backdrop-filter: blur(2px);
}

.popup-content {
  background-color: var(--bg-color);
  color: var(--text-color);
  width: 400px;
  max-width: 90vw;
  border-radius: 12px;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.3);
  border: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.popup-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 15px 20px;
  border-bottom: 1px solid var(--border-color);
}

.popup-header h2 {
  margin: 0;
  font-size: 1.25rem;
  font-weight: 600;
}

.close-btn {
  background: transparent;
  border: none;
  color: var(--icon-color);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 5px;
  border-radius: 6px;
  transition: background-color 0.2s, color 0.2s;
}

.close-btn:hover {
  background-color: var(--hover-bg);
  color: var(--hover-text);
}

.popup-body {
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.setting-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 0;
  font-size: 1rem;
}

.toggle-btn {
  width: 44px;
  height: 24px;
  background-color: var(--border-color);
  border: none;
  border-radius: 12px;
  position: relative;
  cursor: pointer;
  transition: background-color 0.3s;
  padding: 0;
}

.toggle-btn.active {
  background-color: #3b82f6;
}

.toggle-circle {
  width: 18px;
  height: 18px;
  background-color: white;
  border-radius: 50%;
  position: absolute;
  top: 3px;
  left: 3px;
  transition: transform 0.3s;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.toggle-btn.active .toggle-circle {
  transform: translateX(20px);
}

.button {
  background-color: var(--border-color);
  color: var(--text-color);
  border: none;
  border-radius: 6px;
  padding: 8px 12px;
  cursor: pointer;
  font-size: 0.95rem;
  transition: background-color 0.2s;
}

.button:hover {
  background-color: var(--hover-bg);
}

</style>
