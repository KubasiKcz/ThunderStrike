<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { t, locale, setLocale, availableLocales } from '$lib/i18n';
  import { onMount } from 'svelte';

  let gamePath = '';
  let saveStatus: 'idle' | 'saved' | 'error' = 'idle';
  let saveError = '';
  let fetchStatus: 'idle' | 'fetching' | 'success' | 'error' = 'idle';
  let fetchMessage = '';
  let showPathHelp = false;

  onMount(async () => {
    try {
      const config = await invoke<{ war_thunder_files: string }>('load_app_config');
      gamePath = config.war_thunder_files;
    } catch (e) {
      console.error('Failed to load config:', e);
    }
  });

  async function saveGamePath() {
    saveStatus = 'idle';
    try {
      await invoke('save_war_thunder_path', { path: gamePath });
      saveStatus = 'saved';
      setTimeout(() => { saveStatus = 'idle'; }, 2500);
    } catch (e) {
      saveError = String(e);
      saveStatus = 'error';
    }
  }

  // Fallback list of common War Thunder lang files (used when lang/ folder isn't loaded yet)
  const KNOWN_FILES = [
    'menu.csv', 'units.csv', 'char.csv', 'tips.csv', 'tutorial.csv',
    'controls.csv', 'encyclopedia.csv', 'inf.csv', 'item_shop.csv',
    'matching.csv', 'benchmark.csv'
  ];

  async function fetchDatamine() {
    fetchStatus = 'fetching';
    fetchMessage = '';
    try {
      // Try to get actual available file list; fall back to known list
      let fileNames: string[] = KNOWN_FILES;
      try {
        const loaded = await invoke<string[]>('get_localization_files');
        if (loaded && loaded.length > 0) fileNames = loaded;
      } catch { /* ignore – lang folder may not be present */ }

      const count = await invoke<number>('fetch_clean_lang_from_datamine', { fileNames });
      fetchMessage = $t('settings.fetchSuccess', { count });
      fetchStatus = 'success';
    } catch (e) {
      fetchMessage = $t('settings.fetchError', { error: String(e) });
      fetchStatus = 'error';
    }
  }
</script>

<div class="settings-page">
  <h1>{$t('settings.title')}</h1>

  <!-- Game path -->
  <section class="section">
    <div class="field-label-row">
      <label class="field-label" for="game-path">{$t('settings.gamePath')}</label>
      <div class="help-wrapper">
        <button 
          type="button" 
          class="help-btn"
          aria-label={$t('settings.pathHelpTitle')}
          title={$t('settings.pathHelpTitle')}
          on:click={() => showPathHelp = !showPathHelp}
        >
          ⓘ
        </button>

        {#if showPathHelp}
          <!-- svelte-ignore a11y-click-events-have-key-events -->
          <!-- svelte-ignore a11y-no-static-element-interactions -->
          <div class="help-backdrop" on:click={() => showPathHelp = false}></div>
          <div class="help-popover">
            <div class="help-header">
              <strong>{$t('settings.pathHelpTitle')}</strong>
              <button type="button" class="help-close" on:click={() => showPathHelp = false}>✕</button>
            </div>
            <div class="help-section">
              <span class="help-badge">Steam</span>
              <p>{$t('settings.pathHelpSteamPath')}</p>
            </div>
            <div class="help-section">
              <span class="help-badge">Launcher</span>
              <p>{$t('settings.pathHelpLauncherPath')}</p>
            </div>
          </div>
        {/if}
      </div>
    </div>

    <div class="input-row">
      <input
        id="game-path"
        type="text"
        bind:value={gamePath}
        placeholder={$t('settings.gamePathPlaceholder')}
        class="path-input"
      />
      <button type="button" on:click={saveGamePath} class:success={saveStatus === 'saved'}>
        {saveStatus === 'saved' ? $t('settings.saved') : $t('settings.save')}
      </button>
    </div>
    {#if saveStatus === 'error'}
      <p class="status-error">{$t('settings.saveError', { error: saveError })}</p>
    {/if}
  </section>

  <!-- App language -->
  <section class="section">
    <label class="field-label" for="app-lang">{$t('settings.appLanguage')}</label>
    <select
      id="app-lang"
      value={$locale}
      on:change={(e) => setLocale(e.currentTarget.value)}
    >
      {#each availableLocales as loc}
        <option value={loc.code}>{loc.name}</option>
      {/each}
    </select>
  </section>

  <!-- Fetch datamine -->
  <section class="section">
    <h2 class="section-title">{$t('settings.fetchDatamine')}</h2>
    <p class="hint">{$t('settings.fetchDatamineHint')}</p>
    <button
      type="button"
      on:click={fetchDatamine}
      disabled={fetchStatus === 'fetching'}
      class="fetch-btn"
    >
      {fetchStatus === 'fetching' ? $t('settings.fetching') : $t('settings.fetchDatamine')}
    </button>
    {#if fetchMessage}
      <p class="fetch-status" class:success={fetchStatus === 'success'} class:error={fetchStatus === 'error'}>
        {fetchMessage}
      </p>
    {/if}
  </section>
</div>

<style>
  .settings-page {
    padding: 20px 24px;
    max-width: 600px;
    height: 100%;
    overflow-y: auto;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    gap: 28px;
  }

  h1 {
    margin: 0 0 4px;
    font-size: 18px;
    font-weight: 600;
    color: #e4e4e7;
  }

  .section {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .section-title {
    margin: 0;
    font-size: 13px;
    font-weight: 600;
    color: #e4e4e7;
  }

  .field-label-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .field-label {
    font-size: 12px;
    color: #a1a1aa;
    font-weight: 500;
  }

  .help-wrapper {
    position: relative;
    display: inline-flex;
    align-items: center;
  }

  .help-btn {
    width: 20px;
    height: 20px;
    padding: 0;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    background: #3f3f46;
    color: #e4e4e7;
    font-size: 12px;
    font-weight: bold;
    border: 1px solid #52525b;
    cursor: pointer;
    line-height: 1;
    transition: background 0.15s, color 0.15s;
  }

  .help-btn:hover {
    background: #3b82f6;
    color: #fff;
    border-color: #2563eb;
  }

  .help-backdrop {
    position: fixed;
    inset: 0;
    z-index: 99;
  }

  .help-popover {
    position: absolute;
    top: calc(100% + 6px);
    left: 0;
    width: 380px;
    max-width: 85vw;
    background: #18181b;
    border: 1px solid #3f3f46;
    border-radius: 8px;
    box-shadow: 0 10px 25px -5px rgba(0, 0, 0, 0.5), 0 8px 10px -6px rgba(0, 0, 0, 0.5);
    padding: 14px;
    z-index: 100;
    display: flex;
    flex-direction: column;
    gap: 10px;
    font-size: 13px;
    color: #d4d4d8;
  }

  .help-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    border-bottom: 1px solid #27272a;
    padding-bottom: 8px;
  }

  .help-header strong {
    color: #fafafa;
    font-size: 13px;
  }

  .help-close {
    background: transparent;
    border: none;
    color: #a1a1aa;
    cursor: pointer;
    padding: 2px 6px;
    font-size: 12px;
  }

  .help-close:hover {
    color: #fff;
  }

  .help-section {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .help-section p {
    margin: 0;
    line-height: 1.4;
    color: #a1a1aa;
    font-size: 12px;
  }

  .help-badge {
    display: inline-block;
    align-self: flex-start;
    padding: 1px 6px;
    background: #3f3f46;
    color: #fafafa;
    font-size: 11px;
    font-weight: 600;
    border-radius: 3px;
  }

  .input-row {
    display: flex;
    gap: 8px;
  }

  .path-input {
    flex: 1;
    background: #27272a;
    color: #e4e4e7;
    border: 1px solid #3f3f46;
    border-radius: 4px;
    padding: 6px 10px;
    font-size: 13px;
  }

  .path-input:focus {
    outline: none;
    border-color: #3b82f6;
  }

  button.success {
    background: #166534;
    border-color: #15803d;
    color: #bbf7d0;
  }

  .hint {
    margin: 0;
    font-size: 12px;
    color: #71717a;
    line-height: 1.5;
  }

  .fetch-btn {
    align-self: flex-start;
  }

  .fetch-status {
    margin: 0;
    font-size: 12px;
  }

  .fetch-status.success {
    color: #4ade80;
  }

  .fetch-status.error {
    color: #f87171;
  }

  .status-error {
    margin: 0;
    font-size: 12px;
    color: #f87171;
  }

  select {
    align-self: flex-start;
  }
</style>
