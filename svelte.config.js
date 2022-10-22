import adapter from '@sveltejs/adapter-static';
// import adapter from '@sveltejs/adapter-auto';
// import vercel from '@sveltejs/adapter-vercel';
import preprocess from 'svelte-preprocess';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://github.com/sveltejs/svelte-preprocess
	// for more information about preprocessors
	preprocess: preprocess(),

	kit: {
		adapter: adapter({
            pages: "docs",
            assets: "docs"
        }),
        // adapter: vercel({
        //   edge: false,
        //   external: [],
        //   split: false
        // })

	}
};

export default config;
