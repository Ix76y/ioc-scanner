<script>
// @ts-nocheck

    import { invoke } from '@tauri-apps/api/tauri';

    export let ipinfoResult = {
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

    function getIpInfo() {
        // split location for map
        var location = ipinfoResult.loc.split(",")

        // display the map, throws an error... not sure why :(
        /*
        var map = L.map('map').setView([location[0], location[1]], 10);
        
        L.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
            maxZoom: 19,
            attribution: '© OpenStreetMap'
        }).addTo(map);
        L.marker([location[0], location[1]]).addTo(map);*/
        console.log("Location: ", ipinfoResult.loc);
    }

    getIpInfo();
</script>

<div class="grid grid-cols-6 grid-rows-4 grid-flow-col gap-4">
    <div class="select-none cursor-default">IP:</div>
    <div class="select-none cursor-default">Hostname:</div>
    <div class="select-none cursor-default">Org:</div>
    <div class="select-none cursor-default">Timezone:</div>
    <div class="col-span-2">{ipinfoResult.ip}</div>
    <div class="col-span-2">{ipinfoResult.hostname}</div>
    <div class="col-span-2">{ipinfoResult.org}</div>
    <div class="col-span-2">{ipinfoResult.timezone}</div>
    <div id="map" class="col-span-3 row-span-3 bg-sky-900 z-0" style="height: 200px;"></div>
    {#if ipinfoResult.city.length > 0}
        <div class="col-span-2">{ipinfoResult.city}, {ipinfoResult.region}, {ipinfoResult.country}</div>
    {:else}
        <div class="col-span-2"></div>
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