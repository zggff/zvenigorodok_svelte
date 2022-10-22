<script>
    export let ref;
    let map_loaded = false;
    let center = [55.746309, 36.878061];

    const loadMap = () => {
        let myMap = new ymaps.Map("map", {
            center: center,
            zoom: 16,
        });
        const points = myMap.geoObjects;
        points.add(
            new ymaps.Placemark(
                center,
                { balloonContent: "шиномонтаж ЗвенигородОк" },
                {}
            )
        );
        map_loaded = true;
    };
    const load = () => {
        ymaps.ready(loadMap);
    };
</script>

<svelte:head>
    <script
        async
        defer
        src="https://api-maps.yandex.ru/2.1/?lang=ru_RU&amp;apikey=75f279d9-379b-42cf-af2d-f7f5df98d242"
        on:load={load}
    >
    </script>
</svelte:head>

<main>
    {#if map_loaded === false}
        <h3>Карта загружается...</h3>
    {/if}

    <div id="map" {ref} />
</main>
