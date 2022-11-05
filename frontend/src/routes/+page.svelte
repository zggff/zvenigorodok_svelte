<script lang="ts">
    import Map from "$lib/Map.svelte";
    import Reviews from "$lib/Reviews.svelte";
</script>

<svelte:head>
    <meta
        name="keywords"
        content="Звенигород, шиномонтаж, запись, zvenigorodok, звенигородок"
    />
    <meta name="description" content="шиномонтаж в Звенигороде" />
</svelte:head>

<main>
    <h1>шиномонтаж</h1>

    <h2>Сезонный шиномонтаж в Звенигороде для вашего удобства</h2>
    <p>Без очередей и стресса!</p>
    <p>Только по записи:<br />+7(916)-683-46-38</p>
    <p>c 8:00 до 22:00 без выходных</p>

    <div class="split">
        <div>
            <Map ref="map" />
        </div>
        <div class="certificate">
            <img src="images/certificate_small.webp" alt="Сертификат" />
        </div>
    </div>
    <p>Оборудование: Hofmann</p>
    <p>Расходные материалы: Clipper, Rema Tip-Top</p>
    <Reviews target="Tyres" />
</main>

<style lang="scss">
    :global([ref="map"]) {
        @media screen and (max-width: 800px) {
            width: 90vw;
        }
        @media screen and (min-width: 800px) {
            width: 40vw;
        }
        height: 600px;
        margin-bottom: 10px;
    }

    .split {
        width: 90vw;
        margin: auto;
        font-size: 30px;
        gap: 20px;
        flex-grow: 1;
        flex-direction: row;
        @media screen and (max-width: 800px) {
            margin: auto;
            display: block;
        }
        @media screen and (min-width: 800px) {
            margin: 0;
            display: flex;
        }
    }
    .certificate {
        img {
            border-radius: 10px;
            @media screen and (min-width: 800px) {
                height: 600px;
            }
            @media screen and (max-width: 800px) {
                width: 90vw;
            }
        }
    }
</style>
