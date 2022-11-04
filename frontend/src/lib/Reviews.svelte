<script lang="ts" context="module">
    export interface Review {
        user: string;
        text: string;
        date: Date;
        target: Target;
    }
    type Target = "Tyres" | "Cleaning" | "HomeMaster";
</script>

<script lang="ts">
    import { dev } from "$app/environment";
    import { onMount } from "svelte";
    const address: string = dev ? "http://localhost:8080" : "";

    export let target: Target;

    let reviews: Review[];
    let reviews_loaded = false;
    let show_reviews = false;
    const get_reviews = async () => {
        fetch(`${address}/get_reviews?target=${target.toString()}`)
            .then((response) => response.json())
            .then((data: Review[]) => {
                data.forEach((review) => (review.date = new Date(review.date)));
                data.sort((a, b) => b.date.getTime() - a.date.getTime());
                reviews = data;
                reviews_loaded = true;
            });
    };

    let error_text: string = "";
    let review: Review = {
        user: "",
        text: "",
        date: new Date(),
        target,
    };
    const add_review = async () => {
        if (review.text.length == 0) {
            error_text = "текст отзыва не может быть пустым";
            return;
        }
        if (review.user.length == 0) {
            error_text = "имя пользователя не может быть пустым";
            return;
        }
        error_text = "";
        review.date = new Date();
        await fetch(`${address}add_review`, {
            method: "POST",
            mode: "cors",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(review),
        })
            .then(async () => {
                review.text = "";
                review.user = "";
                await get_reviews();
            })
            .catch(() => {
                error_text = "не удалось добавить отзыв";
            });
    };

    onMount(get_reviews);
</script>

<main>
    <button
        class="input"
        on:click={() => {
            show_reviews = !show_reviews;
        }}
        >Показать отзывы
    </button>
    {#if show_reviews}
        <div class="review_pane">
            <input
                class="input"
                placeholder="Введите имя"
                bind:value={review.user}
            />
            <textarea
                class="input"
                placeholder="Введите текст отзыва"
                bind:value={review.text}
            />
            <button class="input" on:click={add_review}>оставить отзыв</button>
            {#if error_text != ""}
                <b class="error">{error_text}</b>
            {/if}
        </div>
        {#if reviews_loaded}
            <ul>
                {#each reviews as review}
                    <li>
                        <b>
                            {review.user} ({review.date.toLocaleDateString(
                                "ru-RU"
                            )})
                        </b>
                        <p>{review.text}</p>
                    </li>
                {/each}
            </ul>
        {/if}
    {/if}
</main>

<style lang="scss">
    button {
        width: 100%;
        margin-bottom: 2rem;
    }
    .input {
        border: 1px solid transparent;

        &:hover {
            border: 1px solid red;
        }
        font-size: 1.1rem;
        border-radius: 5px;
        min-height: 2rem;
        padding: 10px;
        background-color: white;
    }
    .review_pane {
        display: grid;
        gap: 5px;
        grid-template-areas:
            "textarea textarea textarea input"
            "textarea textarea textarea button"
            "error error error error";
        margin-bottom: 2rem;
        input {
            border: none;
            height: 2rem;
            grid-area: input;
        }
        textarea {
            resize: none;
            min-height: 6rem;
            grid-area: textarea;
        }
        button {
            margin-top: auto;
            margin-bottom: 0;
            grid-area: button;
        }
        .error {
            grid-area: error;
            color: red;
            font-size: 1rem;
        }
    }
    ul {
        padding: 0;
        margin: 0;
        font-size: 1rem;
        li {
            p {
                padding: 0;
                margin-bottom: 0;
            }
            margin-bottom: 1rem;
            background-color: white;
            border-radius: 10px;
            padding: 20px;
            list-style-type: "";
        }
    }
</style>
