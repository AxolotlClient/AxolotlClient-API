import fs from "fs";
import path from "path";
// @ts-expect-error
import fzf from "node-fzf"; 

export const url = "ws://localhost:5000/api/ws";

const tests = fs.readdirSync(path.resolve(__dirname, "tests")).filter((file) => file.endsWith(".js"));

;(async () => {
    const result = await fzf({
        list: tests,
        mode: "fuzzy",
    })

    const { selected, query } = result;

    if (!selected) {
        console.log(`No test found for query: ${query}`);
        process.exit(0);
    }

    const test = require(`./tests/${selected.value}`);

})()
    