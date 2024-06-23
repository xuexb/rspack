/**
 * @type {import('@rspack/cli').Configuration}
 */
module.exports = {
	context: __dirname,
	entry: {
		main: "./index.ts"
	},
	module: {
		rules: [
			{
				test: /\.ts$/,
				use: [
					{
						/**
						 * @type {import('@rspack/cli').Configuration['sw']}
						 */
						loader: "builtin:swc-loader",
						options: {
							jsc: {
								parser: {
									syntax: "typescript"
								}
							},
							experimental: {
								emitIsolateDts: true
							}
						}
					}
				]
			}
		]
	}
};
