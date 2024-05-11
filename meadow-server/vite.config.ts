import { resolve } from "node:path";
import { defineConfig } from "vite";

import postcss from "./cfg/postcss.config";

const pages = {
    main: resolve(__dirname, "index.html"),
    register: resolve(__dirname, "account/register/index.html")
};

export default defineConfig({
    css: {postcss},
    build: {
        rollupOptions: {
            input: pages
        }
    }
});
