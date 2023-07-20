import chalk from "chalk";
import Utils from "./utils";

// TAKEN FROM gartutils/logger v2.1.0

const GLOAL_LOG_SETTINGS = {
  // 0 : info, 1 : warn, 2 : error
  consoleLogLevel: 0,
  debugEnabled: process.env.DEBUG === "true",
};

export default class Logger {
  public static init() {

    if (process.env.DEBUG === "true") {
      GLOAL_LOG_SETTINGS.debugEnabled = true;
    }

    if (GLOAL_LOG_SETTINGS.debugEnabled) {
      console.log(chalk.blue.bold("[DEBUG] ") + chalk.blue("Debug mode enabled"));
    }
  }

  public static debug(method: string, ...message: any[]) {
    if (GLOAL_LOG_SETTINGS.debugEnabled) {
      const col = Utils.getColor(method);
      console.log(
        chalk.blue.bold(`[DEBUG]`) + chalk.hex(col).bold(`[${method}] `) + chalk.blue(`${message.join("\n")}`)
      );
    }
  }

  public static log(method: string, ...message: any[]) {
    const col = Utils.getColor(method);
    if (GLOAL_LOG_SETTINGS.consoleLogLevel === 0)
      console.log(chalk.hex(col).bold(`[${method}] `) + chalk.green(`${message.join("\n")}`));
  }

  public static warn(method: string, ...message: any[]) {
    const col = Utils.getColor(method);
    if (GLOAL_LOG_SETTINGS.consoleLogLevel <= 1)
      console.warn(chalk.hex(col).bold(`[${method}] `) + chalk.yellow(`${message.join("\n")}`));
  }

  public static error(method: string, ...message: any[]) {
    const col = Utils.getColor(method);
    if (GLOAL_LOG_SETTINGS.consoleLogLevel <= 2)
      console.error(chalk.hex(col).bold(`[${method}] `) + chalk.red(`${message.join("\n")}`));
  }

  public static info = (method: string, ...message: any[]) => Logger.log(method, ...message);
}
