<script lang="ts">
  import { t, locale, setLocale, availableLocales } from '$lib/i18n';

  export let isLocalizationEnabled = false;
  export let onToggle: () => void;
  export let onCheckFolder: () => void;
  export let statusMessage = '';
</script>

<header class="header">
  <div class="header-left">
    <label class="checkbox-label">
      <input 
        type="checkbox" 
        bind:checked={isLocalizationEnabled} 
        on:change={onToggle}
      />
      <span>{$t('header.activateTestLocalization')}</span>
    </label>

    {#if isLocalizationEnabled}
      <button type="button" on:click={onCheckFolder}>
        {$t('header.checkGameFolder')}
      </button>
    {/if}
  </div>

  <div class="header-right">
    {#if statusMessage && isLocalizationEnabled}
      <span class="status-text">{statusMessage}</span>
    {/if}

    <label class="locale-label">
      <span>{$t('header.language')}:</span>
      <select 
        value={$locale} 
        on:change={(e) => setLocale(e.currentTarget.value)}
      >
        {#each availableLocales as loc}
          <option value={loc.code}>{loc.name}</option>
        {/each}
      </select>
    </label>
  </div>
</header>

<style>
  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 8px 12px;
    background: #27272a;
    border: 1px solid #3f3f46;
    border-radius: 4px;
  }

  .header-left, .header-right {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .checkbox-label, .locale-label {
    display: flex;
    align-items: center;
    gap: 6px;
    cursor: pointer;
  }

  .status-text {
    color: #60a5fa;
    font-size: 12px;
  }
</style>
