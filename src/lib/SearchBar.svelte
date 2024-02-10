<script>
// @ts-nocheck
    import { createEventDispatcher } from 'svelte';

    const dispatch = createEventDispatcher();
    
    let categories = [
        'IP',
        'Domain',
        'URL',
        'Email'
    ];

    let scanInput = '';
    let currentCategory = categories[1];
    let isDropdownOpen = false;

    const handleDropdownClick = () => {
        isDropdownOpen = !isDropdownOpen;
    }

    const handleDropdownFocusLoss = ({relatedTarget, currentTarget}) => {
        console.log("TODO: Focus Loss... Not working :(");
        if (relatedTarget instanceof HTMLElement && currentTarget.contains(relatedTarget)) {
            return;
        } else {
            isDropdownOpen = false;
        }
    }

    function selectItem(category) {
        isDropdownOpen = false;
        currentCategory = category;
    }

</script>

<div class="flex flex-row w-full bg-zinc-100 dark:bg-zinc-900 text-zinc-900 dark:text-zinc-100">
    <div class="flex flex-row p-2 w-full border border-zinc-500 rounded-l">
        <svg xmlns="http://www.w3.org/2000/svg" class="ionicon" viewBox="0 0 512 512" width="24px" height="24px"><path d="M221.09 64a157.09 157.09 0 10157.09 157.09A157.1 157.1 0 00221.09 64z" fill="none" stroke="currentColor" stroke-miterlimit="10" stroke-width="32"/><path fill="none" stroke="currentColor" stroke-linecap="round" stroke-miterlimit="10" stroke-width="32" d="M338.29 338.29L448 448"/></svg>
        <input bind:value={scanInput} class="px-2 h-full w-full bg-transparent outline outline-0 transition-all outline-0" placeholder=""/>
    </div>

    <div class="relative inline-block text-left" on:focusout={handleDropdownFocusLoss}>
        <button type="button" class="inline-flex w-32 justify-between gap-x-1.5 rounded-r font-semibold shadow-sm p-2 border border-zinc-500 " on:click={handleDropdownClick}>
            {currentCategory}
            <svg class="-mr-1 mt-1 h-5 w-5 text-gray-400" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
            <path fill-rule="evenodd" d="M5.23 7.21a.75.75 0 011.06.02L10 11.168l3.71-3.938a.75.75 0 111.08 1.04l-4.25 4.5a.75.75 0 01-1.08 0l-4.25-4.5a.75.75 0 01.02-1.06z" clip-rule="evenodd" />
            </svg>
        </button>
      
        <!--
          Dropdown menu, show/hide based on menu state.
      
          Entering: "transition ease-out duration-100"
            From: "transform opacity-0 scale-95"
            To: "transform opacity-100 scale-100"
          Leaving: "transition ease-in duration-75"
            From: "transform opacity-100 scale-100"
            To: "transform opacity-0 scale-95"
        -->
        <div class="absolute right-0 z-10 mt-2 w-32 origin-top-right shadow focus:outline-none" style:visibility={isDropdownOpen ? 'visible' : 'hidden'}>
            <!-- Active: "bg-gray-100 text-gray-900", Not Active: "text-gray-700" -->
            <ul class="divide-y divide-zinc-200 dark:divide-zinc-700 text-zinc-900 dark:text-zinc-100 bg-zinc-100 dark:bg-zinc-800 shadow rounded-md ">
                {#each categories as category}
                    <!-- svelte-ignore a11y-click-events-have-key-events -->
                    <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
                    <li class="block px-4 py-2 text-sm hover:text-sky-600 hover:dark:text-sky-500 cursor-pointer" on:click={() => selectItem(category)}>{category}</li>
                {/each}
            </ul>
        </div>
    </div>

    <button on:click={() => dispatch('scan', {'input': scanInput, 'category': currentCategory})} class="px-4 py-2 ml-4 flex-none rounded bg-rose-400/25 text-white hover:bg-rose-500/25">Scan</button>
</div>