<script lang="ts">
  import { t } from '$lib/i18n';

  export let isUpdated = false;
  export let currentVersion = '';
  export let previousVersion: string | null = null;
  export let isWaitingForLang = false;
  export let isPatching = false;
  export let isSuccess = false;
  export let onUpdate: () => void;
</script>

{#if isUpdated}
  <div class="banner" class:is-success={isSuccess}>
    <div class="banner-text">
      <strong>{$t('updateBanner.title')}:</strong>
      {#if isSuccess}
        {$t('updateBanner.success', { version: currentVersion })}
      {:else if isWaitingForLang}
        {$t('updateBanner.waitingForGame')}
      {:else if isPatching}
        {$t('editor.applying')}
      {:else}
        {$t('updateBanner.description', { 
          oldVersion: previousVersion ?? '?', 
          newVersion: currentVersion 
        })}
      {/if}
    </div>

    {#if !isWaitingForLang && !isPatching && !isSuccess}
      <button type="button" class="update-btn" on:click={onUpdate}>
        {$t('updateBanner.button')}
      </button>
    {/if}
  </div>
{/if}

<style>
  .banner {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 8px 12px;
    background: #422006;
    border: 1px solid #854d0e;
    border-radius: 4px;
    color: #fef08a;
  }

  .banner.is-success {
    background: #064e3b;
    border-color: #047857;
    color: #a7f3d0;
  }

  .update-btn {
    background: #ca8a04;
    border-color: #eab308;
    color: #000;
    font-weight: bold;
  }

  .update-btn:hover {
    background: #eab308;
  }
</style>
