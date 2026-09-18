import adapter from '@sveltejs/adapter-static';
import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vite';

export default defineConfig({
     plugins: [
          {
               name: 'svelte-css-guard',
               enforce: 'pre',
               transform(code, id) {
                    if (id.includes('.svelte') && id.includes('lang.css') && code.trimStart().startsWith('<script')) {
                         return { code: '', map: null };
                    }
               }
          },
		sveltekit({
			compilerOptions: {
				// Force runes mode for the project, except for libraries. Can be removed in svelte 6.
				runes: ({ filename }) =>
					filename.split(/[/\\]/).includes('node_modules') ? undefined : true
			},

			// adapter-auto only supports some environments, see https://svelte.dev/docs/kit/adapter-auto for a list.
			// If your environment is not supported, or you settled on a specific environment, switch out the adapter.
			// See https://svelte.dev/docs/kit/adapters for more information about adapters.
			adapter: adapter({
                    pages: 'dist',
                    assets: 'dist',
                    fallback: 'index.html',
                    precompress: false,
                    strict: false,
               }),
               prerender: {
                    handleHttpError: 'warn',
               }
          }),
		tailwindcss()
     ]
});
