const path = require('path')
const webpack = require('webpack')

const dotenv = require('dotenv').config({
  path: path.join(__dirname, '.env')
})

module.exports = (env, argv) => {
  return {
    mode: 'production',
    target: 'node',
    amd: false,
    optimization: {
      nodeEnv: false,
      minimize: false,
      moduleIds: 'deterministic',
      chunkIds: 'deterministic',
      mangleExports: true,
      concatenateModules: true,
      innerGraph: true,
      sideEffects: true
    },
    node: false,
    devtool: 'inline-source-map',
    entry: {
      fetch_prices: path.resolve(__dirname, path.join('src', 'fetch_prices.ts')),
      'create-chart': path.resolve(__dirname, path.join('src', 'create-chart.ts')),
    },
    output: {
      filename: '[name].bundle.js',
      path: path.resolve(__dirname, 'dist'),
      libraryTarget: "commonjs2",
      strictModuleExceptionHandling: true,
      sourceMapFilename: '[name].js.map',
      publicPath: '/dist/'
    },
    plugins: [
      new webpack.EnvironmentPlugin(Object.keys(dotenv.parsed || {})),
    ],
    module: {
      rules: [
        {
          test: /\.tsx?$/,
          use: 'ts-loader',
          exclude: /node_modules/,
        },
      ]
    },
  }
}

