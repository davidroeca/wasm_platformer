const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

const PATHS = {
  src: path.join(__dirname, 'src'),
  build: path.join(__dirname, 'build'),
};

const config = {
  mode: 'development',
  entry: [
    path.join(PATHS.src, 'index.js')
  ],
  output: {
    path: PATHS.build,
    publicPath: '/',
    filename: 'bundle.js',
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: 'index_template.html',
      filename: 'index.html',
    }),
  ],
  module: {
    rules: [
      {
        test: /\.js$/,
        enforce: 'pre',
        include: [
          PATHS.src,
        ],
        loader: 'eslint-loader',
      },
      {
        test: /\.js$/,
        include: [
          PATHS.src,
        ],
        exclude: /node_modules/,
        loader: 'babel-loader',
      },
      {
        test: /\.rs$/,
        include: [
          PATHS.src,
        ],
        use: [
          {
            loader: 'wasm-loader',
          },
          {
            loader: 'rust-native-wasm-loader',
            options: {
              'release': true
            }
          },
        ]
      }
    ]
  }
};

module.exports = config;
