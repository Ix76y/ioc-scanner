<script>
// @ts-nocheck

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



{#if urlscanLoading }
<button type="button" class="inline-flex items-center px-4 py-2 font-semibold leading-6 text-sm shadow rounded-md text-white bg-indigo-500 hover:bg-indigo-400 transition ease-in-out duration-150 cursor-not-allowed" disabled>
  <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
    <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
  </svg>
  Scanning Page...
</button>
{:else}
<div class="divide-y divide-dashed divide-gray-400 dark:divide-gray-600">
  <div class="grid grid-cols-2 gap-3">
    <div class="select-none cursor-default col-span-2 w-full italic font-thin text-slate-600 dark:text-slate-400 text-sm">Page</div>
    <div class="">
        <div class="select-none cursor-default text-xs font-light italic text-gray-600  dark:text-gray-400">ASN</div>
        <div class="">{urlscanResult.page.asn} ({#if urlscanResult.page.asnname}{urlscanResult.page.asnname}{:else}-{/if})</div>
    </div>

    <img class="row-span-6" src="{urlscanResult.task.screenshotURL}" alt="Page Screenshot">

    <div class="">
        <div class="select-none cursor-default text-xs font-light italic text-gray-600  dark:text-gray-400">IP</div>
        <div class="">{urlscanResult.page.ip}</div>
    </div>

    <div class="">
        <div class="select-none cursor-default text-xs font-light italic text-gray-600  dark:text-gray-400">City, Country</div>
        <div class="">{urlscanResult.page.city}, {urlscanResult.page.country}</div>
    </div>

    <div class="">
        <div class="select-none cursor-default text-xs font-light italic text-gray-600  dark:text-gray-400">Server</div>
        <div class="">{#if urlscanResult.page.server}{urlscanResult.page.server}{:else}-{/if}</div>
    </div>

    <div class="">
      <div class="select-none cursor-default text-xs font-light italic text-gray-600  dark:text-gray-400">Redirected</div>
      <div class="">{#if urlscanResult.page.redirected}{urlscanResult.page.redirected}{:else}-{/if}</div>
    </div>

   

    <div class="">
      <div class="select-none cursor-default text-xs font-light italic text-gray-600  dark:text-gray-400">Title</div>
      <div class="">{#if urlscanResult.page.title}{urlscanResult.page.title}{:else}-{/if}</div>
    </div>

    <div class="">
      <div class="select-none cursor-default text-xs font-light italic text-gray-600  dark:text-gray-400">Apex Domain</div>
      <div class="">{#if urlscanResult.page.apexDomain}{urlscanResult.page.apexDomain}{:else}-{/if}</div>
    </div>

    <div class="grid grid-cols-2 gap-3">
      <div class="">
        <div class="select-none cursor-default text-xs font-light italic text-gray-600  dark:text-gray-400">MimeType</div>
        <div class="">{#if urlscanResult.page.mimeType}{urlscanResult.page.mimeType}{:else}-{/if}</div>
      </div>

      <div class="">
        <div class="select-none cursor-default text-xs font-light italic text-gray-600  dark:text-gray-400">Status</div>
        {#if urlscanResult.page.status.startsWith("2")}
          <div class="select-none inline-flex items-center rounded bg-green-50/25 px-2 py-1 text-xs font-medium text-green-700 dark:text-green-300 ring-1 ring-inset ring-green-600/20 dark:ring-green-400/75">{urlscanResult.page.status}</div>
        {:else if urlscanResult.page.status.startsWith("3")}
          <div class="select-none inline-flex items-center rounded bg-amber-50/25 px-2 py-1 text-xs font-medium text-amber-700 dark:text-amber-300 ring-1 ring-inset ring-amber-600/10 dark:ring-amber-400/75">{urlscanResult.page.status}</div>
        {:else}
          <div class="select-none inline-flex items-center rounded bg-red-50/25 px-2 py-1 text-xs font-medium text-red-700 dark:text-red-300 ring-1 ring-inset ring-red-600/10 dark:ring-red-400/75">{urlscanResult.page.status}</div>
        {/if}
      </div>
    </div>


    

  </div>

  <!-- Verdicts -->
  <div class="my-4 pt-2">
      <div class="select-none cursor-default w-full italic font-thin text-slate-600 dark:text-slate-400 text-sm mb-3">Verdict</div>
      <div class="grid grid-cols-4 gap-3">
        <div class="">
          <div class="select-none cursor-default text-xs font-light italic text-gray-600  dark:text-gray-400">Categories</div>
          {#if urlscanResult.verdicts.urlscan.categories.length > 0}
            {#each urlscanResult.verdicts.urlscan.categories as category }
              <div class="select-none inline-flex items-center rounded bg-indigo-50/25 px-2 py-1 text-xs font-medium text-indigo-700 dark:text-indigo-300 ring-1 ring-inset ring-indigo-600/10 dark:ring-indigo-400/75">{category}</div>
            {/each}
          {:else}
            <div class="">-</div>
          {/if}
        </div>

        <div class="col-span-2">
          <div class="select-none cursor-default text-xs font-light italic text-gray-600  dark:text-gray-400">Targeting</div>
          {#if urlscanResult.verdicts.urlscan.brands.length > 0 }
            {#each urlscanResult.verdicts.urlscan.brands as brand }
              <div class="">{brand.name} (
              {#each brand.vertical as vertical}
                <span class="select-none inline-flex items-center rounded bg-slate-50/25 px-2 py-1 text-xs font-medium text-slate-700 dark:text-slate-300 ring-1 ring-inset ring-slate-600/10 dark:ring-slate-400/75">{vertical}</span>
              {/each}
              )</div>
            {/each}
          {:else}
            <div class="">-</div>
          {/if}
        </div>

        <div class="">
          <div class="select-none cursor-default text-xs font-light italic text-gray-600  dark:text-gray-400">Malicious</div>
          {#if urlscanResult.verdicts.urlscan.malicious}
                  <ion-icon class="text-red-600 dark:text-red-400" name="checkmark-outline"></ion-icon>
              {:else}
                  <ion-icon class="text-green-600 dark:text-green-400" name="close-outline"></ion-icon>
              {/if}
        </div>

      </div>
  </div>

  <!-- Hosted Domains -->
  <!--<div class="my-4 pt-2">
      <div class="select-none cursor-default w-full italic font-thin text-slate-600 dark:text-slate-400 text-sm mb-3">Hosted Domains</div>
      <div>Content</div>
  </div>-->
</div>
{/if}

