<script lang="ts" context="module">
    interface Review {
        user: string;
        text: string;
        date: Date;
    }
</script>

<script lang="ts">
    import { USER } from "$env/static/private";
    import { onMount } from "svelte";

    let reviews: Review[];
    let reviews_loaded = false;
    let show_reviews = true;
    const get_reviews = async () => {
        fetch("http://localhost:8080/get_reviews")
            .then((response) => response.json())
            .then((data: Review[]) => {
                data.forEach((review) => (review.date = new Date(review.date)));
                reviews = data;
                reviews_loaded = true;
            });
    };

    let review: Review = {
        user: "Ваше имя",
        text: "Введите текст отзыва",
        date: new Date(),
    };

    const add_review = async () => {
        review.date = new Date();

        const res = await fetch("http://localhost:8080/add_review", {
            method: "POST",
            mode: "cors",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(review),
        });
    };

    onMount(get_reviews);
</script>

<main>
    <button
        on:click={() => {
            show_reviews = !show_reviews;
        }}>Показать отзывы</button
    >
    {#if show_reviews}
        <div class="review_pane">
            <input bind:value={review.user} />
            <textarea bind:value={review.text} />
            <button on:click={add_review}>оставить отзыв</button>
        </div>
        {#if reviews_loaded}
            <ul>
                {#each reviews as review}
                    <li>
                        <h4>
                            {review.user} on {review.date.toLocaleString(
                                "ru-RU"
                            )}
                        </h4>
                        <p>{review.text}</p>
                    </li>
                {/each}
            </ul>
        {/if}
    {/if}
</main>

<style lang="scss">
</style>
