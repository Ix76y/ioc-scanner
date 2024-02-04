<script>
    import { invoke } from '@tauri-apps/api/tauri';

    const rowHeight = 32;

    let hasStore = false;
    let storeError = "";

    /**
     * @type {string[]}
     */
    let configuredIntegrations = [];

    async function getIntegrations() {
        createSecretStore();
        configuredIntegrations = await invoke('get_keys');
        console.log('Configured Integrations: ' + configuredIntegrations);
    }

    async function createSecretStore() {
        hasStore = await invoke('has_secrets_store');
        if (!hasStore) {
            hasStore = await invoke('create_secrets_store');
            console.log("Created secrets store..." + hasStore);
            if (!hasStore) {
                storeError = "Error: Couldn't save secrets storage correctly..."
            }
        }
    }

    getIntegrations();


</script>

<div class="p-4 bg-zinc-100 dark:bg-zinc-900 text-zinc-900 dark:text-zinc-100">
    <div class="flex justify-between">
        <h3 class="text-xl font-extrabold my-2">Integrations</h3>
        {#if hasStore }
            <button><ion-icon name="add" class="text-3xl text-sky-500 hover:text-sky-600"></ion-icon></button>
        {/if}
    </div>
    {#if hasStore }
        <div>
            {#each configuredIntegrations as integration}
            <div class="bg-white dark:bg-zinc-800 hover:bg-zinc-50 hover:dark:bg-zinc-700 rounded overflow-hidden flex flew-row p-3 my-2 items-center">
                <img src={integration.img} class="mx-auto" alt="logo" width="{rowHeight}px" height="{rowHeight}px" />

                <p class="grow mx-2 inline-block align-middle "  style="height='{rowHeight}px'">{integration.name} <!--<a href="{integration.url}" target="_blank"><ion-icon name="link-outline"></ion-icon></a>--></p>
                {#if integration.status == 'Not Started'}
                    <p class="inline-block align-middle font-extralight text-xs text-slate-600 dark:text-slate-400">{integration.status}</p>
                {:else if integration.status == 'In Progress'}
                    <p class="inline-block align-middle font-extralight text-xs text-amber-600 dark:text-amber-400">{integration.status}</p>
                {:else if integration.status == 'Done'}
                    <p class="inline-block align-middle font-extralight text-xs text-green-600 dark:text-green-400">{integration.status}</p>
                {:else}
                    <p class="inline-block align-middle font-extralight text-xs text-purple-600 dark:text-purple-400">{integration.status}</p>
                {/if}
            </div>
        {/each}
        </div>
    {/if}
    {#if storeError != "" }
        <div class="flex text-red-900 dark:text-red-50 bg-red-500/25 rounded p-2">
            <ion-icon name="warning-outline" class="text-2xl pr-2"></ion-icon><p>{ storeError }</p>
        </div>
    {/if}
</div>