<script>
// @ts-nocheck
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/tauri';

    // import { setContext, getContext } from 'svelte';

    import SearchBar from '../SearchBar.svelte';

    let IpInfo;
    let EmailRep; 
    let UrlScan;

    // load components dynamically
    onMount(async () => {
		IpInfo = (await import('./tools/IpInfo.svelte')).default;
        EmailRep = (await import('./tools/EmailRep.svelte')).default;
        UrlScan = (await import('./tools/UrlScan.svelte')).default;
	});

    // variables
    var selectedTab = 0;
    var tabs = ["Summary"];
    var tabContent = [];
   
    let quota = {'day': 'No Quota retreived yet.'};
    let hasStore = false;
    let scanResult ={};
    var uiUpdate = false;

    let ipinfoResult = {
        "ip": "",
        "hostname": "",
        "city": "",
        "region": "",
        "country": "",
        "loc": "",
        "org": "",
        "postal": "",
        "timezone": ""
    };

    //let showIPMap;

    /**
     * @param {string} tab
     */
    function switchTab(tab) {
        selectedTab = tabs.indexOf(tab);
    }

    // @ts-ignore
    async function getUrlScanQuota() {
        let data = await invoke('get_urlscan_quota');
        console.log(data);
        quota = JSON.parse(data);
    }

    async function hasSecretsStore() {
        hasStore = await invoke('has_secrets_store');
    }

    // @ts-ignore
    async function scanCallback(event) {
        let input = event.detail.input;
        let category = event.detail.category;
        let data = await invoke('scan', {input: input, category: category});
        // TODO: just for testing (to not submit URLScan scans all the time):
        /*let uuid = '90169cd3-5f06-46c5-b2d3-6c297d9c07f8';
        scanResult.urlscan = {};
        scanResult.urlscan.loading = true;
        tabs.push("URLScan.io");
        tabContent.push(UrlScan);
        tabs = tabs;
        tabContent = tabContent;
        console.log("Tabs: " + tabs + " Selected Tab: " + selectedTab + " Result: " + scanResult);
        waitForURLScanResults(uuid, 5);
        return;*/

        console.log(`Scan Result for ${input}:${category}`)
        for (const i of data) {
            console.log(`${i.integration} - Success: ${i.successfull} - Result: ${i.result}`);
            if (i.successfull) {
                tabs.push(i.integration);
                if (i.integration == 'IPInfo.io') {
                    tabContent.push(IpInfo);
                    ipinfoResult = JSON.parse(i.result);
                    scanResult.ipinfo = ipinfoResult;
                    console.log(`Updated IPInfo.io Result: ${ipinfoResult}`);
                    //showIPMap();
                    // split location for map
                    // showIPMap(ipinfoResult.loc.split(","));
                } else if (i.integration == 'URLScan.io') {
                    tabContent.push(UrlScan);
                    let uuid = i.result;
                    scanResult.urlscan = {};
                    scanResult.urlscan.loading = true;
                    waitForURLScanResults(uuid, 5);
                } else if (i.integration == 'EmailRep') {
                    tabContent.push(EmailRep);
                    scanResult.emailrep = JSON.parse(i.result);
                    //console.log('EmailRep: ' + i.result);
                }
            } else {
                // TODO: disable tab or show error message...
            }
        }
        uiUpdate = !uiUpdate;
        tabs = tabs;
        tabContent = tabContent;
        console.log("Tabs: " + tabs + " Selected Tab: " + selectedTab + " Result: " + scanResult);
    }

    async function waitForURLScanResults(uuid, retries) {
        if (retries<1) {
            console.log("Max Retries reached.");
            return;
        }
        console.log("Wait for URLScan Result, start.");
        await new Promise(r => setTimeout(r, 10000));
        console.log("Wait for URLScan Result, after sleep.");
        let result = await invoke('get_urlscan_result', {uuid: uuid});
        // console.log(result);
        try {
            scanResult.urlscan = JSON.parse(result);
            scanResult.urlscan.loading = false;
            return;
        } catch (error) {
            console.log("Error casting: " + result);
            waitForURLScanResults(uuid, retries-1);
        }
    }

    function showIPMap(location) {
        console.log('Show map at ' + location);
        // display the map
        let ipMap = document.getElementById('ip-map');
        if (ipMap) {
            var mymap = L.map('ip-map').setView([location[0], location[1]], 10);
            L.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
                maxZoom: 19,
                attribution: '© OpenStreetMap'
            }).addTo(mymap);
            L.marker([location[0], location[1]]).addTo(mymap);
        } else {
            console.log("No map found :(");
        }
        console.log("Location: ", ipinfoResult.loc);
    }

    hasSecretsStore();
</script>

<div class="px-4 pt-4">
    <SearchBar on:scan={scanCallback}/>
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
    <div class="bg-zinc-200 dark:bg-zinc-800 rounded overflow-hidden shadow-lg p-4">
        {#if selectedTab == 0}
            <div>Summary: { scanResult }</div>
        {:else }
            <svelte:component this={tabContent[selectedTab-1]} scanResult={scanResult}>
            </svelte:component>
        {/if}
    </div>
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