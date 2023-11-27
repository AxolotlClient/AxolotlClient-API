import { Socket } from "net";
import OpenSocket from "../openSocket";

/**
 * User that is not yet authenticated.
 */
export default class TempUser {

    public socket: OpenSocket;

    constructor(socket: Socket | OpenSocket) {
        this.socket = socket instanceof OpenSocket ? socket : new OpenSocket(socket);
    }

}