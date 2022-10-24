<script lang="ts">
    import Map from "$lib/Map.svelte";
</script>

<main>
    <div class="content">
        <h1>
            <img src="logo.svg" alt="ЗвенигородОк" />
        </h1>
        <h2>Сезонный шиномонтаж для вашего удобства</h2>
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
        <!-- TODO:responses  -->
        <!-- TODO:flyer  -->
    </div>
</main>

<style lang="scss">
    :global([ref="map"]) {
        max-width: 100hw;

        width: 600px;
        height: 600px;
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
            flex-grow: 2;
            flex-direction: row;
            @media screen and (max-width: 800px) {
                display: block;
            }
        }
        .certificate {
            img {
                height: 600px;
            }
        }
    }
</style>
