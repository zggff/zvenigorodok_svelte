import adapter from '@sveltejs/adapter-static';
// import adapter from '@sveltejs/adapter-auto';
// import vercel from '@sveltejs/adapter-vercel';
import preprocess from 'svelte-preprocess';

const dev = "production" === "development";

const config = {
	// Consult https://github.com/sveltejs/svelte-preprocess
	// for more information about preprocessors
	preprocess: preprocess(),

	kit: {
		adapter: adapter({
            pages: "docs",
            assets: "docs"
        }),
        paths: {
            base: dev ? "/" : "/zvenigorodok",
        },
        
	}
};

export default config;
