"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.db = void 0;
const fs_1 = __importDefault(require("fs"));
const path_1 = __importDefault(require("path"));
const logger_1 = __importDefault(require("./src/util/logger"));
const dotenv_1 = require("dotenv");
if (!fs_1.default.existsSync(path_1.default.resolve(".env"))) {
    logger_1.default.error("Main", "No .env file found, creating one for you...");
    fs_1.default.copyFileSync(path_1.default.resolve(".env.example"), path_1.default.resolve(".env"));
    logger_1.default.info("Main", "Please fill out the .env file and restart the server");
    process.exit(1);
}
(0, dotenv_1.config)();
logger_1.default.init();
const database_1 = __importDefault(require("./src/database"));
require("./src/server");
exports.db = new database_1.default();
//# sourceMappingURL=index.js.map