<script>
    import { invoke } from '@tauri-apps/api/tauri';

	import SearchBar from '../lib/SearchBar.svelte';
    import Sidebar from '../lib/Sidebar.svelte';
    import ScanPage from '$lib/pages/ScanPage.svelte';
    import SettingsPage from '$lib/pages/SettingsPage.svelte';
    import KeysPage from '$lib/pages/KeysPage.svelte';
    import HistoryPage from '$lib/pages/HistoryPage.svelte';

    document.body.classList.add('bg-zinc-100');
    document.body.classList.add('dark:bg-zinc-900');

</script>


<div class="flex flex-row relative">
    <Sidebar />
    <div style="margin-left: 200px; width: 100%">
        <div class="px-4 pt-4">
            <SearchBar/>
        </div>
        <div id="scanner-page" style="display: block;">
            <ScanPage></ScanPage>
        </div>
        <div id="history-page" style="display: none;">
            <HistoryPage></HistoryPage>
        </div>
        <div id="keys-page" style="display: none;">
            <KeysPage></KeysPage>
        </div>
        <div id="settings-page" style="display: none;">
            <SettingsPage></SettingsPage>
        </div>

    </div>
</div>

<style>
    html, body {
        height: 100%;
    }
</style>