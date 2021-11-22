const { addBeforeLoader, loaderByName } = require('@craco/craco');
const { CracoAliasPlugin, configPaths } = require('react-app-rewire-alias');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')

const path = require('path');

module.exports = {
	webpack: {
		configure: {
			experiments: {
				asyncWebAssembly: true,
				futureDefaults: true,
			},
		},
		plugins: {
			add: [
				new WasmPackPlugin({
					crateDirectory: path.resolve(__dirname, 'genetic-wasm'),
				}),
			],
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
