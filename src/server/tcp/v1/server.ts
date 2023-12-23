import net from "net";
import { TCPServer } from "../tcpServer";
import Logger from "../../../util/logger";
import { userManager } from "../..";
import DataRouter from "../../managers/dataRouter";

export default class TCPServerV1 implements TCPServer {
  public server: net.Server;

  constructor(public port: number) {
    this.server = net.createServer(this.listener);
  }

  public start(): void {
    this.server.listen(this.port, () => {
      Logger.log(`TCPServer`, `Started on port ${this.port}`);

      DataRouter.loadMessageTypes();
    });

    this.server.on("error", (err) => {
      console.error(err);
    });

    this.server.on("close", () => {
      Logger.log(`TCPServer`, `Server stopped.`);
    });

    this.server.on("connection", (socket) => {
      Logger.log(`TCPServer`, `New connection from ${socket.remoteAddress}:${socket.remotePort}`);
    });
  }

  public stop(): void {
    this.server.close();
  }

  private listener(socket: net.Socket): void {
    userManager.onSocketCreate(socket);

    socket.on("data", (msg) => {
      console.log(msg.toString());
    })
  }

  public static getSocketId(socket: net.Socket): string {
    return `${socket.remoteAddress}:${socket.remotePort}`;
  }
}
