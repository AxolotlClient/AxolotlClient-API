import { WebSocket } from "ws";
import { socketServer } from "../..";

export default class PreSocketConnection {

    // handles a socket connection before it is authenticated
    // this is where we will handle the handshake and authentication

    public uuid?: string;
    public id = Math.random().toString(36).substring(7);

    constructor(public socket: WebSocket) {
        this.socket = socket;
    }

    public onMessage(message: string): void {
        // this will handle the handshake and authentication in the future, but for testing purposes we will just assign the uuid we need,
        // then we will pass the connection to the socket connection manager

        const msg = JSON.parse(message) as {
            id: string,
            type: "handshake"
            data: {
                uuid: string
            },
            timestamp: number
        } | {
            id: string,
            type: "authentication"
            data: {
                username: string,
                password: string
            },
            timestamp: number
        }

        if (msg.type === "handshake") {
            this.uuid = msg.data.uuid;
            const newConnection = socketServer.connectionManager.createConnection(this);

            newConnection.send({
                id: msg.id,
                type: "handshake",
                data: {
                    uuid: newConnection.uuid,
                    connectionId: newConnection.connectionId
                },
                timestamp: Date.now()
            })
        }


    }

}