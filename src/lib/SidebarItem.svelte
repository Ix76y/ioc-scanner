<script>
// @ts-nocheck
    import logo from '$lib/assets/logo.svg';

    export let name = "Scanner";
    export let svgPath = "";
    export let img = false;
    
    let sidebaritems = ["home", "scanner", "history", "keys", "settings"]
    // export var active = false;

    // let classListHover = ["hover:bg-gradient-to-r", "hover:from-rose-600/25", "after:hover:border-r-4", "after:hover:border-rose-500"];
    let classListActive = ["bg-rose-600/25", "after:border-r-4", "after:border-rose-500"];
    
    let items = document.getElementsByClassName("sidebar-item");

    /**
     * @param {string | CustomEvent<any>} event
     */
     function sidebarItemClicked(event) {
        for (var i = 0; i < items.length; i++) {
            items[i].classList.remove(...classListActive);
        };
        let clickedElement = event.target;
        if (clickedElement.tagName == 'LI') {
            clickedElement.classList.add(...classListActive);
        } else {
            let parent = clickedElement.parentNode;
            if (parent.tagName == 'LI') {
                parent.classList.add(...classListActive);
            }
        }

        let id = name.toLowerCase() + "-page";
        for (var i = 0; i < sidebaritems.length; i++) {
            if (sidebaritems[i] == name.toLowerCase()) {
                document.getElementById(id).style.display = 'block';
            } else {
                document.getElementById(sidebaritems[i] + "-page").style.display = 'none';
            }
        }
    }

</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div on:click={(e) => sidebarItemClicked(e)}>
    {#if img}
        <img src={logo} class="mx-auto my-8" alt="logo" width="64px" height="64px" />
    {:else}
    <li class="transition transition-bg ease-in-out sidebar-item flex flex-row select-none cursor-pointer hover:bg-gradient-to-r hover:from-rose-600/25  after:hover:border-r-4  after:hover:border-rose-500">
        <svg xmlns="http://www.w3.org/2000/svg" class="ml-4 mr-2 my-4" viewBox="0 0 512 512" width=24px height=24px>{@html svgPath}</svg>
        <p class="grow my-4">{name}</p>
    </li>
    {/if}
</div>