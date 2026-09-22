<script context="module" lang="ts">
  export interface LocalizationRow {
    key: string;
    original: string;
    custom: string | null;
  }
</script>

<script lang="ts">
  import { t } from '$lib/i18n';

  export let rows: LocalizationRow[] = [];
  export let selectedGameLang = 'English';
  export let starredKeys: Set<string> = new Set();
  export let pendingDiffs: Record<string, string> = {};

  export let onToggleStar: (key: string) => void;
  export let onEdit: (key: string, val: string, original: string) => void;
  export let onRevertRow: (key: string) => void;
  export let tableWrapEl: HTMLDivElement | null = null;

  function autoResize(node: HTMLTextAreaElement, _val?: any) {
    function resize() {
      node.style.height = 'auto';
      node.style.height = `${node.scrollHeight}px`;
    }

    requestAnimationFrame(resize);
    node.addEventListener('input', resize);

    return {
      update() {
        requestAnimationFrame(resize);
      },
      destroy() {
        node.removeEventListener('input', resize);
      }
    };
  }
</script>

<div class="table-wrap" bind:this={tableWrapEl}>
  <table>
    <thead>
      <tr>
        <th style="width: 28%;">{$t('table.colKey')}</th>
        <th style="width: 36%;">{$t('table.colOriginal')} ({selectedGameLang})</th>
        <th style="width: 36%;">{$t('table.colCustom')}</th>
      </tr>
    </thead>
    <tbody>
      {#each rows as row (row.key)}
        {@const isStarred = starredKeys.has(row.key)}
        {@const isModified = pendingDiffs[row.key] !== undefined}
        {@const currentValue = pendingDiffs[row.key] ?? row.custom ?? row.original}
        <tr class:modified={isModified}>
          <td>
            <div class="key-inner">
              <button 
                type="button" 
                class="star-btn" 
                class:starred={isStarred}
                on:click={() => onToggleStar(row.key)}
              >
                {isStarred ? '★' : '☆'}
              </button>
              <code>{row.key}</code>
            </div>
          </td>

          <td class="orig-cell">
            {row.original}
          </td>

          <td class="custom-cell">
            <div class="input-row">
              <textarea 
                value={currentValue}
                use:autoResize={currentValue}
                on:input={(e) => onEdit(row.key, e.currentTarget.value, row.original)}
                rows="1"
              ></textarea>

              {#if isModified}
                <button 
                  type="button" 
                  class="revert-btn" 
                  on:click={() => onRevertRow(row.key)} 
                  title={$t('editor.revertRow')}
                >
                  ↺
                </button>
              {/if}
            </div>
          </td>
        </tr>
      {/each}

      {#if rows.length === 0}
        <tr>
          <td colspan="3" style="text-align: center; padding: 20px; color: #a1a1aa;">
            No entries found
          </td>
        </tr>
      {/if}
    </tbody>
  </table>
</div>

<style>
  .table-wrap {
    flex: 1;
    min-height: 0;
    overflow: auto;
    border: 1px solid #3f3f46;
    border-radius: 4px;
    background: #18181b;
  }

  table {
    width: 100%;
    border-collapse: collapse;
    font-size: 13px;
  }

  th {
    position: sticky;
    top: 0;
    background: #27272a;
    text-align: left;
    padding: 6px 8px;
    border-bottom: 1px solid #3f3f46;
    z-index: 1;
  }

  td {
    padding: 6px 8px;
    border-bottom: 1px solid #2d2d30;
    vertical-align: middle;
  }

  tr:hover {
    background: #27272a;
  }

  tr.modified {
    background: #1e293b;
  }

  .key-inner {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .star-btn {
    background: none;
    border: none;
    padding: 0;
    font-size: 15px;
    color: #71717a;
    line-height: 1;
  }

  .star-btn.starred {
    color: #eab308;
  }

  code {
    overflow-wrap: anywhere;
    word-break: break-word;
    color: #93c5fd;
    font-family: monospace;
    font-size: 12px;
  }

  .orig-cell {
    white-space: pre-wrap;
    word-break: break-word;
  }

  .input-row {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  textarea {
    width: 100%;
    resize: none;
    box-sizing: border-box;
    white-space: pre-wrap;
    line-height: 1.4;
    overflow-y: hidden;
  }

  .revert-btn {
    padding: 2px 6px;
    font-size: 13px;
  }
</style>
