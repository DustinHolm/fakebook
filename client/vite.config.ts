import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import tsPaths from "vite-tsconfig-paths";

export default defineConfig({
  plugins: [
    tsPaths(),
    react({
      plugins: [
        [
          "@swc/plugin-relay",
          {
            rootDir: __dirname,
            artifactDirectory: "__generated__",
            language: "typescript",
            eagerEsModules: true,
          },
        ],
      ],
    }),
  ],
});
