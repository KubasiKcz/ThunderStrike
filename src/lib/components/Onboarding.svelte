<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { toastStore } from "../stores/toastStore";
  import { i18n, AVAILABLE_LANGUAGES } from "../stores/i18n.svelte";
  
  let { isOpen = $bindable(false) } = $props();

  let currentStep = $state(1);
  let tempGameLang = $state("");
  let tempLocation = $state("");
  let tempUsername = $state("");
  let tempSteamUsername = $state("");
  let foundLanguages: string[] = $state([]);
  let isWaitingForLang = $state(false);
  let showSteamInput = $state(false);

  async function searchWtPath() {
    if (!tempLocation.trim()) {
      toastStore.add(i18n.t("Please set the game location"), "error");
      return;
    }

    try {
      const response: any = await invoke("find_wt_path", {
        userInput: tempLocation,
      });
      if (response.status === "ok") {
        tempLocation = response.path;
        foundLanguages = response.languages;
        isWaitingForLang = false;

        let targetWtLang = i18n.currentLanguage === "cs" ? "Czech" : "English";
        let foundMatch = foundLanguages.find(
          (l) => l.toLowerCase() === targetWtLang.toLowerCase(),
        );

        if (foundMatch) {
          tempGameLang = foundMatch;
        } else if (foundLanguages.length > 0) {
          tempGameLang = foundLanguages[0];
        }

        currentStep = 3;
      } else if (response.status === "waiting_for_lang") {
        tempLocation = response.path;
        isWaitingForLang = true;
      } else {
        toastStore.add(response.msg || "Lang folder not found.", "error");
      }
    } catch (e) {
      toastStore.add(String(e), "error");
    }
  }

  async function nextStep() {
    try {
      if (currentStep === 1) {
        await invoke("save_setting", {
          key: "lang",
          value: i18n.currentLanguage,
        });
        currentStep = 2;
      } else if (currentStep === 2) {
        await searchWtPath();
      } else if (currentStep === 3) {
        currentStep = 4;
      }
    } catch (e) {
      console.error("Failed to save setting:", e);
      toastStore.add(String(e), "error");
    }
  }

  async function finishOnboarding() {
    try {
      await invoke("save_setting", {
        key: "gameLocation",
        value: tempLocation,
      });
      await invoke("save_setting", {
        key: "lang",
        value: i18n.currentLanguage,
      }); // ensure saved
      await invoke("save_setting", { key: "gameLang", value: tempGameLang });
      await invoke("save_setting", { key: "wtUsername", value: tempUsername });
      await invoke("save_setting", {
        key: "steamUsername",
        value: tempSteamUsername,
      });
      await invoke("save_setting", { key: "firstLaunch", value: "false" });

      // Emit event to notify other pages
      const { emit } = await import("@tauri-apps/api/event");
      await emit("onboarding-finished");

      isOpen = false;
    } catch (e) {
      console.error("Failed to save location:", e);
      toastStore.add(String(e), "error");
    }
  }
</script>

{#if isOpen}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div class="popup-overlay" role="dialog" aria-modal="true" tabindex="-1">
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="popup-content" onclick={(e) => e.stopPropagation()}>
      <div class="popup-header">
        <h2>{i18n.t("Welcome to ThunderStrike!")}</h2>
        <!-- Intentionally no close button, as onboarding is a required step -->
      </div>

      <div class="popup-body">
        {#if currentStep === 1}
          <div class="step-content">
            <h3>{i18n.t("Step 1 of 4: App Language")}</h3>
            <p>{i18n.t("Which language should the application UI use?")}</p>
            <select class="custom-input" bind:value={i18n.currentLanguage}>
              {#each AVAILABLE_LANGUAGES as lang}
                <option value={lang.code}>{i18n.t(lang.name)}</option>
              {/each}
            </select>
            <button class="button next-btn" onclick={nextStep}
              >{i18n.t("Continue")}</button
            >
          </div>
        {:else if currentStep === 2}
          <div class="step-content">
            <h3>{i18n.t("Step 2 of 4: Game Location")}</h3>
            {#if isWaitingForLang}
              <div
                class="wait-lang-box"
                style="background: rgba(243, 156, 18, 0.1); padding: 15px; border-radius: 8px; border: 1px solid #f39c12; margin-bottom: 15px;"
              >
                <p style="color: #f39c12; font-weight: bold; margin-top: 0;">
                  {i18n.t("Lang folder missing!")}
                </p>
                <p style="font-size: 0.9rem;">
                  {i18n.t(
                    "Please START and then CLOSE War Thunder to generate the required files.",
                  )}
                </p>
                <div class="btn-group" style="margin-top: 15px;">
                  <button class="button next-btn" onclick={searchWtPath}
                    >{i18n.t("Done / Re-check")}</button
                  >
                </div>
              </div>
            {:else}
              <p>
                {i18n.t(
                  "The disk location where the game installation is found:",
                )}
              </p>
              <input
                type="text"
                class="custom-input"
                bind:value={tempLocation}
                placeholder={i18n.t(
                  "e.g. C:/Program Files/steam/steamapps/common/War Thunder",
                )}
              />
              <div class="btn-group">
                <button class="button back-btn" onclick={() => currentStep--}
                  >{i18n.t("Back")}</button
                >
                <button class="button next-btn" onclick={nextStep}
                  >{i18n.t("Continue")}</button
                >
              </div>
            {/if}
          </div>
        {:else if currentStep === 3}
          <div class="step-content">
            <h3>{i18n.t("Step 3 of 4: In-Game Language")}</h3>
            <p>{i18n.t("Which language should be used inside the game?")}</p>
            <select class="custom-input" bind:value={tempGameLang}>
              {#each foundLanguages as lang}
                <option value={lang}>{lang}</option>
              {/each}
            </select>
            <div class="btn-group">
              <button class="button back-btn" onclick={() => currentStep--}
                >{i18n.t("Back")}</button
              >
              <button class="button next-btn" onclick={nextStep}
                >{i18n.t("Continue")}</button
              >
            </div>
          </div>
        {:else if currentStep === 4}
          <div class="step-content">
            <h3>{i18n.t("Step 4 of 4: Usernames")}</h3>
            <p>
              {i18n.t(
                "Enter your exact War Thunder nickname (Warning: It is Case-Sensitive!).",
              )}
            </p>
            <input
              type="text"
              class="custom-input"
              bind:value={tempUsername}
              placeholder={i18n.t("WarThunder username")}
            />
            <p>{i18n.t("Do you play on Steam?")}</p>
            <label class="switch">
              <input type="checkbox" bind:checked={showSteamInput} />
              <span class="slider"></span>
            </label>

            {#if showSteamInput}
              <div id="steamusername">
                <p>
                  {i18n.t(
                    "Enter your steam username or ID (Warning: It is Case-Sensitive!).",
                  )}
                </p>
                <input
                  type="text"
                  class="custom-input"
                  bind:value={tempSteamUsername}
                  placeholder={i18n.t("Steam username")}
                />
              </div>
            {/if}

            <div class="btn-group">
              <button class="button back-btn" onclick={() => currentStep--}
                >{i18n.t("Back")}</button
              >
              <button class="button next-btn" onclick={finishOnboarding}
                >{i18n.t("Finish Setup")}</button
              >
            </div>
          </div>
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
.step-content {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.step-content h3 {
  margin: 0;
  font-size: 1.1rem;
  color: var(--text-color, #000);
}

.step-content p {
  margin: 0;
  font-size: 0.9rem;
  color: var(--text-muted, #666);
}

.custom-input {
  width: 100%;
  padding: 10px;
  background-color: transparent;
  border: 1px solid var(--border-color, #ccc);
  color: var(--text-color, #000);
  border-radius: 8px;
  font-family: inherit;
  font-size: 0.95rem;
  outline: none;
  box-sizing: border-box;
}

.custom-input:focus {
  border-color: #5b9bd5;
}

.custom-input option {
  background-color: var(--bg-color, #fff);
  color: var(--text-color, #000);
}

.btn-group {
  display: flex;
  gap: 10px;
  margin-top: 10px;
}

.button {
  padding: 10px 16px;
  border-radius: 8px;
  border: none;
  cursor: pointer;
  font-family: inherit;
  font-weight: 500;
  transition: background-color 0.2s, opacity 0.2s;
}

.next-btn {
  background-color: #5b9bd5;
  color: white;
  flex: 1;
}

.next-btn:hover {
  background-color: #4a82b8;
}

.back-btn {
  background-color: transparent;
  color: var(--text-color, #000);
  border: 1px solid var(--border-color, #ccc);
}

.back-btn:hover {
  background-color: var(--hover-bg, #eee);
}

</style>
