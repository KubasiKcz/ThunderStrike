<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import "../styles/app.css";
  import WindowControls from "../lib/components/layout/WindowControls.svelte";
  import SettingsPopup from "../lib/components/popups/SettingsPopup.svelte";
  import ToastContainer from "../lib/components/popups/ToastContainer.svelte";
  import Onboarding from "../lib/components/Onboarding.svelte";
  import { page } from "$app/stores";
  import { goto } from "$app/navigation";
  import { toastStore } from "../lib/stores/toastStore";
  import { i18n } from "../lib/stores/i18n.svelte";
  import { navStore } from "../lib/stores/navStore.svelte";
  import { initDiscordRPC, updateDiscordRPC } from "../lib/utils/discord";

  let { children } = $props();
  let isDark = $state(true);
  let isSettingsOpen = $state(false);
  let isWtRunning = $state(false);
  let gamePath = $state("");

  //Perma Onboard
  let showOnboarding = $state(false);

  function toggleTheme() {
    isDark = !isDark;
  }

  async function launchWT() {
  if (!gamePath || isWtRunning) return; 
  isWtRunning = true;
  
  await updateDiscordRPC("In Battle", "Playing War Thunder");

  try {
    const currentSteamUser = await invoke("get_setting", { key: "steamUsername" }) || "";
    await invoke("launch_wt", {
      path: gamePath, 
      steamUsername: currentSteamUser
    });
  } catch (e) {
    toastStore.add("Chyba při spouštění hry: " + e, "error");
  } finally {
    isWtRunning = false;
    await updateDiscordRPC("Idle", "Home page");
  }
}

  onMount(async () => {
    window.addEventListener("error", (e) => toastStore.add(e.message, "error", 5000));
    window.addEventListener("unhandledrejection", (e) => toastStore.add(String(e.reason), "error", 5000));

    await initDiscordRPC();

    try {
      const savedLang = await invoke("get_setting", { key: "lang" });
      if (savedLang === "cs" || savedLang === "en") {
        i18n.currentLanguage = savedLang;
      }
      
      try { gamePath = await invoke("get_setting", { key: "gameLocation" }) as string; } catch(e) {}

      const firstLaunch = await invoke("get_setting", { key: "firstLaunch" });
      showOnboarding = firstLaunch === "true" || firstLaunch === "[true]";
    } catch (e) {
      console.warn("Failed to load settings:", e);
      showOnboarding = true;
    }
  });
</script>

<main class={isDark ? "dark" : "light"}>
  <header data-tauri-drag-region>
    <div id="logo-wrapper" data-tauri-drag-region style="margin-left: 15px;">
      <img
        id="logo"
        src="/img/logo.png"
        alt="Logo"
        data-tauri-drag-region
      />
    </div>
    <h1 data-tauri-drag-region>ThunderStrike</h1>

    <div class="launch-container" data-tauri-drag-region>
       <button class="LaunchWT {isWtRunning ? 'running' : ''}" onclick={launchWT} disabled={isWtRunning}>
          <span class="launch-text">{isWtRunning ? i18n.t("Running...") : i18n.t("Launch WT")}</span>
          <img class="launch-bg" src="/img/button.png" alt="Launch WT" />
       </button>
    </div>

<WindowControls />
  </header>

  <aside>
    <button class="nav-btn" class:active={$page.url.pathname === '/'} onclick={() => goto("/")} title={i18n.t("Home Dashboard")}><img src="/img/icons/home.svg" alt="Home" /></button>
    <button class="nav-btn" class:active={$page.url.pathname === '/text'} onclick={() => goto("/text")} title={i18n.t("WT Text Editor")}><img src="/img/icons/text.svg" alt="Text" /></button>
    <button class="nav-btn" id="settings" onclick={() => (isSettingsOpen = true)} title={i18n.t("Settings")}>
      <img src="/img/icons/settings.svg" alt="Settings" />
    </button>
    <button class="nav-btn" class:active={$page.url.pathname === '/info'} id="info" onclick={() => goto("/info")} title={i18n.t("Software Info")}><img src="/img/icons/info.svg" alt="Info" /></button>
  </aside>

  <section>
    {@render children()}
  </section>

  <SettingsPopup bind:isOpen={isSettingsOpen} {isDark} {toggleTheme} />
  <ToastContainer />

  <Onboarding bind:isOpen={showOnboarding} />
</main>
