<script>
// @ts-nocheck

    import { invoke } from '@tauri-apps/api/tauri';

    let ipinfo = {
        "ip": "?.?.?.?",
        "hostname": "",
        "city": "",
        "region": "",
        "country": "",
        "loc": "",
        "org": "",
        "postal": "",
        "timezone": ""
    }

    async function getIpInfo() {
        // button
        let btn = document.getElementById("get-ipinfo-btn");
        btn?.classList.add("cursor-progress");

        // get data
        let data = await invoke('get_ipinfo', {ip: ''});

        // reset button
        btn?.classList.remove("cursor-progress");
        btn?.classList.add("cursor-auto");
        
        // load data
        ipinfo = JSON.parse(data);
        console.log(`IPInfo data: ${ipinfo}`)
        /*
        var location = ipinfo.loc.split(",")
        // display the map
        var mymap = L.map('my-map').setView([location[0], location[1]], 10);
        L.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
            maxZoom: 19,
            attribution: '© OpenStreetMap'
        }).addTo(mymap);
        L.marker([location[0], location[1]]).addTo(mymap);
        console.log("Location: ", ipinfo.loc);
        */
    }
</script>

<div class="bg-zinc-200 dark:bg-zinc-800 rounded overflow-hidden shadow-lg my-4">
    <h3 class="text-xl font-extrabold my-2 mx-4 pt-3">Your IP</h3>
    <div class="grid grid-cols-1 gap-3 m-4">
        <div class="">
            <div class="select-none cursor-default text-xs font-light italic text-gray-600  dark:text-gray-400">IP</div>
            <div class="">{ipinfo.ip}</div>
        </div>
        {#if ipinfo.hostname}
        <div class="">
            <div class="select-none cursor-default text-xs font-light italic text-gray-600  dark:text-gray-400">Hostname</div>
            <div class="">{ipinfo.hostname}</div>
        </div>
        {/if}
        {#if ipinfo.org}
        <div class="">
            <div class="select-none cursor-default text-xs font-light italic text-gray-600  dark:text-gray-400">Org</div>
            <div class="">{ipinfo.org}</div>
        </div>
        {/if}
        {#if ipinfo.timezone}
        <div class="">
            <div class="select-none cursor-default text-xs font-light italic text-gray-600  dark:text-gray-400">Timezone</div>
            <div class="">{ipinfo.timezone}</div>
        </div>
        {/if}
        {#if ipinfo.city}
        <div class="">
            <div class="select-none cursor-default text-xs font-light italic text-gray-600  dark:text-gray-400">Location</div>
            {#if ipinfo.city.length > 0}
                <div class="">{ipinfo.city}, {ipinfo.region}, {ipinfo.country}</div>
            {:else}
                <div class=""></div>
            {/if}
        </div>
        {/if}
        <!--<div id="my-map" class="col-span-3 row-span-3 bg-sky-900 z-0" style="height: 200px;"></div>-->
    </div>
    {#if ipinfo.ip == "?.?.?.?"}
    <div class="flex flex-row-reverse">
        <button on:click="{getIpInfo}" id="get-ipinfo-btn" class="rounded border-b-4 border-sky-500 bg-zinc-100/25 hover:bg-sinz-100/50 dark:bg-zinc-700/25 hover:dark:bg-zinc-700/50 hover:border-sky-600 hover:dark:border-sky-400 text-sky-500 hover:text-sky-600 hover:dark:text-sky-400 p-2 mx-3 mb-3">Get My IP</button>
    </div>
    {/if}
</div>

<!--
    {
  "ip": "81.103.62.93",
  "hostname": "cpc104716-belf11-2-0-cust604.2-1.cable.virginm.net",
  "org": "AS5089 Virgin Media Limited",
  -"city": "Belfast",
  -"region": "Northern Ireland",
  -"country": "GB",
  -"loc": "54.5810,-5.9665",
  - "postal": "BT11",
  "timezone": "Europe/London",
  "readme": "https://ipinfo.io/missingauth"
}%  

IP                  |
Hostname            |
ORG                 |
timezone            Belfast, BT 11, NOrthern Ireland, GB
-->