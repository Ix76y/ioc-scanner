<script>
// @ts-nocheck

    import { invoke } from '@tauri-apps/api/tauri';

    let ipinfo = {
        "ip": "",
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
        var location = ipinfo.loc.split(",")
        // display the map
        var mymap = L.map('my-map').setView([location[0], location[1]], 10);
        L.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
            maxZoom: 19,
            attribution: '© OpenStreetMap'
        }).addTo(mymap);
        L.marker([location[0], location[1]]).addTo(mymap);
        console.log("Location: ", ipinfo.loc);
    }
</script>

<div class="bg-zinc-200 dark:bg-zinc-800 rounded overflow-hidden shadow-lg my-4">
    <h3 class="text-xl font-extrabold my-2 mx-4 pt-3">My IP</h3>
    <div class="grid grid-cols-6 grid-rows-4 grid-flow-col gap-4 m-4">
        <div class="select-none cursor-default">IP:</div>
        <div class="select-none cursor-default">Hostname:</div>
        <div class="select-none cursor-default">Org:</div>
        <div class="select-none cursor-default">Timezone:</div>
        <div class="col-span-2">{ipinfo.ip}</div>
        <div class="col-span-2">{ipinfo.hostname}</div>
        <div class="col-span-2">{ipinfo.org}</div>
        <div class="col-span-2">{ipinfo.timezone}</div>
        <div id="my-map" class="col-span-3 row-span-3 bg-sky-900 z-0" style="height: 200px;"></div>
        {#if ipinfo.city.length > 0}
            <div class="col-span-2">{ipinfo.city}, {ipinfo.region}, {ipinfo.country}</div>
        {:else}
            <div class="col-span-2"></div>
        {/if}
        <button on:click="{getIpInfo}" id="get-ipinfo-btn" class="rounded border border-sky-500 hover:border-sky-600 text-sky-500 hover:text-sky-600 p-2">Get My IP</button>
      </div>
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