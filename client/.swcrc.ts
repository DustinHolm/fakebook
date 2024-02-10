export default {
  jsc: {
    experimental: {
      plugins: [
        [
          "@swc/plugin-relay",
          {
            rootDir: __dirname,
            language: "typescript",
            eagerEsModules: true,
          },
        ],
      ],
    },
    parser: {
      syntax: "typescript",
      tsx: true,
    },
    transform: {
      react: {
        runtime: "automatic",
      },
    },
  },
};
