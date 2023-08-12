<script>
    import { invoke } from '@tauri-apps/api/tauri'
  
    let email = 'info@barracudabyte.de'
    let result = ''
  
    async function checkEmail() {
      result = await invoke('get_emailrep', { email })
    }
</script> 

<button on:click="{checkEmail}" id="get-emailrep-btn" class="rounded border border-sky-500 hover:border-sky-600 text-sky-500 hover:text-sky-600 p-2">Get Email Reputation</button>

<p>Email: {email}</p>
<p>This is the result from Email Rep: {result}</p>