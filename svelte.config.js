import adapter from '@sveltejs/adapter-static';
// import adapter from '@sveltejs/adapter-auto';
// import vercel from '@sveltejs/adapter-vercel';
import preprocess from 'svelte-preprocess';

// const dev = true

const config = {
	// Consult https://github.com/sveltejs/svelte-preprocess
	// for more information about preprocessors
	preprocess: preprocess(),

	kit: {
		adapter: adapter({
            pages: "build",
            assets: "build",
            fallback: null
        }),
        paths: {
            base: "/zvenigorodok",
        },
        
	}
};

export default config;
