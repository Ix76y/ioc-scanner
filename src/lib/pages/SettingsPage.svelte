<script>
    import { invoke } from '@tauri-apps/api/tauri';
    import { Store } from "tauri-plugin-store-api";

    const store = new Store("ioc-scanner.bin");
    const secureScanKey = "SecureScan";

    let secureScan = false;

    // update store if value changes
    const secureScanChange = async () => {
        console.log(`Secure Scan changed: ${secureScan}`);
        store.set(secureScanKey, {value: secureScan});
        await store.save();
        console.log(`Saved store.`);
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
    }
    load_data();
</script>

<div class="p-4 bg-zinc-100 dark:bg-zinc-900 text-zinc-900 dark:text-zinc-100">
    <div class="bg-zinc-200 dark:bg-zinc-800 rounded overflow-hidden shadow-lg p-4">
        <h2>General</h2>
        <div class="flex items-center mb-4">
            <input id="default-checkbox" type="checkbox" bind:checked={secureScan} on:change={secureScanChange} value="" class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600">
            <div class="ms-2 mt-5">
                <label for="default-checkbox" class="text-sm font-medium text-gray-900 dark:text-gray-300">Secure Scan</label>
                <p class="text-sm text-gray-700 dark:text-gray-500">When checked IOCs are not submitted to any tools, and this app will only do lookups.</p>
            </div>
        </div>
        <h2>Summary</h2>
    </div>
</div>