<script lang="ts">
    import { onMount } from 'svelte';
    import Chart from 'chart.js/auto';

    import vtLogo from '$lib/assets/vt-logo.png';
    import urlscanLogo from '$lib/assets/urlscan-logo.png';
    import ipinfoLogo from '$lib/assets/ipinfo-logo.png';
    import emailrepLogo from '$lib/assets/emailrep-logo.png';
    import abuseipdbLogo from '$lib/assets/abuseipdb-logo.png';
    import domaintoolsLogo from '$lib/assets/domaintools-logo.png';

    import IpInfo from './tools/MyIpInfo.svelte';

    const rowHeight = 32;
    const chartSize = 250;

    let integrations = [
        {'name': 'URLScan.io', 'url': 'https://urlscan.io/', 'img': urlscanLogo, 'status': 'Done'},
        {'name': 'EmailRep', 'url': 'https://emailrep.io/','img': emailrepLogo, 'status': 'Done'},
        {'name': 'IPInfo.io', 'url':'https://ipinfo.io/', 'img': ipinfoLogo, 'status': 'Done'},
        {'name': 'VirusTotal', 'url':'https://www.virustotal.com/gui/home/upload', 'img': vtLogo, 'status': 'In Progress'},
        {'name': 'IPLocation', 'url':'https://www.iplocation.net/', 'img': 'vt-logo.png', 'status': 'Not Started'},
        {'name': 'AbuseIPDB', 'url':'https://www.abuseipdb.com/', 'img': abuseipdbLogo, 'status': 'Not Started'},
        {'name': 'HaveIBeenPwned', 'url':'https://haveibeenpwned.com/', 'img': 'vt-logo.png', 'status': 'Not Started'},
        {'name': 'DomainTools', 'url':'https://www.domaintools.com', 'img': domaintoolsLogo, 'status': 'Not Started'}
    ];

    
    const changeLog = [
        {version: '0.4', description: 'URLScan.io added to scan results.'},
        {version: '0.3', description: 'IPInfo.io fully integrated: Showing general and privacy information.'},
        {version: '0.2', description: 'Added support for EmailRep and IPInfo.'},
        {version: '0.1', description: 'First alpha release of app.'},
    ]


      let integrationsCanvas: HTMLCanvasElement;
      const data = {
            labels: ['Configured', 'Not Configured', 'No Configuration Needed'],
            datasets: [
                {
                    label: 'Integrations',
                    data: [2, 4, 1],
                    backgroundColor: ['#8b5cf6', '#64748b', '#6366f1'],
                    hoverOffset: 4,
                    borderWidth: 0
                }
            ]
        };

      onMount(()=> {
        const ctx = integrationsCanvas.getContext('2d');
        // Initialize chart using default config set
        if (ctx) {
            var myChart = new Chart(ctx, {
            type: 'doughnut',
            data: data,
            options: {
                borderRadius: '30',
                responsive: true,
                cutout: '90%',
                spacing: 4,
                plugins: {
                    legend: {
                        position: 'bottom',
                        display: true,
                        labels: {
                            usePointStyle: true,
                            padding: 8,
                            /*font: {
                                size: 14
                            }*/
                        }
                    },
                    /*title: {
                        display: true,
                        text: 'My Personal integrationsCanvas'
                    }*/
                    }
                }
            });
            integrationsCanvas.style.height = chartSize  + 'px';
            integrationsCanvas.style.maxHeight = chartSize  + 'px';
            integrationsCanvas.style.width = chartSize  + 'px';
            integrationsCanvas.style.maxWidth = chartSize  + 'px';
        }
      })
</script>

<div class="p-4 bg-zinc-100 dark:bg-zinc-900 text-zinc-900 dark:text-zinc-100">
    <div class="grid grid-cols-2 gap-4">
        <div class="bg-white dark:bg-zinc-800 rounded overflow-hidden p-3">
            <h3 class="text-xl font-extrabold my-2">Integrations Added</h3>
            <div class="mx-auto">
                <canvas class="mx-auto" bind:this={integrationsCanvas} width={chartSize} height={chartSize} />
            </div>
        </div>

        <div class="bg-white dark:bg-zinc-800 rounded overflow-hidden p-3">
            <h3 class="text-xl font-extrabold my-2">Change Log</h3>
            <ul>
            {#each changeLog as change}
                <li class="text-sm font-extralight my-3">
                    <span class="rounded-full bg-sky-600 dark:bg-sky-500 text-sky-50 px-2 py-1">v{change.version}</span> {change.description}
                </li>    
            {/each}
            </ul>
        </div>
    </div>
    <IpInfo></IpInfo>
    <div>
        <h3 class="text-xl font-extrabold my-2">Integrations</h3>
        <p class="text-sm font-extralight">
            Following is a list of integrations that are either already supported or will be supported in the future and the current status.
            If you would like to have a specific tool added, open an issue on our GitHub repository. 
        </p>
        {#each integrations as integration}
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

   <!-- <div class="bg-zinc-200 dark:bg-zinc-800 rounded overflow-hidden shadow-lg">
        <p class="text-gray-700 dark:text-gray-300 text-base p-16">
        Home Page...  🏡
        </p>
    </div>-->
</div>
