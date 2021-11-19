const { addBeforeLoader, loaderByName } = require('@craco/craco');
const { CracoAliasPlugin, configPaths } = require('react-app-rewire-alias');
const path = require('path');

module.exports = {
	webpack: {
		configure: (webpackConfig) => {
			const wasmExtRegex = /\.wasm$/;
			webpackConfig.resolve.extensions.push('.wasm');
			webpackConfig.resolve.symlinks = false;
			//webpackConfig.resolve.roots = [path.resolve(__dirname, 'node_modules')]
			//webpackConfig.resolveLoader.roots = [path.resolve(__dirname, 'node_modules')]

			webpackConfig.module.rules.forEach((rule) => {
				(rule.oneOf || []).forEach((oneOf) => {
					if (oneOf.loader && oneOf.loader.indexOf('file-loader') >= 0) {
						oneOf.exclude.push(wasmExtRegex);
					}
				});
			});

			const wasmLoader = {
				test: wasmExtRegex,
				exclude: /node_modules/,
				loaders: ['wasm-loader'],
			};

			addBeforeLoader(webpackConfig, loaderByName('file-loader'), wasmLoader);

			return webpackConfig;
		},
	},
	plugins: [
		{
			plugin: CracoAliasPlugin,
			options: {
				alias: configPaths('./tsconfig.paths.json'),
			},
		},
	],
	style: {
		postcss: {
			plugins: [
				require('tailwindcss'),
				require('autoprefixer'),
			],
		},
	},
};
