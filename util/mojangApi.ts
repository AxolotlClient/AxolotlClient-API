import fetch from "node-fetch";
import Logger from "./logger";

export default class MojangAPI {

    public static async getUsername(uuid: string): Promise<string> {
        const response = await fetch(`https://playerdb.co/api/player/minecraft/${uuid}`);
        
        if (response.status !=  200) {
            throw new Error("User not found");
        }
        
        const data = await response.json();

        Logger.debug("MojangAPI", `Got username ${data.data.player.username} for uuid ${uuid}`);

        return data.data.player.username;
    }

    public static async getUUID(username: string): Promise<string> {
        const response = await fetch(`https://playerdb.co/api/player/minecraft/${username}`)

        if (response.status !=  200) {
            throw new Error("User not found");
        }
        
        const data = await response.json();
        
        Logger.debug("MojangAPI", `Got uuid ${data.data.player.raw_id} for username ${username}`);

        return data.data.player.raw_id;
    }
}