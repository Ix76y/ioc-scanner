<script>
// @ts-nocheck

    import { invoke } from '@tauri-apps/api/tauri';

    export let ipinfoResult;

    export const showMap = () => {
        // split location for map
        if (ipinfoResult.loc) {
            var location = ipinfoResult.loc.split(",")

            // display the map
            let ipMap = document.getElementById('ip-map');
            if (ipMap) {
                var mymap = L.map('ip-map').setView([location[0], location[1]], 10);
                L.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
                    maxZoom: 19,
                    attribution: '© OpenStreetMap'
                }).addTo(mymap);
                L.marker([location[0], location[1]]).addTo(mymap);
            }
            console.log("Location: ", ipinfoResult.loc);
        }
    }

    // getIpInfo();
</script>

<!--<div class="grid grid-cols-6 grid-rows-4 grid-flow-col gap-4">
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
</div>-->

 <!-- grid-rows-4 grid-flow-col -->
    {#if ipinfoResult.ip}
    <div class="divide-y divide-dashed divide-gray-400 dark:divide-gray-600">
        <div class="grid grid-cols-2 gap-3">
            <div class="select-none cursor-default col-span-2 w-full italic font-thin text-slate-600 dark:text-slate-400 text-sm">General</div>
            <div class="">
                <div class="select-none cursor-default text-xs font-light italic text-gray-600  dark:text-gray-400">IP</div>
                <div class="">{ipinfoResult.ip}</div>
            </div>
        
            <div class="">
                <div class="select-none cursor-default text-xs font-light italic text-gray-600  dark:text-gray-400">Hostname</div>
                <div class="">{#if ipinfoResult.hostname}{ipinfoResult.hostname}{:else}-{/if}</div>
            </div>

            <div class="">
                <div class="select-none cursor-default text-xs font-light italic text-gray-600  dark:text-gray-400">Org</div>
                <div class="">{#if ipinfoResult.org}{ipinfoResult.org}{:else}-{/if}</div>
            </div>

            <div class="">
                <div class="select-none cursor-default text-xs font-light italic text-gray-600  dark:text-gray-400">Timezone</div>
                <div class="">{#if ipinfoResult.timezone}{ipinfoResult.timezone}{:else}-{/if}</div>
            </div>

            <!--{#if ipinfoResult.loc }
                <div id="ip-map" class="col-span-3 row-span-3 bg-sky-900 z-0" style="height: 200px;"></div>
            {/if}-->
            <div class="">
                <div class="select-none cursor-default text-xs font-light italic text-gray-600  dark:text-gray-400">Location</div>
                {#if ipinfoResult.city && ipinfoResult.city.length > 0}
                    <div class="">{ipinfoResult.city}, {ipinfoResult.region}, {ipinfoResult.country}</div>
                {:else}
                    <div class="">-</div>
                {/if}
            </div>

            <div class="">
                <div class="select-none cursor-default text-xs font-light italic text-gray-600  dark:text-gray-400">Other</div>
                <div class="grid grid-cols-2">
                    <div class="flex items-center mb-1 select-none cursor-default ">
                        {#if ipinfoResult.bogon}
                            <ion-icon class="text-green-600 dark:text-green-400" name="checkmark-outline"></ion-icon>
                        {:else}
                            <ion-icon class="text-red-600 dark:text-red-400" name="close-outline"></ion-icon>
                        {/if}
                        <div class="">Bogon</div>
                    </div>
                    <div class="flex items-center select-none cursor-default ">
                        {#if ipinfoResult.anycast}
                            <ion-icon class="text-green-600 dark:text-green-400" name="checkmark-outline"></ion-icon>
                        {:else}
                            <ion-icon class="text-red-600 dark:text-red-400" name="close-outline"></ion-icon>
                        {/if}
                        <div class="">Anycast</div>
                    </div>
                </div>
            </div>
        </div>

        <!-- Privacy Information -->
        <div class="my-4 pt-2">
            <div class="select-none cursor-default w-full italic font-thin text-slate-600 dark:text-slate-400 text-sm mb-3">Privacy</div>
            <div class="grid grid-cols-4 gap-3">
                <div class="flex items-center mb-1 select-none cursor-default ">
                    {#if ipinfoResult.privacy}
                        {#if ipinfoResult.privacy.vpn}
                            <ion-icon class="text-green-600 dark:text-green-400" name="checkmark-outline"></ion-icon>
                        {:else}
                            <ion-icon class="text-red-600 dark:text-red-400" name="close-outline"></ion-icon>
                        {/if}
                    {:else}
                        <ion-icon class="text-slate-600 dark:text-slate-400" name="help-outline"></ion-icon>
                    {/if}
                    <div class="">VPN</div>
                </div>
                <div class="flex items-center mb-1 select-none cursor-default ">
                    {#if ipinfoResult.privacy}
                        {#if ipinfoResult.privacy.vpn}
                            <ion-icon class="text-green-600 dark:text-green-400" name="checkmark-outline"></ion-icon>
                        {:else}
                            <ion-icon class="text-red-600 dark:text-red-400" name="close-outline"></ion-icon>
                        {/if}
                    {:else}
                        <ion-icon class="text-slate-600 dark:text-slate-400" name="help-outline"></ion-icon>
                    {/if}
                    <div class="">Proxy</div>
                </div>

                <div class="flex items-center mb-1 select-none cursor-default ">
                    {#if ipinfoResult.privacy}
                        {#if ipinfoResult.privacy.vpn}
                            <ion-icon class="text-green-600 dark:text-green-400" name="checkmark-outline"></ion-icon>
                        {:else}
                            <ion-icon class="text-red-600 dark:text-red-400" name="close-outline"></ion-icon>
                        {/if}
                    {:else}
                        <ion-icon class="text-slate-600 dark:text-slate-400" name="help-outline"></ion-icon>
                    {/if}
                    <div class="">Tor</div>
                </div>

                <div class="flex items-center mb-1 select-none cursor-default ">
                    {#if ipinfoResult.privacy}
                        {#if ipinfoResult.privacy.relay}
                            <ion-icon class="text-green-600 dark:text-green-400" name="checkmark-outline"></ion-icon>
                        {:else}
                            <ion-icon class="text-red-600 dark:text-red-400" name="close-outline"></ion-icon>
                        {/if}
                    {:else}
                        <ion-icon class="text-slate-600 dark:text-slate-400" name="help-outline"></ion-icon>
                    {/if}
                    <div class="">Relay</div>
                </div>

                <div class="flex items-center mb-1 select-none cursor-default ">
                    {#if ipinfoResult.privacy}
                        {#if ipinfoResult.privacy.hosting}
                            <ion-icon class="text-green-600 dark:text-green-400" name="checkmark-outline"></ion-icon>
                        {:else}
                            <ion-icon class="text-red-600 dark:text-red-400" name="close-outline"></ion-icon>
                        {/if}
                    {:else}
                        <ion-icon class="text-slate-600 dark:text-slate-400" name="help-outline"></ion-icon>
                    {/if}
                    <div class="">Hosting</div>
                </div>

                <div class="">
                    <div class="select-none cursor-default text-xs font-light italic text-gray-600  dark:text-gray-400">Service</div>
                    <div class="">{#if ipinfoResult.privacy}{ipinfoResult.privacy.service}{:else}-{/if}</div>
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
    <!--<div id="my-map" class="col-span-3 row-span-3 bg-sky-900 z-0" style="height: 200px;"></div>-->


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