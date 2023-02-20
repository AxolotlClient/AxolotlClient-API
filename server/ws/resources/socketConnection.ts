import { WebSocket } from "ws";
import { socketServer } from "../..";
import { db } from "../../..";
import { User } from "../../../database/entities/user";
import Logger from "../../../util/logger";
import MojangAPI from "../../../util/mojangApi";
import Utils from "../../../util/utils";
import { ServerToClientChannelTypes, Status } from "../../types";

export default class WebsocketConnection {
  public socket: WebSocket;
  public status: Status = {
    online: false,
    title: "Offline",
    description: "Status is unknown",
    text: "Offline",
    icon: "offline",
  };
  public username?: string;
  public uuid: string;
  public connectionId: string;

  constructor(socket: WebSocket, uuid: string) {
    this.socket = socket;
    this.uuid = uuid;
    this.connectionId = Math.random().toString(36).substring(7);

    this.loadData();

    socket.once("close", () => {
      socketServer.connectionManager.removeConnection(this);
    });
  }

  public async loadData() {
    const user = await db.getEntityManager().findOne(User, { uuid: this.uuid });
    this.username = await MojangAPI.getUsername(this.uuid);

    Logger.info("WS", `User ${this.username} (${this.uuid}) connected.`);

    if (!user || user.username !== this.username) {
      if (!user) {
        const newUser = new User();
        newUser.uuid = this.uuid;
        newUser.username = this.username;
        await db.getEntityManager().persistAndFlush(newUser);
        return;
      }

      
      user.username = this.username;
      await db.getEntityManager().persistAndFlush(user);
    }
  }

  public send<T = ServerToClientChannelTypes>(message: T): void {
    Logger.debug("WS", `Sending message to ${this.username} (${this.uuid})\n${JSON.stringify(message, null, 2)}`);
    this.socket.send(JSON.stringify(message));
  }

  public close(): void {
    this.socket.close();
  }

  public async setStatus(online: false): Promise<void>;
  public async setStatus(
    online: true,
    title: string,
    description: string,
    text: string,
    icon: string
  ): Promise<void>;
  public async setStatus(
    online: boolean,
    title?: string,
    description?: string,
    text?: string,
    icon?: string
  ): Promise<void> {
    const user = await db.getEntityManager().findOne(User, { uuid: this.uuid });
    if (!user) return;
    const onlineFriends = Array.from(socketServer.connectionManager.connections.values()).filter(
      (connection) =>
        connection.uuid !== this.uuid &&
        connection.status.online &&
        user.friends.map((friend) => friend).includes(connection.uuid)
    );

    if (online) {
      this.status = {
        online,
        title: title!,
        description: description!,
        text: text!,
        icon: icon!,
      };
    } else {
      this.status = {
        online,
        title: "Offline",
        description: `${this.username} is offline.`,
        text: "Offline",
        icon: "offline",
      };
    }

    onlineFriends.forEach((friend) => {
      friend.send({
        type: "statusUpdate",
        id: Utils.randomKey(6),
        data: {
          uuid: this.uuid,
          status: this.status,
        },
      });
    });
  }
}
