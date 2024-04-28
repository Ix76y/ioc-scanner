<script>
    import { invoke } from '@tauri-apps/api/tauri'
  
    /**
     * @type {boolean}
     */
    export let urlscanLoading;

    /**
     * @type {JSON}
     */
    export let urlscanResult;
</script> 


<div class="bg-zinc-200 dark:bg-zinc-800 rounded overflow-hidden shadow-lg">
  <div class="m-4">
    {#if urlscanLoading }
      <p>Loading...</p>
    {/if}
    { urlscanResult }
  </div>
</div>
