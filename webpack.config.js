const path = require("path");

module.exports = {
  entry: {
    index: path.resolve("./data/client/ts/index.ts"),
    bg: path.resolve("./data/client/ts/bg.ts"),
  },
  devtool: "inline-source-map",
  output: {
    path: path.resolve("./data/client/js"),
    filename: "[name].js",
    sourceMapFilename: "[name].js.map",
  },
  resolve: {
    extensions: [".ts", ".js"],
  },
  module: {
    rules: [
      {
        test: /\.ts$/,
        use: [
          {
            loader: "ts-loader",
          },
        ],
      },
    ],
  },
  mode: "development",
};
