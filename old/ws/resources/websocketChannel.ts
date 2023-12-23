import { socketServer } from "../..";
import WebsocketConnection from "./socketConnection";

export default abstract class WebsocketChannel<ServerToClient, ClientToServer> {
        
    constructor(public name: string) {
        socketServer.connectionManager.addChannel(this.name, this);
    }

    public abstract onMessage(socket: WebsocketConnection, message: ClientToServer): void

    public broadcast(message: ServerToClient): void {
        socketServer.connectionManager.getChannel(this.name).broadcast(message);
    }

    public send(socket: WebsocketConnection, message: ServerToClient): void {
        socketServer.connectionManager.getChannel(this.name).send(socket, message);
    }

    public close(socket: WebsocketConnection): void {
        socketServer.connectionManager.getChannel(this.name).close(socket);
    }

}