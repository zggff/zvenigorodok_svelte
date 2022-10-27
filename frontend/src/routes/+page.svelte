<script lang="ts">
    import Map from "$lib/Map.svelte";
    import Reviews from "$lib/Reviews.svelte";
</script>

<main>
    <div class="content">
        <h1>
            <img src="logo.svg" alt="ЗвенигородОк" />
        </h1>
        <h2>Сезонный шиномонтаж в Звенигороде для вашего удобства</h2>
        <p>Без очередей и стресса!</p>
        <p>Только по записи:<br />+7(916)-683-46-38</p>
        <p>c 8:00 до 22:00 без выходных</p>

        <div class="split">
            <div>
                <Map ref="map" />
            </div>
            <div class="certificate">
                <img src="certificate.png" alt="Сертификат" />
            </div>
        </div>
        <p>Оборудование: Hofmann</p>
        <p>Расходные материалы: Clipper, Rema Tip-Top</p>
        <Reviews />
    </div>
</main>

<style lang="scss">
    :global([ref="map"]) {
        @media screen and (max-width: 800px) {
            width: 90vw;
        }
        @media screen and (min-width: 800px) {
            width: 400px;
        }
        @media screen and (min-width: 1100px) {
            width: 600px;
        }
        /* max-width: 100hw; */
        /* width: 600px; */
        height: 600px;
        /* width: 100%; */
        max-width: 600px;
        margin-bottom: 10px;
    }

    .content {
        font-family: Helvetica, sans-serif;
        font-size: 35px;
        max-width: 90%;
        padding: 1rem;
        margin: auto;

        h1 {
            width: 80%;
            max-width: 600px;
            padding: 0;
            margin: auto;
        }

        .split {
            font-size: 30px;
            gap: 20px;
            display: flex;
            flex-grow: 1;
            flex-direction: row;
            @media screen and (max-width: 800px) {
                display: block;
            }
        }
        .certificate {
            img {
                border-radius: 10px;
                height: 600px;
                @media screen and (max-width: 800px) {
                    width: 600px;
                }
            }
        }
    }
</style>
