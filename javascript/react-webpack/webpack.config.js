/**
 * This is a NON-Bazel Webpack config for demonstration purposes, take a look at the webpack.bazel.config.js for the
 * config that is used by the Bazel build. The two configurations produce the same end result.
 */

const HtmlWebpackPlugin = require('html-webpack-plugin');
const path = require('path');

module.exports = {
  entry: path.resolve(__dirname, 'src', 'index.jsx'),
  mode: 'development',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'bundle.js',
  },
  module: {
    rules: [
      {
        test: /\.m?jsx?$/,
        exclude: /(node_modules)/,
        use: {
          loader: 'swc-loader',
        },
      },
    ],
  },
  plugins: [
    new HtmlWebpackPlugin({
      filename: './index.html',
      template: path.join(__dirname, 'public/index.html'),
    }),
  ],
  resolve: {
    extensions: ['', '.js', '.jsx'],
  },
};
