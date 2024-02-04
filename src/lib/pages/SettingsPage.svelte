<script>
    import { invoke } from '@tauri-apps/api/tauri';

    let created = false
    async function createSecretStore() {
        created = await invoke('create_secrets_store');
        console.log("Created secrets store..." + created);
        hasSecretsStore();
    }

    let hasStore = false
    async function hasSecretsStore() {
        hasStore = await invoke('has_secrets_store');
    }

    let apiKey = "None";
    /**
     * @param {string} integration
     * @param {string} secret
     */
    async function createNewSecret(integration, secret) {
        let tmp = await invoke('update_secret', {key: integration, secret: secret})
        console.log("Result from Create New Secret: " + tmp);
    }

    /**
     * @param {string} integration
     */
    async function getSecret(integration) {
        let tmp = await invoke('get_secret', {key: integration});
        console.log("Result from get Secret: " + tmp);
        apiKey = tmp;
    }

    hasSecretsStore();
</script>

<div class="p-4 bg-zinc-100 dark:bg-zinc-900 text-zinc-900 dark:text-zinc-100">
    <div class="bg-zinc-200 dark:bg-zinc-800 rounded overflow-hidden shadow-lg p-4">
        {#if hasStore}
            <div>Congratulations! You have a place to safe your secrets!</div>
            <p>Create New Secret: </p><button on:click={() => createNewSecret('URLScan', 'HelloWorld!')}>New</button>
            <p>Retreive Secret: </p><button on:click={() => getSecret('URLScan')}>Get</button>
            <p>{apiKey}</p>
        {:else}
            <div>You don't have a secrets store yet.</div>
            <button on:click={() => createSecretStore()}>Create secret store</button>
        {/if}
        <p>Has Secrets Store: {hasStore}</p>
        <p>Created Store: {created}</p>
    </div>
</div>