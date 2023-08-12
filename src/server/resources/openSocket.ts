import { Socket } from "net";
import Logger from "../../util/logger";

export default class OpenSocket {
  public uptime: number = Date.now();
  public lastPing: number = Date.now();

  public reconnectAttempts: number = 0;
  public reconnectTimeout: number = 0;

  public remotePort: number = 0;
  public remoteAddress: string = "";

  constructor(public socket: Socket) {
    this.remotePort = socket.remotePort || 0;
    this.remoteAddress = socket.remoteAddress || "";

    socket.on("data", () => {
      this.lastPing = Date.now();
    });

    socket.on("close", (hadError) => {
      if (!hadError) return;

      socket.connect(this.remotePort, this.remoteAddress, () => {
        this.reconnectAttempts = 0;
        this.reconnectTimeout = 0;

        this.lastPing = Date.now();
        this.uptime = Date.now();
      });

      this.reconnectAttempts++;
    });

    socket.on("error", (err) => {
      Logger.error("OpenSocket", `Socket error: ${err}`);
    });
  }
}
