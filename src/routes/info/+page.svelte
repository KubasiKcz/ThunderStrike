<script lang="ts">
  import { navStore } from "../../lib/stores/navStore.svelte";
</script>

<div class="content-wrapper">
  <p>Lorem ipsum dolor sit amet consectetur adipisicing elit. Sequi, similique?</p>
</div>

<style>
  .content-wrapper {
    animation: fadein 0.3s;
  }
  
  @keyframes fadein {
    from { opacity: 0; transform: translateY(5px); }
    to { opacity: 1; transform: translateY(0); }
  }
</style>
