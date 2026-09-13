<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount, onDestroy } from 'svelte';
  import { t } from '$lib/i18n';
  import Header from '$lib/components/Header.svelte';
  import UpdateBanner from '$lib/components/UpdateBanner.svelte';
  import Toolbar from '$lib/components/Toolbar.svelte';
  import LocalizationTable from '$lib/components/LocalizationTable.svelte';
  import Pagination from '$lib/components/Pagination.svelte';

  interface LocalizationEntry {
    key: string;
    original: string;
    custom: string | null;
  }

  interface LocalizationFileData {
    entries: LocalizationEntry[];
    languages: string[];
    selected_lang: string;
  }

  let isLocalizationEnabled = false;
  let langFolderExists = false;
  let availableCsvFiles: string[] = [];
  let selectedFile = 'menu.csv';
  let availableGameLanguages: string[] = [];
  let selectedGameLang = 'English';
  let starredKeys: Set<string> = new Set();

  let entries: LocalizationEntry[] = [];
  let searchQuery = '';
  let pendingDiffs: Record<string, string> = {};
  let currentPage = 1;
  let pageSize = 50;
  let onlyStarred = false;

  let tableWrapEl: HTMLDivElement | null = null;
  let searchInputEl: HTMLInputElement | null = null;
  let isApplyingToGame = false;

  let updateBannerState = {
    isUpdated: false,
    currentVersion: '',
    previousVersion: null as string | null,
    isWaitingForLang: false,
    isPatching: false,
    isSuccess: false
  };
  let updatePollInterval: ReturnType<typeof setInterval> | null = null;

  let statusState: { key: string; params?: Record<string, string | number> } | null = null;
  $: statusMessage = statusState ? $t(statusState.key, statusState.params) : '';

  async function checkGameVersionUpdate() {
    try {
      const res = await invoke<{ is_updated: boolean; current_version: string; previous_version: string | null }>('check_game_update');
      if (res.previous_version === null) {
        await invoke('acknowledge_game_version', { version: res.current_version });
        updateBannerState.currentVersion = res.current_version;
        updateBannerState.isUpdated = false;
      } else if (res.is_updated) {
        updateBannerState.isUpdated = true;
        updateBannerState.currentVersion = res.current_version;
        updateBannerState.previousVersion = res.previous_version;
      } else {
        updateBannerState.isUpdated = false;
        updateBannerState.currentVersion = res.current_version;
        updateBannerState.previousVersion = res.previous_version;
      }
    } catch {
      // Ignored if game version file does not exist
    }
  }

  async function startUpdateWorkflow() {
    try {
      await invoke('reset_lang_folder');
      updateBannerState.isWaitingForLang = true;
      updateBannerState.isPatching = false;

      if (updatePollInterval) {
        clearInterval(updatePollInterval);
      }

      let pollAttempts = 0;
      updatePollInterval = setInterval(async () => {
        pollAttempts += 1;
        if (pollAttempts > 120) {
          if (updatePollInterval) {
            clearInterval(updatePollInterval);
            updatePollInterval = null;
          }
          updateBannerState.isWaitingForLang = false;
          statusState = { key: 'status.error', params: { error: 'Update timeout waiting for War Thunder lang/ folder.' } };
          return;
        }

        try {
          const exists = await invoke<boolean>('check_lang_folder_exists');
          if (exists) {
            if (updatePollInterval) {
              clearInterval(updatePollInterval);
              updatePollInterval = null;
            }
            updateBannerState.isWaitingForLang = false;
            updateBannerState.isPatching = true;

            await invoke('apply_all_diffs_to_game', { targetLang: selectedGameLang || null });
            await invoke('acknowledge_game_version', { version: updateBannerState.currentVersion });

            updateBannerState.isPatching = false;
            updateBannerState.isSuccess = true;

            await checkFolderAndLoad();

            setTimeout(() => {
              updateBannerState.isUpdated = false;
              updateBannerState.isSuccess = false;
            }, 4000);
          }
        } catch {
          // Continue polling
        }
      }, 2000);
    } catch (err) {
      statusState = { key: 'status.error', params: { error: String(err) } };
    }
  }

  async function checkFolderAndLoad() {
    if (!isLocalizationEnabled) {
      langFolderExists = false;
      entries = [];
      availableCsvFiles = [];
      availableGameLanguages = [];
      statusState = null;
      return;
    }

    statusState = { key: 'status.checkingFolder' };
    try {
      langFolderExists = await invoke<boolean>('check_lang_folder_exists');
      
      if (langFolderExists) {
        statusState = { key: 'status.folderFoundLoadingCsv' };
        
        try {
          const loadedStarred = await invoke<string[]>('get_starred_keys');
          starredKeys = new Set(loadedStarred);
        } catch (e) {
          console.error('Failed to load starred keys:', e);
        }

        availableCsvFiles = await invoke<string[]>('get_localization_files');
        
        if (selectedFile !== '__starred__') {
          if (availableCsvFiles.includes('menu.csv')) {
            selectedFile = 'menu.csv';
          } else if (availableCsvFiles.length > 0 && !availableCsvFiles.includes(selectedFile)) {
            selectedFile = availableCsvFiles[0];
          }
        }

        await loadContent(selectedFile, selectedGameLang);
        await checkGameVersionUpdate();
        statusState = null;
      } else {
        entries = [];
        availableCsvFiles = [];
        availableGameLanguages = [];
        statusState = { key: 'status.langFolderNotFound' };
      }
    } catch (err) {
      statusState = { key: 'status.error', params: { error: String(err) } };
    }
  }

  async function handleToggle() {
    try {
      await invoke('set_localization', { enabled: isLocalizationEnabled });
      await checkFolderAndLoad();
      if (isLocalizationEnabled) {
        await checkGameVersionUpdate();
      }
    } catch (err) {
      statusState = { key: 'status.errorWritingConfig', params: { error: String(err) } };
    }
  }

  async function handleFileChange(newFile: string) {
    selectedFile = newFile;
    searchQuery = '';
    currentPage = 1;
    await loadContent(selectedFile, selectedGameLang);
  }

  async function handleGameLangChange(newLang: string) {
    selectedGameLang = newLang;
    currentPage = 1;
    await loadContent(selectedFile, newLang);
  }

  async function loadContent(fileOrView: string, targetLang?: string) {
    try {
      let data: LocalizationFileData;
      if (fileOrView === '__starred__') {
        data = await invoke<LocalizationFileData>('get_starred_entries', {
          targetLang: targetLang || null
        });
      } else {
        data = await invoke<LocalizationFileData>('get_localization_file', {
          fileName: fileOrView,
          targetLang: targetLang || null
        });
      }

      entries = data.entries;
      availableGameLanguages = data.languages;
      selectedGameLang = data.selected_lang;

      pendingDiffs = {};
      for (const item of entries) {
        if (item.custom !== null) {
          pendingDiffs[item.key] = item.custom;
        }
      }
    } catch (err) {
      statusState = { key: 'status.errorLoadingFile', params: { error: String(err) } };
    }
  }

  async function toggleStar(key: string) {
    if (starredKeys.has(key)) {
      starredKeys.delete(key);
    } else {
      starredKeys.add(key);
    }
    starredKeys = new Set(starredKeys);
    try {
      await invoke('save_starred_keys', { keys: Array.from(starredKeys) });
    } catch (err) {
      console.error('Failed to save starred keys:', err);
    }

    if (selectedFile === '__starred__') {
      await loadContent('__starred__', selectedGameLang);
    }
  }

  function handleEdit(key: string, val: string, original: string) {
    if (val.trim() === '' || val === original) {
      delete pendingDiffs[key];
    } else {
      pendingDiffs[key] = val;
    }
    pendingDiffs = { ...pendingDiffs };
  }

  function revertRow(key: string) {
    if (pendingDiffs[key] !== undefined) {
      delete pendingDiffs[key];
      pendingDiffs = { ...pendingDiffs };
    }
  }

  async function saveDiff() {
    try {
      const fileNameToSave = selectedFile === '__starred__' ? 'starred.csv' : selectedFile;
      await invoke('save_localization_diff', {
        fileName: fileNameToSave,
        diffs: pendingDiffs
      });
      const cleanStem = fileNameToSave.replace(/\.csv$/i, '');
      statusState = { key: 'status.diffSavedSuccess', params: { path: `mods/localization/diffs/${cleanStem}.json` } };
    } catch (err) {
      statusState = { key: 'status.errorSaving', params: { error: String(err) } };
    }
  }

  async function applyToGame() {
    if (isApplyingToGame) return;
    isApplyingToGame = true;
    try {
      const fileNameToSave = selectedFile === '__starred__' ? 'starred.csv' : selectedFile;
      await invoke('save_localization_diff', {
        fileName: fileNameToSave,
        diffs: pendingDiffs
      });

      if (selectedFile === '__starred__') {
        await invoke('apply_all_diffs_to_game', { targetLang: selectedGameLang || null });
      } else {
        await invoke('apply_diff_to_csv', {
          fileName: selectedFile,
          targetLang: selectedGameLang || null
        });
      }

      statusState = { key: 'editor.appliedSuccess' };
    } catch (err) {
      statusState = { key: 'status.error', params: { error: String(err) } };
    } finally {
      isApplyingToGame = false;
    }
  }

  async function restoreVanilla() {
    const confirmed = confirm($t('editor.confirmRestore'));
    if (!confirmed) return;

    try {
      const fileNameToRestore = selectedFile === '__starred__' ? 'starred.csv' : selectedFile;
      await invoke('restore_original_csv', { fileName: fileNameToRestore });
      statusState = { key: 'editor.restoredSuccess' };
      await loadContent(selectedFile, selectedGameLang);
    } catch (err) {
      statusState = { key: 'status.error', params: { error: String(err) } };
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.ctrlKey || e.metaKey) {
      if (e.key === 's' || e.key === 'S') {
        e.preventDefault();
        applyToGame();
      } else if (e.key === 'f' || e.key === 'F') {
        e.preventDefault();
        searchInputEl?.focus();
        searchInputEl?.select();
      }
    }
  }

  function goToPage(p: number) {
    if (p < 1 || p > totalPages || p === currentPage) return;
    currentPage = p;
    if (tableWrapEl) {
      tableWrapEl.scrollTop = 0;
    }
  }

  onMount(async () => {
    if (typeof window !== 'undefined') {
      window.addEventListener('keydown', handleKeydown);
    }
    try {
      const config = await invoke<{ war_thunder_files: string; localization: boolean; last_game_version?: string }>('load_app_config');
      isLocalizationEnabled = config.localization;
      if (isLocalizationEnabled) {
        await checkFolderAndLoad();
        await checkGameVersionUpdate();
      }
    } catch (err) {
      statusState = { key: 'status.error', params: { error: String(err) } };
    }
  });

  onDestroy(() => {
    if (updatePollInterval) {
      clearInterval(updatePollInterval);
      updatePollInterval = null;
    }
    if (typeof window !== 'undefined') {
      window.removeEventListener('keydown', handleKeydown);
    }
  });

  $: q = searchQuery.trim().toLowerCase();
  $: filtered = entries.filter((row) => {
    if (onlyStarred && !starredKeys.has(row.key)) return false;
    if (!q) return true;
    return (
      row.key.toLowerCase().includes(q) ||
      row.original.toLowerCase().includes(q)
    );
  });

  $: totalPages = Math.max(1, Math.ceil(filtered.length / pageSize));
  $: if (currentPage > totalPages) {
    currentPage = totalPages;
  }
  $: if (currentPage < 1) {
    currentPage = 1;
  }

  $: startIndex = (currentPage - 1) * pageSize;
  $: endIndex = Math.min(startIndex + pageSize, filtered.length);
  $: visibleRows = filtered.slice(startIndex, endIndex);
</script>

<div class="app-layout">
  <Header 
    bind:isLocalizationEnabled
    onToggle={handleToggle}
    onCheckFolder={checkFolderAndLoad}
    {statusMessage}
  />

  <UpdateBanner 
    isUpdated={isLocalizationEnabled && updateBannerState.isUpdated}
    currentVersion={updateBannerState.currentVersion}
    previousVersion={updateBannerState.previousVersion}
    isWaitingForLang={updateBannerState.isWaitingForLang}
    isPatching={updateBannerState.isPatching}
    isSuccess={updateBannerState.isSuccess}
    onUpdate={startUpdateWorkflow}
  />

  {#if isLocalizationEnabled}
    {#if !langFolderExists}
      <div class="warning-box">
        <strong>{$t('warning.title')}</strong>
        <p>{$t('warning.description')}</p>
        <small>{$t('warning.hint')}</small>
      </div>
    {:else}
      <Toolbar 
        {selectedFile}
        {availableCsvFiles}
        starredCount={starredKeys.size}
        {selectedGameLang}
        {availableGameLanguages}
        bind:searchQuery
        bind:onlyStarred
        pendingCount={Object.keys(pendingDiffs).length}
        isApplying={isApplyingToGame}
        onFileChange={handleFileChange}
        onGameLangChange={handleGameLangChange}
        onToggleStarredFilter={() => { onlyStarred = !onlyStarred; currentPage = 1; }}
        onSaveDiff={saveDiff}
        onApplyToGame={applyToGame}
        onRestoreVanilla={restoreVanilla}
        bind:searchInputEl
      />

      <LocalizationTable 
        rows={visibleRows}
        {selectedGameLang}
        {starredKeys}
        {pendingDiffs}
        onToggleStar={toggleStar}
        onEdit={handleEdit}
        onRevertRow={revertRow}
        bind:tableWrapEl
      />

      {#if filtered.length > 0}
        <Pagination 
          {currentPage}
          {totalPages}
          totalCount={filtered.length}
          {startIndex}
          {endIndex}
          bind:pageSize
          onPageChange={goToPage}
          onPageSizeChange={(newSize) => { pageSize = newSize; currentPage = 1; }}
        />
      {/if}
    {/if}
  {/if}
</div>

<style>
  .app-layout {
    display: flex;
    flex-direction: column;
    height: 100vh;
    padding: 8px;
    gap: 8px;
    box-sizing: border-box;
  }

  .warning-box {
    padding: 16px;
    background: #422006;
    border: 1px solid #854d0e;
    border-radius: 4px;
    color: #fef08a;
  }

  .warning-box p {
    margin: 6px 0;
    color: #e4e4e7;
  }

  .warning-box small {
    color: #a1a1aa;
  }
</style>