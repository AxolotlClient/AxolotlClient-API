import http from 'http';
import WebSocket from 'ws';
import Logger from '../../util/logger';
import PreSocketConnection from './resources/preSocketConnection';
import SocketConnectionManager from './socketConnectionManager';

export default class WebsocketServer {

    public connectionManager = new SocketConnectionManager();
    public preConnectionInstances: Map<string, PreSocketConnection> = new Map();

    constructor(public wss: WebSocket.Server) {
        this.wss = wss;
    }

    public register(): void {

        this.wss.on("message", (message, socket) => {
            this.connectionManager.onMessage(this, socket, message);
        })

        this.wss.on("error", (error) => {
            Logger.error("WebsocketServer", error.message);
            console.error(error);
        })

    }

}
