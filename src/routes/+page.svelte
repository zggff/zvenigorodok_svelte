<script lang="ts">
    import Map from "../Map.svelte";
</script>

<svelte:head>
    <title>home</title>
</svelte:head>
<main>
    <div class="content">
        <h1>
            <img src="logo.svg" alt="ЗвенигородОк" />
        </h1>
        <p1>Сезонный шиномонтаж для вашего удобства</p1>
        <p>Без очередей и стресса!</p>

        <p>Только по записи:<br />+7(916)-683-46-38</p>

        <div class="split">
            <div>
                <Map ref="map" />
                <p>оборудование Hoffman</p>
                <p>расходные материалы Reme Tip-Top</p>
            </div>
            <div class="certificate">
                <img src="certificate.png" alt="Сертификат" />
            </div>
        </div>
    </div>
</main>

<style lang="scss">
    :global(body) {
        background-color: whitesmoke;
    }
    :global([ref="map"]) {
        max-width: 600px;
        width: 90%;
        height: 500px;
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
            display: flex;
            flex-grow: 2;
            flex-direction: row;
            @media screen and (max-width: 800px) {
                display: block;
            }
        }
        .certificate {
            width: 50%;

            img {
                height: 60vh;
            }
        }
    }
</style>
