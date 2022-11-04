<script lang="ts">
    import Icon from "svelte-awesome";
    import bars from "svelte-awesome/icons/bars";

    export let destinations: { href: string; text: string }[];

    let is_expanded = false;
    const on_click = () => {
        is_expanded = !is_expanded;
    };

    import { page } from "$app/stores";

    // hide the expanded navbar when clicked outside of it
    function clickOutside(
        element: HTMLElement,
        callbackFunction: any
    ): { update(newCallbackFunction: any): void; destroy(): void } {
        function onClick(event: { target: any }) {
            if (!element.contains(event.target)) {
                callbackFunction();
            }
        }
        document.body.addEventListener("click", onClick);
        return {
            update(newCallbackFunction: any) {
                callbackFunction = newCallbackFunction;
            },
            destroy() {
                document.body.removeEventListener("click", onClick);
            },
        };
    }
</script>

<main>
    <div
        class="navbar"
        use:clickOutside={() => {
            if (is_expanded) {
                is_expanded = false;
            }
        }}
    >
        <div class="inner" class:hidden={is_expanded === false}>
            <div
                class="item"
                tabindex="0"
                class:selected={is_expanded === true}
                on:click={on_click}
                on:keypress={on_click}
                role="button"
            >
                <Icon scale={2} data={bars} />
            </div>
            {#each destinations as destination}
                <a
                    class:current={$page.url.pathname === destination.href}
                    on:click={on_click}
                    class="item"
                    href={destination.href}>{destination.text}</a
                >
            {/each}
        </div>
    </div>
    <div style="height: 4rem;" />
</main>

<style lang="scss">
    .navbar {
        z-index: 100;
        background-color: white;
        position: fixed;
        left: 0;
        top: 0;
        width: 100%;
        .inner {
            top: 2rem;
            padding: 0.5rem 0;
            width: 100%;
            flex-direction: column;
            display: flex;
            gap: 0.5rem;
            align-items: stretch;
            /* style the <a> */
            .item {
                padding: 0.4rem 0;
                background-color: whitesmoke;
                color: black;
                text-align: center;
                text-decoration: none;
                border-radius: 5px;
                width: 96%;
                margin: auto;
                border: 1px solid transparent;
                &:hover {
                    border: 1px solid red;
                }
            }
            /* style the topmost elemt */
            .selected {
                background-color: red;
                color: white;
            }
            /* style the <a> corresponding with the current page*/
            .current {
                color: red;
            }
        }
        /* hide navbar buttons*/
        .hidden {
            a {
                display: none;
            }
        }
    }
</style>
