import { createRequire } from "node:module";
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";

const require = createRequire(import.meta.url);

global.require = require;
global.__filename = fileURLToPath(import.meta.url);
global.__dirname = dirname(__filename);

export * from "./index.js";
