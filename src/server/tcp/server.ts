import net from "net";

export default class TCPServer {
  public server: net.Server;

  constructor(public port: number) {
    this.server = net.createServer(this.listener);
  }

  public start(): void {
    this.server.listen(this.port, () => {
      console.log(`TCP server started on port ${this.port}`);
    });
  }

  public stop(): void {
    this.server.close();
  }

  private listener(socket: net.Socket): void {
    

  }
}
