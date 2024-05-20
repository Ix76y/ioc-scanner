<script>
    import { invoke } from '@tauri-apps/api/tauri';
    import { save } from '@tauri-apps/api/dialog';
    import { Store } from "tauri-plugin-store-api";

    const store = new Store("ioc-scanner.bin");
    const secureScanKey = "SecureScan";
    const secureStorePathKey = "SecureStorePath";

    let secureScan = false;
    let secureStorePath = "Not Set";

    // update store if value changes
    const secureScanChange = async () => {
        console.log(`Secure Scan changed: ${secureScan}`);
        store.set(secureScanKey, {value: secureScan});
        await store.save();
        console.log(`Saved store.`);
    }

    // show save dialog
    const saveStore = async () => {
        const filePath = await save({
            filters: []
        });

        if (filePath) {
            secureStorePath = filePath;
            store.set(secureStorePathKey, {value: filePath});
            await store.save();
            console.log(`Saved user selected file path: ${filePath}`);
        }
    }


    async function load_data() {
        let tmp = await store.get(secureScanKey);
        console.log(`Secure Scan: ${secureScan}`);
        if (tmp == null) {
            secureScan = false;
            store.set(secureScanKey, secureScan);
            await store.save();
            console.log(`Saved store.`);
        } else {
            secureScan = tmp.value;
        }

        tmp = await store.get(secureStorePathKey);
        if (tmp) {
            secureStorePath = tmp.value;
        }
    }
    load_data();
</script>

<div class="p-4 bg-zinc-100 dark:bg-zinc-900 text-zinc-900 dark:text-zinc-100">
    <div class="bg-white dark:bg-zinc-800 rounded overflow-hidden my-2 p-4 select-none cursor-default">
        <h2 class="text-xl font-extrabold select-none cursor-default">General</h2>
        <div class="flex items-center mb-4">
            <input id="default-checkbox" type="checkbox" bind:checked={secureScan} on:change={secureScanChange} value="" class="mb-3 w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600">
            <div class="ms-2 mt-2">
                <label for="default-checkbox" class="text-sm font-medium text-gray-900 dark:text-gray-300">Secure Scan</label>
                <p class="text-sm text-gray-700 dark:text-gray-500">When checked IOCs are not submitted to any tools, and this app will only do lookups.</p>
            </div>
        </div>
        <h2 class="text-xl font-extrabold select-none cursor-default mt-2 mb-1">Key Store</h2>
        <div class="flex flew-row">
            <div class="grow">
                <div class="">{secureStorePath}</div>
                <p class="text-sm text-gray-700 dark:text-gray-500">File to store all your API Keys and credentials securely.</p>
            </div>
            <button
                on:click={saveStore}
                class="align-middle select-none font-bold text-center uppercase transition-all disabled:opacity-50 disabled:shadow-none disabled:pointer-events-none text-xs py-3 px-6 rounded-xs border border-gray-900 dark:border-gray-100 text-gray-900 dark:text-gray-100 hover:opacity-75 focus:ring focus:ring-gray-300 active:opacity-[0.85] flex items-center gap-1"
                type="button">
                {#if secureStorePath == "Not Set" }
                    <ion-icon class="text-lg" name="add-outline"></ion-icon>
                    Create
                {:else}
                    <ion-icon class="text-lg" name="refresh-outline"></ion-icon>
                    Update
                {/if}
            </button>
        </div>
    </div>
</div>