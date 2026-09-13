<script lang="ts">
  import { t } from '$lib/i18n';

  export let currentPage = 1;
  export let totalPages = 1;
  export let totalCount = 0;
  export let startIndex = 0;
  export let endIndex = 0;
  export let pageSize = 50;

  export let onPageChange: (p: number) => void;
  export let onPageSizeChange: (size: number) => void;

  function getPaginationPages(current: number, total: number): (number | string)[] {
    if (total <= 7) {
      return Array.from({ length: total }, (_, i) => i + 1);
    }
    if (current <= 4) {
      return [1, 2, 3, 4, 5, '...', total];
    }
    if (current >= total - 3) {
      return [1, '...', total - 4, total - 3, total - 2, total - 1, total];
    }
    return [1, '...', current - 1, current, current + 1, '...', total];
  }

  $: pageNumbers = getPaginationPages(currentPage, totalPages);
</script>

<footer class="pagination">
  <div>
    {$t('pagination.pageInfo', { start: Math.min(startIndex + 1, totalCount), end: endIndex, total: totalCount })}
    ({$t('pagination.pageOf', { current: currentPage, total: totalPages })})
  </div>

  <nav class="nav-pages">
    <button 
      type="button" 
      disabled={currentPage <= 1}
      on:click={() => onPageChange(currentPage - 1)}
    >
      {$t('pagination.previous')}
    </button>

    {#each pageNumbers as p, idx (p === '...' ? `dots-${idx}` : `page-${p}`)}
      {#if p === '...'}
        <span style="padding: 0 4px;">…</span>
      {:else}
        <button 
          type="button" 
          class:current={currentPage === p}
          on:click={() => onPageChange(Number(p))}
        >
          {p}
        </button>
      {/if}
    {/each}

    <button 
      type="button" 
      disabled={currentPage >= totalPages}
      on:click={() => onPageChange(currentPage + 1)}
    >
      {$t('pagination.next')}
    </button>
  </nav>

  <label>
    {$t('pagination.perPage')}:
    <select 
      value={pageSize} 
      on:change={(e) => onPageSizeChange(Number(e.currentTarget.value))}
    >
      <option value={25}>25</option>
      <option value={50}>50</option>
      <option value={100}>100</option>
      <option value={200}>200</option>
    </select>
  </label>
</footer>

<style>
  .pagination {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    padding: 6px 12px;
    background: #27272a;
    border: 1px solid #3f3f46;
    border-radius: 4px;
    flex-wrap: wrap;
    font-size: 12px;
  }

  .nav-pages {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .current {
    background: #2563eb;
    border-color: #1d4ed8;
    color: #fff;
  }
</style>
