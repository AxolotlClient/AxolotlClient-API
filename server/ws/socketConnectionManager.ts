import { WebSocket } from "ws";
import WebsocketServer from ".";
import { socketServer } from "..";
import Logger from "../../util/logger";
import { ClientToServerChannelTypes } from "../types";
import PreSocketConnection from "./resources/preSocketConnection";
import WebsocketConnection from "./resources/socketConnection";
import WebsocketChannel from "./resources/websocketChannel";

export default class SocketConnectionManager {
  public channels: Map<string, WebsocketChannel<any, any>> = new Map();
  public connections: Map<string, WebsocketConnection> = new Map();

  public addChannel(name: string, channel: WebsocketChannel<any, any>): void {
    this.channels.set(name, channel);
    Logger.debug("SocketConnectionManager", `Added channel ${name}`);
  }

  public getChannel(name: string): WebsocketChannel<any, any> {
    if (!this.channels.has(name)) throw new Error(`Channel ${name} does not exist!`);
    return this.channels.get(name) as WebsocketChannel<any, any>;
  }

  public onMessage(server: WebsocketServer, socket: WebSocket, message: string): void {

    if (
      Array.from(socketServer.preConnectionInstances.values())
        .map((preConnection) => preConnection.socket)
        .includes(socket)
    ) {
      const preConnection = Array.from(socketServer.preConnectionInstances.values()).find(
        (preConnection) => preConnection.socket === socket
      ) as PreSocketConnection;
      preConnection.onMessage(message);
      return;
    }

    const connection = Array.from(this.connections.values()).find(
      (connection) => connection.socket === socket
    );
    if (!connection) throw new Error("Connection not found!");

    const msg = JSON.parse(message) as ClientToServerChannelTypes;

    if (!this.channels.has(msg.type)) throw new Error(`Channel ${msg.type} does not exist!`);
    this.channels.get(msg.type)!.onMessage(connection, msg.data);

    Logger.debug(
        "SocketConnectionManager",
        `Received message from ${connection.uuid}, ID: ${connection.connectionId}, routed to ${msg.type} channel.`
    );
  }

  public isUserOnline(uuid: string): boolean {
    return this.connections.has(uuid);
  }

  public createConnection(preConnectionInstance: PreSocketConnection): WebsocketConnection {
    const connection = new WebsocketConnection(preConnectionInstance.socket, preConnectionInstance.uuid!);
    this.connections.set(connection.uuid, connection);
    socketServer.preConnectionInstances.delete(preConnectionInstance.id);
    
    Logger.debug(
      "SocketConnectionManager",
      `Created connection for ${connection.uuid}, ID: ${connection.connectionId}`
    );

    return connection;
  }

    public removeConnection(connection: WebsocketConnection): void {
        this.connections.delete(connection.uuid);

        Logger.debug(
            "SocketConnectionManager",
            `Removed connection for ${connection.uuid}, ID: ${connection.connectionId}`
        );
    }
}
