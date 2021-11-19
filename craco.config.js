const { addBeforeLoader, loaderByName } = require('@craco/craco');
const { CracoAliasPlugin, configPaths } = require('react-app-rewire-alias');

module.exports = {
	webpack: {
		configure: (webpackConfig) => {
			const wasmExtRegex = /\.wasm$/;
			webpackConfig.resolve.extensions.push('.wasm');

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
