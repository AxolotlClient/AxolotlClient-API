import { Socket } from "net";
import OpenSocket from "../openSocket";
import UserManager from "../../managers/userManager";

/**
 * User that is not yet authenticated.
 */
export default class TempUser {

    public id: string = Math.random().toString(36).substr(2, 9);
    public socket: OpenSocket;

    constructor(socket: Socket | OpenSocket) {
        this.socket = socket instanceof OpenSocket ? socket : new OpenSocket(socket);
    }

}