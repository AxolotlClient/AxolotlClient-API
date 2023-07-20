import { Socket } from "net";

/**
 * A user that is currently connected to the server.
 */
export default class OnlineUser {

    public uuid: string;
    public username: string;
    public socket: Socket;

    constructor(uuid: string, username: string) {
        this.uuid = uuid;
        this.username = username;
    }

}