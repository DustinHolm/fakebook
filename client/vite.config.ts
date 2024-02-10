import { PluginOption, defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import { transformSync } from "@swc/core";
import swc from "./.swcrc";
import relay from "vite-plugin-relay";

const _relay: PluginOption = {
  name: "vite:relay",
  transform(src, id) {
    if (/.(t|j)sx?/.test(id) && src.includes("graphql`")) {
      const out = transformSync(src, swc);

      if (!out?.code) {
        throw new Error(`vite-plugin-relay: Failed to transform ${id}`);
      }

      const code = out.code;
      const map = out.map;

      return {
        code,
        map,
      };
    }
  },
};

export default defineConfig({
  plugins: [relay, react()],
});
