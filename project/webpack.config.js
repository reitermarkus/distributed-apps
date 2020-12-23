const path = require('path')
const webpack = require('webpack')

const dotenv = require('dotenv').config({
  path: path.join(__dirname, '.env')
})

module.exports = (env, argv) => {
  return {
    mode: argv.mode || 'development',
    target: 'node',
    devtool: 'inline-source-map',
    entry: {
      fetch_prices: path.resolve(__dirname, 'src/fetch_prices.ts'),
    },
    output: {
      filename: '[name].bundle.js',
      path: path.resolve(__dirname, 'dist'),
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

