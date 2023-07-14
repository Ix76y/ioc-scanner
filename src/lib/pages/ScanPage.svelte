<script>
    import { invoke } from '@tauri-apps/api/tauri';

    let quota = {'day': 'No Quota retreived yet.'};
    async function getUrlScanQuota() {
        let data = await invoke('get_urlscan_quota');
        console.log(data);
        quota = JSON.parse(data);
    }
</script>

<div class="p-4 bg-zinc-100 dark:bg-zinc-900 text-zinc-900 dark:text-zinc-100">
    <div class="bg-zinc-200 dark:bg-zinc-800 rounded overflow-hidden shadow-lg">
        <p class="text-gray-700 dark:text-gray-300 text-base p-16">
        Work In Progress...  💻
        </p>
        <button on:click="{getUrlScanQuota}">Get Quota</button>
        <p>{quota.day}</p>
    </div>
</div>