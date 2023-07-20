import { WebSocket } from "ws";
import { url } from "..";

export default class Client {

    public socket: WebSocket;
    public uuid?: string;
    public messageQueue: string[] = [];
    public requests: Map<string, (message: string) => void> = new Map();

    constructor() {
        this.socket = new WebSocket(url);

        this.socket.on("open", () => {
            console.log("connected");
            this.messageQueue.forEach(message => {
                this.socket.send(message);
            })
        })

        this.socket.on("close", (code, reason) => {
            console.log(`disconnected: ${code} ${reason.toString()}`);
        })

        this.socket.on("error", (error) => {
            console.log(error);
        })
    }

    public send(message: string): void {
        if (this.socket.readyState === WebSocket.OPEN) {
            this.socket.send(message);
        } else {
            this.messageQueue.push(message);
        }
    }

    public addMessageListener(listener: (message: string) => void): void {
        this.socket.on("message", listener);
    }
}
