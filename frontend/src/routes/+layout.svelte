<script lang="ts">
    import Navbar from "$lib/Navbar.svelte";
    let destinations = [
        { href: "/", text: "шиномонтаж" },
        // { href: "/cleaning", text: "химчистка" },
        // { href: "/master", text: "домашний мастер" },
    ];
</script>

<svelte:head>
    <title>звенигородок</title>
    <meta charset="UTF-8" />

    <meta name="author" content="Maxim Giga <github.com/zggff>" />
</svelte:head>

<div class="main">
    <Navbar {destinations} />
    <div class="logo">
        <img src="logo.svg" alt="ЗвенигородОк" />
    </div>
    <hr />

    <div class="content">
        <slot />
    </div>
</div>

<style lang="scss">
    .logo {
        display: flex;
        width: 100%;
        max-width: 700px;
        margin: auto;
        img {
            width: 100%;
        }
    }
    :global(body) {
        font-family: Helvetica, sans-serif;
        font-size: 30px;
        background-color: whitesmoke;
        padding: 1rem;
        margin: 0;
    }
    :global(h1) {
        color: red;
        margin: auto;
        text-align: center;
    }
</style>
