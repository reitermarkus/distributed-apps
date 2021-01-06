const path = require('path')
const webpack = require('webpack')
const fs = require('fs')

const dotenv = require('dotenv').config({
  path: path.join(__dirname, '.env')
})

const forecastSchema = fs.readFileSync(path.resolve(__dirname, path.join('src', 'amazon_forecast_target_schema.json')), 'utf-8')

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
      'process-result': path.resolve(__dirname, path.join('src', 'process-result.ts')),
      forecast: path.resolve(__dirname, path.join('src', 'forecast.ts')),
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
      new webpack.DefinePlugin({
        'process.env.FORECAST_SCHEMA': forecastSchema
      }),
      new webpack.EnvironmentPlugin(
        Object.keys(dotenv.parsed || {})
      ),
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

