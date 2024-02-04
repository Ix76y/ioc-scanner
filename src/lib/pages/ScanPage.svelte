<script>
    import { invoke } from '@tauri-apps/api/tauri';

    import SearchBar from '../SearchBar.svelte';
    import IpInfo from './tools/IpInfo.svelte';
    import EmailRep from './tools/EmailRep.svelte';
    import UrlScan from './tools/UrlScan.svelte';

    var selectedTab = 0;
    var tabs = ["Summary", "URLScan.io", "EmailRep", "IPInfo.io"];

    /**
     * @param {string} tab
     */
    function switchTab(tab) {
        selectedTab = tabs.indexOf(tab);
    }

    let quota = {'day': 'No Quota retreived yet.'};
    async function getUrlScanQuota() {
        let data = await invoke('get_urlscan_quota');
        console.log(data);
        quota = JSON.parse(data);
    }

    let hasStore = false
    async function hasSecretsStore() {
        hasStore = await invoke('has_secrets_store');
    }

    hasSecretsStore();
</script>

<div class="px-4 pt-4">
    <SearchBar/>
</div>
<div class="p-4 bg-zinc-100 dark:bg-zinc-900 text-zinc-900 dark:text-zinc-100 divide-y divide-zinc-200 dark:divide-zinc-700">
    <!-- Tab Bar -->
    <div class="flex">
        {#each tabs as tab, i}
            <div class="mr-8">
            {#if i == selectedTab}
                <div class="border-b-2 border-sky-500 text-sky-500 pb-2 px-2 cursor-pointer">{tab}</div>
            {:else}
                <!-- svelte-ignore a11y-no-static-element-interactions -->
                <!-- svelte-ignore a11y-click-events-have-key-events -->
                <div class="hover:border-b hover:border-gray-500 hover:text-gray-700 hover:dark:text-gray-300 pb-2 px-2 cursor-pointer" on:click={() => switchTab(tab)}>{tab}</div>
            {/if}
            </div>
        {/each}
    </div>

    <!-- Tab Content -->
    {#if selectedTab == 0}
        <div class="bg-zinc-200 dark:bg-zinc-800 rounded overflow-hidden shadow-lg">
            <p class="text-gray-700 dark:text-gray-300 text-base p-16">
                Work In Progress...  💻
                {hasStore}
            </p>
        </div>
    {:else if selectedTab == 1}
        <UrlScan></UrlScan>
    {:else if selectedTab == 2}
        <EmailRep></EmailRep>
    {:else if selectedTab == 3}
        <IpInfo></IpInfo>
    {/if}
    <!--
    
    <div class="bg-zinc-200 dark:bg-zinc-800 rounded overflow-hidden shadow-lg">
        <p class="text-gray-700 dark:text-gray-300 text-base p-16">
        Work In Progress...  💻
        </p>
        <button on:click="{getUrlScanQuota}">Get Quota</button>
        <p>{quota.day}</p>
    </div>
    -->

</div>