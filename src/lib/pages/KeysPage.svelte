<script>
    import { invoke } from '@tauri-apps/api/tauri';
    import { fade } from 'svelte/transition';

    const rowHeight = 32;

    let hasStore = false;
    let storeError = "";

    // variable to store which intgration is currently open if any
    let openedIndex = -1;

    /**
     * @type {any}
     */
    let integrations = [];

    async function getIntegrations() {
        createSecretStore();
        // configuredIntegrations = await invoke('get_keys');
        // console.log('Configured Integrations: ' + configuredIntegrations);
        integrations = await invoke('get_integrations');
        /*console.log('All Integrations: ');
        for (const i of integrations) {
            console.log(`${i.name}: ${i.configured}`);
        }*/
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

    /**
     * @param {number} i
     */
    async function testConnection(i) {
        console.log(`Testing connection for ${integrations[i].name}`);
    }

    /**
     * @param {number} i
     */
    async function saveIntegration(i) {
        let saved = await invoke('update_secret', {key: integrations[i].name, secret: integrations[i].apikey});
        console.log("Key successfully saved! Getting integrations again to update UI?");
        getIntegrations();
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
            {#each integrations as integration, i}
            <div class="divide-y divide-dashed divide-zinc-100 dark:divide-zinc-600 bg-white dark:bg-zinc-800 rounded overflow-hidden my-2 ">
                <div class=" flex flew-row items-center p-3">
                    <img src={integration.image} class="mx-auto" alt="logo" width="{rowHeight}px" height="{rowHeight}px" />
                    <!--     Planned, InProgress, Implemented, Requested-->
                    <p class="grow mx-2 inline-block align-middle "  style="height='{rowHeight}px'">{integration.name} <!--<a href="{integration.url}" target="_blank"><ion-icon name="link-outline"></ion-icon></a>--></p>
                    
                    <!-- TODO: show button when closed, otherwise show x; if x is clicked reset input? -->
                    <button class="text-sky-500 hover:text-gray-700 hover:dark:text-white font-extralight text-sm hover:font-light hover:underline underline-offset-2 decoration-8 decoration-sky-500" on:click={() => openedIndex = i}>
                        {#if integration.configured}
                        Edit
                        {:else}
                        Configure
                        {/if}
                    </button>
                </div>
                {#if i == openedIndex}
                    <div in:fade={{ delay: 0, duration: 250 }} class="p-3">
                        <div class="flex flex-row py-2 items-center min-w-96">
                            <label for="api-key"
                                class="pr-4 pointer-events-none h-full select-nonetext-sm leading-tight text-gray-600 dark:text-gray-200 transition-all  after:scale-x-0 after:border-b-2 after:border-gray-500 after:transition-transform after:duration-300 peer-placeholder-shown:leading-tight peer-placeholder-shown:text-blue-gray-500 peer-focus:text-sm peer-focus:leading-tight peer-focus:text-gray-900 peer-focus:after:scale-x-100 peer-focus:after:border-gray-900 peer-disabled:text-transparent peer-disabled:peer-placeholder-shown:text-blue-gray-500">
                                API Key: 
                            </label>
                            <input type="password" bind:value={integration.apikey} id="api-key" style="min-width: 320px;" class="grow mr-4 bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-sky-500 focus:border-sky-500 p-2.5 dark:bg-gray-800 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-sky-500 dark:focus:border-sky-500" required> 
                            <div class="inline-flex rounded-md shadow-sm" role="group">
                                <!-- TOOD: save/ update function on click & button style-->
                                <button type="button" on:click={() => saveIntegration(i)} class="px-4 py-2 text-sm font-medium text-gray-900 bg-white border border-gray-200 rounded-s-lg hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-2 focus:ring-blue-700 focus:text-blue-700 dark:bg-gray-700 dark:border-gray-600 dark:text-white dark:hover:text-white dark:hover:bg-gray-600 dark:focus:ring-blue-500 dark:focus:text-white">
                                    Save
                                </button>
                                <!--<button type="button" class="px-4 py-2 text-sm font-medium text-gray-900 bg-white border-t border-b border-gray-200 hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-2 focus:ring-blue-700 focus:text-blue-700 dark:bg-gray-700 dark:border-gray-600 dark:text-white dark:hover:text-white dark:hover:bg-gray-600 dark:focus:ring-blue-500 dark:focus:text-white">
                                    Settings
                                </button>-->
                                <!-- TOOD: disbaled button style & test function & show checkmark after test -->
                                <button type="button" disabled={!integration.configured && integration.apikey.length == 0} on:click={() => testConnection(i)} class="px-4 py-2 text-sm font-medium text-gray-900 bg-white border border-gray-200 rounded-e-lg hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-2 focus:ring-blue-700 focus:text-blue-700 dark:bg-gray-700 dark:border-gray-600 dark:text-white dark:hover:text-white dark:hover:bg-gray-600 dark:focus:ring-blue-500 dark:focus:text-white disabled:cursor-not-allowed">
                                    Test
                                </button>
                            </div>
                        </div>
                        {#if integration.configured}
                            <p>Last Successfull Connection: 2023/02/06</p>
                        {/if}
                    </div>
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