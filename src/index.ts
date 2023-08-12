import fs from "fs";
import path from "path";
import Logger from "./util/logger";
import { config as dotenv } from "dotenv";

// check for env file

if (!fs.existsSync(path.resolve(".env"))) {
  Logger.error("Main", "No .env file found, creating one for you...");
  fs.copyFileSync(path.resolve(".env.example"), path.resolve(".env"));
  Logger.info("Main", "Please fill out the .env file and restart the server");
  process.exit(1);
}

// load env

dotenv();

Logger.init();

// load modules

import database from "./database";
export const db = new database();

// start server

import "./server";


import modrinthGetLatestVersion from "./util/version/modrinthVersionCheck";

modrinthGetLatestVersion().then((version) => {
  Logger.info("Main", `Latest version is ${version}`);
});
