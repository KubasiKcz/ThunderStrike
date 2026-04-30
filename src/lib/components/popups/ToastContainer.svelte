<script lang="ts">
  import { toastStore } from '../../stores/toastStore';
  import { fly, fade } from 'svelte/transition';

</script>

<div class="toast-container">
  {#each $toastStore as toast (toast.id)}
    <div 
      class="toast {toast.type}" 
      in:fly={{ y: 20, duration: 300 }} 
      out:fade={{ duration: 200 }}
    >
      <div class="toast-icon">
        {#if toast.type === 'success'}
          <img src="/img/icons/success.svg" alt="Success" class="svg">
        {:else if toast.type === 'error'}
          <img src="/img/icons/error.svg" alt="Error" class="svg">
        {:else if toast.type === 'warning'}
          <img src="/img/icons/warning.svg" alt="Warning" class="svg">
        {:else if toast.type === 'loading'}
          <img src="/img/icons/loading.svg" alt="Loading" class="spinnerSvg">
        {:else}
          <img src="/img/icons/info.svg" alt="Info" class="svg">
        {/if}
      </div>
      <div class="toast-message">
        {toast.message}
      </div>
      <button class="toast-close" onclick={() => toastStore.remove(toast.id)} aria-label="Close">
        <img src="/img/icons/close.svg" alt="Close" class="svg">
      </button>
    </div>
  {/each}
</div>



<style>
.toast-container {
  position: fixed;
  bottom: 20px;
  right: 20px;
  display: flex;
  flex-direction: column;
  gap: 10px;
  z-index: 9999;
  pointer-events: none;
}

.toast {
  display: flex;
  align-items: center;
  gap: 12px;
  background-color: var(--bg-color);
  color: var(--text-color);
  padding: 12px 16px;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  border-left: 4px solid var(--border-color);
  pointer-events: auto;
  max-width: 350px;
  min-width: 250px;
}

.toast-message {
  flex: 1;
  font-size: 0.95rem;
}

.toast-close {
  background: transparent;
  border: none;
  color: var(--icon-color);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 4px;
  border-radius: 4px;
  transition: background-color 0.2s;
}

.toast-close:hover {
  background-color: var(--hover-bg);
}

.toast-icon {
  display: flex;
  align-items: center;
  justify-content: center;
}

/* Status Colors */
.toast.info { border-left-color: #3b82f6; }
.toast.info .toast-icon { color: #3b82f6; }

.toast.success { border-left-color: #10b981; }
.toast.success .toast-icon { color: #10b981; }

.toast.warning { border-left-color: #f59e0b; }
.toast.warning .toast-icon { color: #f59e0b; }

.toast.error { border-left-color: #ef4444; }
.toast.error .toast-icon { color: #ef4444; }

.toast.loading { border-left-color: #8b5cf6; }
.toast.loading .toast-icon { color: #8b5cf6; }

.spinnerSvg {
  animation: spin 1s linear infinite;
  width: 18px;
  height: 18px;
  filter: var(--icon-filter);
}

@keyframes spin {
  100% { transform: rotate(360deg); }
}

</style>
