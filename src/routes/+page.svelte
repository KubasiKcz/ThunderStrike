<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { i18n } from "../lib/stores/i18n.svelte";
  import { toastStore } from "../lib/stores/toastStore";
  import { navStore } from "../lib/stores/navStore.svelte";

  let wtUsername = $state("");
  let profileData: any = $state(null);
  let isFetching = $state(false);
  let lastUpdated = $state("");
  let activeTab = $state("all");
  let configMods = $state({
    localization: false,
    sound: false,
    hangar: false,
  });
  let gamePath = $state("");

  async function loadProfile() {
    try {
      wtUsername = await invoke("get_setting", { key: "wtUsername" });
      gamePath = await invoke("get_setting", { key: "gameLocation" });

      let cached = localStorage.getItem("wtProfileCache");
      let cachedTime = localStorage.getItem("wtProfileLastUpdate");

      configMods.localization =
        localStorage.getItem("wtMod_localization") === "true";
      configMods.sound = localStorage.getItem("wtMod_sound") === "true";
      configMods.hangar = localStorage.getItem("wtMod_hangar") === "true";

      if (cached && cachedTime) {
        profileData = JSON.parse(cached);
        lastUpdated = cachedTime;
      }

      if (wtUsername && !cached) {
        await refreshProfile();
      }
    } catch (e) {
      console.log("Chyba při inicializaci profilu", e);
    }
  }

  async function toggleMod(type: "localization" | "sound" | "hangar") {
    if (!gamePath) {
      toastStore.add("Game location not set!", "error");
      return;
    }
    try {
      const newState = !configMods[type];
      await invoke("toggle_mod_granular", {
        gamePath,
        modType: type,
        enabled: newState,
      });
      configMods[type] = newState;
      localStorage.setItem(`wtMod_${type}`, String(newState));

      let msg = "";
      if (type === "localization")
        msg = newState ? "Localization enabled" : "Localization disabled";
      if (type === "sound")
        msg = newState ? "Sound mods enabled" : "Sound mods disabled";
      if (type === "hangar")
        msg = newState ? "Hangar enabled" : "Hangar disabled";

      toastStore.add(msg + " (Restart game)", "success");
    } catch (e) {
      toastStore.add(String(e), "error");
    }
  }

  async function refreshProfile() {
    if (!wtUsername) {
      toastStore.add(
        "Username not found - please complete onboarding.",
        "error",
      );
      return;
    }

    isFetching = true;
    try {
      const { listen } = await import("@tauri-apps/api/event");

      const resultPromise = new Promise<any>((resolve, reject) => {
        let unlistenFn: (() => void) | undefined;

        const timeout = setTimeout(() => {
          if (unlistenFn) unlistenFn();
          reject(new Error("Profile fetch timed out (20s)"));
        }, 20000);

        listen("wt-profile-result", (event: any) => {
          clearTimeout(timeout);
          if (unlistenFn) unlistenFn();
          resolve(event.payload);
        }).then((u) => {
          unlistenFn = u;
        });
      });

      await invoke("scrape_wt_profile", { nick: wtUsername });

      const res: any = await resultPromise;

      if (res.error) {
        toastStore.add(res.error, "error");
        return;
      }

      let calculatedStats = res.stats || null;
      if (calculatedStats) {
        let all = { wins: 0, missions: 0, deaths: 0, lions: 0, air: 0, ground: 0, naval: 0, timeHours: 0 };
        for (const mode of ["arcade", "realistic", "simulator"]) {
          const m = calculatedStats[mode];
          if (!m) continue;
          all.wins += parseInt((m.wins || "0").toString().replace(/\D/g, "")) || 0;
          all.missions += parseInt((m.missions || "0").toString().replace(/\D/g, "")) || 0;
          all.deaths += parseInt((m.deaths || "0").toString().replace(/\D/g, "")) || 0;
          all.lions += parseInt((m.lions || "0").toString().replace(/\D/g, "")) || 0;
          all.air += parseInt((m.air || "0").toString().replace(/\D/g, "")) || 0;
          all.ground += parseInt((m.ground || "0").toString().replace(/\D/g, "")) || 0;
          all.naval += parseInt((m.naval || "0").toString().replace(/\D/g, "")) || 0;
          
          let t = (m.time || "").toString();
          let daysMatch = t.match(/(\d+)\s*d/);
          let hoursMatch = t.match(/(\d+)\s*h/);
          let days = daysMatch ? parseInt(daysMatch[1]) : 0;
          let hours = hoursMatch ? parseInt(hoursMatch[1]) : 0;
          all.timeHours += (days * 24) + hours;
        }
        let ratio = all.missions > 0 ? Math.round((all.wins / all.missions) * 100) + "%" : "0%";
        let days = Math.floor(all.timeHours / 24);
        let hours = all.timeHours % 24;
        let timeStr = days > 0 ? `${days}d ${hours}h` : `${hours}h`;

        const formatNum = (num: number) => num.toLocaleString("en-US");

        calculatedStats.all = {
          wins: formatNum(all.wins),
          missions: formatNum(all.missions),
          ratio: ratio,
          deaths: formatNum(all.deaths),
          lions: formatNum(all.lions),
          time: timeStr,
          air: formatNum(all.air),
          ground: formatNum(all.ground),
          naval: formatNum(all.naval)
        };
      }

      profileData = {
        status: "ok",
        nick: res.nick || wtUsername,
        level: res.lvl || null,
        registration_date: res.date || null,
        avatar_url: res.avatar || null,
        stats: calculatedStats,
      };

      lastUpdated = new Date().toLocaleString();
      localStorage.setItem("wtProfileCache", JSON.stringify(profileData));
      localStorage.setItem("wtProfileLastUpdate", lastUpdated);
      toastStore.add("Profile updated!", "success");
    } catch (e) {
      toastStore.add(String(e), "error");
    } finally {
      isFetching = false;
    }
  }

  onMount(() => {
    loadProfile();

    let unlisten: (() => void) | undefined;

    import("@tauri-apps/api/event").then(({ listen }) => {
      listen("onboarding-finished", () => {
        loadProfile();
      }).then((u) => {
        unlisten = u;
      });
    });

    return () => {
      if (unlisten) unlisten();
    };
  });
</script>

<div class="home-container">
  <div class="dashboard-layout">
    <div class="main-content">
      {#if profileData}
        <div class="profile-card">
          <div class="avatar-container">
            <img
              class="avatar"
              src={profileData.avatar_url || "/img/logo.png"}
              alt="Avatar"
            />
          </div>
          <div class="profile-details">
            <h2 class="nickname">{profileData.nick}</h2>
            <p class="lvl">{i18n.t("Level")} {profileData.level || "?"}</p>
            <p class="reg-date">
              {i18n.t("Registered:")}
              {profileData.registration_date || "?"}
            </p>

            <div class="update-section">
              <button
                class="update-btn"
                onclick={refreshProfile}
                disabled={isFetching}
              >
                {isFetching ? i18n.t("Updating...") : i18n.t("Sync Profile")}
              </button>
              <span class="last-updated">{lastUpdated}</span>
            </div>
          </div>
        </div>

        <div class="stats-container">
          <div class="stats-nav">
            <button
              class="nav-item"
              class:active={activeTab === "all"}
              onclick={() => (activeTab = "all")}>{i18n.t("All")}</button
            >
            <button
              class="nav-item"
              class:active={activeTab === "arcade"}
              onclick={() => (activeTab = "arcade")}>{i18n.t("Arcade")}</button
            >
            <button
              class="nav-item"
              class:active={activeTab === "realistic"}
              onclick={() => (activeTab = "realistic")}
              >{i18n.t("Realistic")}</button
            >
            <button
              class="nav-item"
              class:active={activeTab === "simulator"}
              onclick={() => (activeTab = "simulator")}
              >{i18n.t("Simulator")}</button
            >
          </div>

          <div class="stats-grid">
            {#if profileData.stats && profileData.stats[activeTab]}
              {@const s = profileData.stats[activeTab]}
              <div class="stat-box">
                <span>{i18n.t("Victories")}</span><strong>{s.wins}</strong>
              </div>
              <div class="stat-box">
                <span>{i18n.t("Missions")}</span><strong>{s.missions}</strong>
              </div>
              <div class="stat-box">
                <span>{i18n.t("Win Ratio")}</span><strong>{s.ratio}</strong>
              </div>
              <div class="stat-box">
                <span>{i18n.t("Deaths")}</span><strong>{s.deaths}</strong>
              </div>
              <div class="stat-box">
                <span>{i18n.t("Lions")}</span><strong>{s.lions}</strong>
              </div>
              <div class="stat-box">
                <span>{i18n.t("Play Time")}</span><strong>{s.time}</strong>
              </div>
              <div class="stat-box">
                <span>{i18n.t("Air Targets")}</span><strong>{s.air}</strong>
              </div>
              <div class="stat-box">
                <span>{i18n.t("Ground Targets")}</span><strong
                  >{s.ground}</strong
                >
              </div>
              <div class="stat-box">
                <span>{i18n.t("Naval Targets")}</span><strong>{s.naval}</strong>
              </div>
            {:else}
              <p class="no-stats">
                {i18n.t("No statistics available for this mode.")}
              </p>
            {/if}
          </div>
        </div>
      {:else}
        <div class="no-profile">
          {#if !wtUsername}
            <p>{i18n.t("No nickname found. Please complete onboarding.")}</p>
          {:else}
            <div class="profile-card">
              <div class="profile-details">
                <p>{i18n.t("Waiting for profile data...")}</p>
                <button
                  class="update-btn"
                  onclick={refreshProfile}
                  disabled={isFetching}
                >
                  {isFetching
                    ? i18n.t("Loading...")
                    : i18n.t("Fetch Profile Now")}
                </button>
              </div>
            </div>
          {/if}
        </div>
      {/if}
    </div>

    <div class="side-panel">
      <div class="mods-card">
        <h3>{i18n.t("Game Mods")}</h3>
        <p class="mods-desc">
          {i18n.t("Configure custom game modifications.")}
        </p>

        <div class="mod-switches">
          <div class="mod-switch-row">
            <span class="mod-title">🚀 {i18n.t("Localization")}</span>
            <label class="switch">
              <input
                type="checkbox"
                checked={configMods.localization}
                onchange={() => toggleMod("localization")}
              />
              <span class="slider"></span>
            </label>
          </div>

          <div class="mod-switch-row">
            <span class="mod-title">🔊 {i18n.t("Sound Mods")}</span>
            <label class="switch">
              <input
                type="checkbox"
                checked={configMods.sound}
                onchange={() => toggleMod("sound")}
              />
              <span class="slider"></span>
            </label>
          </div>

          <div class="mod-switch-row">
            <span class="mod-title">🏠 {i18n.t("Custom Hangar")}</span>
            <label class="switch">
              <input
                type="checkbox"
                checked={configMods.hangar}
                onchange={() => toggleMod("hangar")}
              />
              <span class="slider"></span>
            </label>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>
