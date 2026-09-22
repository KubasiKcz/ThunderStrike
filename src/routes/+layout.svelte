<script lang="ts">
  import '../app.css';
  import { page } from '$app/stores';
  import { t } from '$lib/i18n';

  let { children } = $props();

  let currentPath = $derived($page.url.pathname);
</script>

<svelte:head>
  <title>ThunderStrike</title>
</svelte:head>

<div class="shell">
  <nav class="sidebar" aria-label="Navigation">
    <a
      href="/localization"
      class="nav-item"
      class:active={currentPath.startsWith('/localization') || currentPath === '/'}
      title={$t('nav.localization')}
      aria-label={$t('nav.localization')}
    >
      <span class="nav-icon" style="-webkit-mask-image: url('/icons/ui/localization.svg'); mask-image: url('/icons/ui/localization.svg');"></span>
      <span class="nav-label">{$t('nav.localization')}</span>
    </a>

    <div class="sidebar-spacer"></div>

    <a
      href="/settings"
      class="nav-item"
      class:active={currentPath.startsWith('/settings')}
      title={$t('nav.settings')}
      aria-label={$t('nav.settings')}
    >
      <span class="nav-icon" style="-webkit-mask-image: url('/icons/ui/settings.svg'); mask-image: url('/icons/ui/settings.svg');"></span>
      <span class="nav-label">{$t('nav.settings')}</span>
    </a>
  </nav>

  <main class="content">
    {@render children()}
  </main>
</div>

<style>
  :global(html, body) {
    height: 100%;
    margin: 0;
    padding: 0;
    overflow: hidden;
  }

  .shell {
    display: flex;
    flex-direction: row;
    height: 100vh;
    width: 100vw;
    overflow: hidden;
    background: #18181b;
  }

  .sidebar {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 6px;
    width: 80px;
    min-width: 80px;
    height: 100vh;
    padding: 10px 4px;
    background: #141416;
    border-right: 1px solid #27272a;
    flex-shrink: 0;
    box-sizing: border-box;
  }

  .sidebar-spacer {
    flex: 1;
  }

  .nav-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 4px;
    width: 72px;
    padding: 8px 4px;
    border-radius: 8px;
    color: #71717a;
    text-decoration: none;
    transition: all 0.15s ease;
    cursor: pointer;
    box-sizing: border-box;
  }

  .nav-item:hover {
    background: #27272a;
    color: #e4e4e7;
  }

  .nav-item.active {
    background: rgba(59, 130, 246, 0.15);
    color: #3b82f6;
  }

  .nav-icon {
    width: 22px;
    height: 22px;
    display: block;
    flex-shrink: 0;
    background-color: currentColor;
    -webkit-mask-size: contain;
    mask-size: contain;
    -webkit-mask-repeat: no-repeat;
    mask-repeat: no-repeat;
    -webkit-mask-position: center;
    mask-position: center;
  }

  .nav-label {
    font-size: 11px;
    font-weight: 500;
    text-align: center;
    line-height: 1.1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 100%;
  }

  .content {
    flex: 1;
    min-width: 0;
    height: 100vh;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    box-sizing: border-box;
  }
</style>
