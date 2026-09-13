<script lang="ts">
  import { t } from '$lib/i18n';

  export let selectedFile = 'menu.csv';
  export let availableCsvFiles: string[] = [];
  export let starredCount = 0;
  export let selectedGameLang = 'English';
  export let availableGameLanguages: string[] = [];
  export let searchQuery = '';
  export let onlyStarred = false;
  export let pendingCount = 0;
  export let isApplying = false;

  export let onFileChange: (file: string) => void;
  export let onGameLangChange: (lang: string) => void;
  export let onToggleStarredFilter: () => void;
  export let onSaveDiff: () => void;
  export let onApplyToGame: () => void;
  export let onRestoreVanilla: () => void;
  export let searchInputEl: HTMLInputElement | null = null;
</script>

<div class="toolbar">
  <!-- Top row: File & Language controls + Action buttons -->
  <div class="toolbar-row">
    <div class="toolbar-group">
      <label class="field-label">
        {$t('editor.selectFile')}:
        <select 
          value={selectedFile} 
          on:change={(e) => onFileChange(e.currentTarget.value)}
        >
          <option value="__starred__">★ {$t('editor.starredCount', { count: starredCount })}</option>
          {#if availableCsvFiles.length > 0}
            <optgroup label="CSV Files">
              {#each availableCsvFiles as file}
                <option value={file}>{file}</option>
              {/each}
            </optgroup>
          {/if}
        </select>
      </label>

      {#if availableGameLanguages.length > 0}
        <label class="field-label">
          {$t('editor.gameLanguage')}:
          <select 
            value={selectedGameLang} 
            on:change={(e) => onGameLangChange(e.currentTarget.value)}
          >
            {#each availableGameLanguages as lang}
              <option value={lang}>{lang}</option>
            {/each}
          </select>
        </label>
      {/if}
    </div>

    <div class="toolbar-group actions-group">
      <button type="button" on:click={onSaveDiff}>
        {$t('editor.saveDiff', { count: pendingCount })}
      </button>

      <button 
        type="button" 
        class="btn-primary" 
        on:click={onApplyToGame} 
        disabled={isApplying}
      >
        {isApplying ? $t('editor.applying') : $t('editor.applyToGame')}
      </button>

      <button type="button" class="btn-danger" on:click={onRestoreVanilla}>
        {$t('editor.restoreOriginal')}
      </button>
    </div>
  </div>

  <!-- Bottom row: Search filter & Starred Only toggle -->
  <div class="toolbar-row">
    <input 
      bind:this={searchInputEl}
      type="text" 
      placeholder={$t('editor.searchPlaceholder')} 
      bind:value={searchQuery}
      class="search-input"
    />

    <button 
      type="button" 
      class="star-filter-btn"
      class:is-active={onlyStarred} 
      on:click={onToggleStarredFilter}
    >
      ★ {onlyStarred ? $t('editor.showAll') : $t('editor.starredOnly')}
      {#if starredCount > 0} ({starredCount}){/if}
    </button>
  </div>
</div>

<style>
  .toolbar {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 8px 12px;
    background: #27272a;
    border: 1px solid #3f3f46;
    border-radius: 4px;
  }

  .toolbar-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
  }

  .toolbar-group {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-wrap: wrap;
  }

  .actions-group {
    margin-left: auto;
  }

  .field-label {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    white-space: nowrap;
  }

  .search-input {
    flex: 1;
    min-width: 150px;
  }

  .star-filter-btn {
    white-space: nowrap;
    flex-shrink: 0;
  }

  .btn-primary {
    background: #2563eb;
    border-color: #1d4ed8;
    color: #fff;
    white-space: nowrap;
  }

  .btn-primary:hover:not(:disabled) {
    background: #1d4ed8;
  }

  .btn-danger {
    color: #f87171;
    border-color: #7f1d1d;
    white-space: nowrap;
  }

  .btn-danger:hover {
    background: #450a0a;
  }

  .is-active {
    background: #713f12;
    border-color: #a16207;
    color: #fef08a;
  }
</style>
