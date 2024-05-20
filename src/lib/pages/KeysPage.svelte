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
        let success = await invoke('test_connection', {key: integrations[i].name, secret: integrations[i].secret});
        console.log(`Connection to ${integrations[i].name}: ${success}`);
    }

    /**
     * @param {number} i
     */
    async function saveIntegration(i) {
        let integration = integrations[i];
        let saved = await invoke('update_secret', {key: integration.name, secret: integration.secret});
        if (integration.secret_type == "User") {
            let tmp = await invoke('update_secret', {key: integration.name + '-User', secret: integration.user});
            console.log(`Saved ${integration.user}: ${tmp}`);
        }
        console.log("Key successfully saved! Getting integrations again to update UI?");
        getIntegrations();
    }


    getIntegrations();
</script>

<div class="p-4 bg-zinc-100 dark:bg-zinc-900 text-zinc-900 dark:text-zinc-100">
    <div class="flex justify-between">
        <h3 class="text-xl font-extrabold my-2 select-none cursor-default">Integrations</h3>
        <!--{#if hasStore }
            <button><ion-icon name="add" class="text-3xl text-sky-500 hover:text-sky-600"></ion-icon></button>
        {/if}-->
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
                            {#if integration.secret_type == "User"}
                                <div class="relative h-11 w-full min-w-[200px] mr-4">
                                    <input placeholder="" type="" bind:value={integration.username} id="username" 
                                    class="peer h-full w-full border-b border-gray-200 bg-transparent pt-4 pb-1.5 text-sm text-gray-700 dark:text-gray-300 outline outline-0 transition-all placeholder-shown:border-gray-200 focus:border-gray-900 focus:outline-0 disabled:border-0 disabled:bg-gray-50" />
                                    <label for="username"
                                    class="after:content[' '] pointer-events-none absolute left-0 -top-2.5 flex h-full w-full select-none !overflow-visible truncate text-sm leading-tight text-gray-500 dark:text-gray-300 transition-all after:absolute after:-bottom-2.5 after:block after:w-full after:scale-x-0 after:border-b-2 after:border-gray-500 after:transition-transform after:duration-300 peer-placeholder-shown:leading-tight peer-placeholder-shown:text-gray-500 peer-focus:text-sm peer-focus:leading-tight peer-focus:text-gray-900 dark:peer-focus:text-gray-100 peer-focus:after:scale-x-100 peer-focus:after:border-gray-900 dark:peer-focus:after:border-gray-100 peer-disabled:text-transparent peer-disabled:peer-placeholder-shown:text-gray-500">
                                        Username
                                    </label>
                                </div>
                            {/if}
                            <div class="relative h-11 w-full min-w-[200px] mr-4">
                                <input placeholder="" type="password" bind:value={integration.secret} id="api-key" 
                                  class="peer h-full w-full border-b border-gray-200 bg-transparent pt-4 pb-1.5 text-sm text-gray-700 dark:text-gray-300 outline outline-0 transition-all placeholder-shown:border-gray-200 focus:border-gray-900 focus:outline-0 disabled:border-0 disabled:bg-gray-50" />
                                <label for="api-key"
                                  class="after:content[' '] pointer-events-none absolute left-0 -top-2.5 flex h-full w-full select-none !overflow-visible truncate text-sm leading-tight text-gray-500 dark:text-gray-300 transition-all after:absolute after:-bottom-2.5 after:block after:w-full after:scale-x-0 after:border-b-2 after:border-gray-500 after:transition-transform after:duration-300 peer-placeholder-shown:leading-tight peer-placeholder-shown:text-gray-500 peer-focus:text-sm peer-focus:leading-tight peer-focus:text-gray-900 dark:peer-focus:text-gray-100 peer-focus:after:scale-x-100 peer-focus:after:border-gray-900 dark:peer-focus:after:border-gray-100 peer-disabled:text-transparent peer-disabled:peer-placeholder-shown:text-gray-500">
                                {#if integration.secret_type == "ApiKey"}
                                    API Key
                                {:else if integration.secret_type == "Token"}
                                    Token
                                {:else }
                                    Password
                                {/if}
                                </label>
                            </div>
                            <!--<label for="api-key"
                                class="pr-4 pointer-events-none h-full select-nonetext-sm leading-tight text-gray-600 dark:text-gray-200 transition-all  after:scale-x-0 after:border-b-2 after:border-gray-500 after:transition-transform after:duration-300 peer-placeholder-shown:leading-tight peer-placeholder-shown:text-gray-500 peer-focus:text-sm peer-focus:leading-tight peer-focus:text-gray-900 peer-focus:after:scale-x-100 peer-focus:after:border-gray-900 peer-disabled:text-transparent peer-disabled:peer-placeholder-shown:text-gray-500">
                                {#if integration.secret_type == "ApiKey"}
                                    API Key:
                                {:else}
                                    Token:
                                {/if}
                            </label>
                            <input type="password" bind:value={integration.secret} id="api-key" style="min-width: 320px;" class="grow mr-4 bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-sky-500 focus:border-sky-500 p-2.5 dark:bg-gray-800 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-sky-500 dark:focus:border-sky-500" required> 
                            -->
                            <div class="inline-flex shadow-sm" role="group">
                                <!-- TOOD: save/ update function on click & button style-->
                                <button type="button" on:click={() => saveIntegration(i)} class="align-middle select-none font-bold text-center uppercase transition-all disabled:opacity-50 disabled:shadow-none disabled:pointer-events-none text-xs py-3 px-6 rounded-xs border border-gray-900 dark:border-gray-100 text-gray-900 dark:text-gray-100 hover:opacity-75 focus:ring focus:ring-gray-300 active:opacity-[0.85] flex items-center gap-1">
                                    Save
                                </button>
                                <!--<button type="button" class="px-4 py-2 text-sm font-medium text-gray-900 bg-white border-t border-b border-gray-200 hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-2 focus:ring-blue-700 focus:text-blue-700 dark:bg-gray-700 dark:border-gray-600 dark:text-white dark:hover:text-white dark:hover:bg-gray-600 dark:focus:ring-blue-500 dark:focus:text-white">
                                    Settings
                                </button>-->
                                <!-- TOOD: disbaled button style & test function & show checkmark after test -->
                                <button type="button" disabled={!integration.configured && integration.secret.length == 0} on:click={() => testConnection(i)} class="align-middle select-none font-bold text-center uppercase transition-all disabled:opacity-50 disabled:shadow-none disabled:pointer-events-none text-xs py-3 px-6 rounded-xs border border-gray-900 dark:border-gray-100 text-gray-900 dark:text-gray-100 hover:opacity-75 focus:ring focus:ring-gray-300 active:opacity-[0.85] flex items-center gap-1">
                                    Test
                                </button>
                            </div>
                        </div>
                        {#if integration.configured}
                            <p class="text-sm text-gray-700 dark:text-gray-500">Last Successfull Connection: 2023/02/06</p>
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