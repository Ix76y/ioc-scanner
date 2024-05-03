<script>
// @ts-nocheck

    import { invoke } from '@tauri-apps/api/tauri'
  
    export let scanResult;

</script> 


{#if scanResult.emailrep}
<div class="divide-y divide-dashed divide-gray-400 dark:divide-gray-600">
  <div class="grid grid-cols-3 gap-3">
    <div class="select-none cursor-default col-span-3 w-full italic font-thin text-slate-600 dark:text-slate-400 text-sm">General</div>

    <div class="">
        <div class="select-none cursor-default text-xs font-light italic text-gray-600  dark:text-gray-400">Reputation</div>
        <div class="select-none inline-flex items-center rounded bg-slate-50/25 px-2 py-1 text-xs font-medium text-slate-700 dark:text-slate-300 ring-1 ring-inset ring-slate-600/10 dark:ring-slate-400/75">{scanResult.emailrep.reputation}</div>
    </div>

    <div class="">
        <div class="select-none cursor-default text-xs font-light italic text-gray-600  dark:text-gray-400">Suspicious</div>
        {#if scanResult.emailrep.suspicious}
          <ion-icon class="text-red-600 dark:text-red-400" name="checkmark-outline"></ion-icon>
        {:else}
          <ion-icon class="text-green-600 dark:text-green-400" name="close-outline"></ion-icon>
        {/if}
    </div>

    <div class="">
        <div class="select-none cursor-default text-xs font-light italic text-gray-600  dark:text-gray-400">References</div>
        <div class="">{scanResult.emailrep.references}</div>
    </div>
  </div>

  <!-- Details -->
  <div class="my-4 pt-2">
    <div class="select-none cursor-default w-full italic font-thin text-slate-600 dark:text-slate-400 text-sm mb-3">Details</div>
      <div class="grid grid-cols-4 gap-3">
        <div class="">
          <div class="select-none cursor-default text-xs font-light italic text-gray-600  dark:text-gray-400">Blacklisted</div>
          {#if scanResult.emailrep.details.blacklisted}
            <ion-icon class="text-red-600 dark:text-red-400" name="checkmark-outline"></ion-icon>
          {:else}
            <ion-icon class="text-green-600 dark:text-green-400" name="close-outline"></ion-icon>
          {/if}
        </div>

        <div class="">
          <div class="select-none cursor-default text-xs font-light italic text-gray-600  dark:text-gray-400">Malicious Activity</div>
          {#if scanResult.emailrep.details.malicious_activity}
            <ion-icon class="text-red-600 dark:text-red-400" name="checkmark-outline"></ion-icon>
          {:else}
            <ion-icon class="text-green-600 dark:text-green-400" name="close-outline"></ion-icon>
          {/if}
        </div>

        <div class="">
          <div class="select-none cursor-default text-xs font-light italic text-gray-600  dark:text-gray-400">Spam</div>
          {#if scanResult.emailrep.details.spam}
            <ion-icon class="text-red-600 dark:text-red-400" name="checkmark-outline"></ion-icon>
          {:else}
            <ion-icon class="text-green-600 dark:text-green-400" name="close-outline"></ion-icon>
          {/if}
        </div>

        <div class="">
          <div class="select-none cursor-default text-xs font-light italic text-gray-600  dark:text-gray-400">Disposable</div>
          {#if scanResult.emailrep.details.disposable}
            <ion-icon class="text-red-600 dark:text-red-400" name="checkmark-outline"></ion-icon>
          {:else}
            <ion-icon class="text-green-600 dark:text-green-400" name="close-outline"></ion-icon>
          {/if}
        </div>

        <div class="">
          <div class="select-none cursor-default text-xs font-light italic text-gray-600  dark:text-gray-400">Credentials Leaked</div>
          {#if scanResult.emailrep.details.credentials_leaked}
            <ion-icon class="text-red-600 dark:text-red-400" name="checkmark-outline"></ion-icon>
          {:else}
            <ion-icon class="text-green-600 dark:text-green-400" name="close-outline"></ion-icon>
          {/if}
        </div>

        <div class="">
          <div class="select-none cursor-default text-xs font-light italic text-gray-600  dark:text-gray-400">Days Since Domain Creation</div>
          <div class="">{scanResult.emailrep.details.days_since_domain_creation}</div>
        </div>

      </div>
  </div>
</div>
{/if}

