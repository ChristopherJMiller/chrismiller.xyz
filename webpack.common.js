const HtmlWebpackPlugin = require("html-webpack-plugin");

module.exports = {
  entry: "./src/index.tsx",
  plugins: [
    new HtmlWebpackPlugin({
      title: "Chris M",
      meta: {
        viewport: "width=device-width, initial-scale=1, shrink-to-fit=no",
      },
      template: "./public/index.html",
      favicon: "./public/favicon.png",
    }),
  ],
  module: {
    rules: [
      {
        test: /\.(jpg|jpeg|png|webp)$/i,
        use: {
          loader: "responsive-loader",
          options: {
            adapter: require("responsive-loader/sharp"),
          },
        },
      },
      {
        test: /\.(svg|gif)$/i,
        type: "asset/resource",
      },
      {
        test: /\.(woff|woff2|eot|ttf|otf)$/i,
        type: "asset/resource",
      },
      {
        test: /\.tsx?$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: [".tsx", ".ts", ".js"],
  },
};
