import { WebSocket } from "ws";
import { socketServer } from "../..";
import { db } from "../../..";
import { User } from "../../../database/entities/user";
import Logger from "../../../util/logger";
import MojangAPI from "../../../util/mojangApi";
import Utils from "../../../util/utils";
import { ServerToClientChannelTypes, Status, StatusUpdateServerToClient } from "../../types";

export default class WebsocketConnection {
  public socket: WebSocket;
  public status: Status = {
    online: false,
    title: "Offline",
    description: "Status is unknown",
    text: "Offline",
    icon: "offline",
  };
  public connectionId: string;
  public uuid: string;
  public username!: string;
  private user!: User;

  constructor(socket: WebSocket, uuid: string) {
    this.socket = socket;
    this.connectionId = Math.random().toString(36).substring(7);
    this.uuid = uuid;
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

        this.user = newUser;

        Logger.debug("WS", `Created new user ${this.username} (${this.uuid})`);
        return;
      }

      user.username = this.username;
      this.user = user;
      await db.getEntityManager().persistAndFlush(user);
    }

    this.user = user!;

    Logger.debug("WS", `Loaded user ${this.username} (${this.uuid})`);

    // set status
    await this.setStatus(true, {
      title : "ONLINE",
      description : "IN_MENU",
      icon : "online"
    })
  }

  public send<T = ServerToClientChannelTypes>(message: T): void {
    Logger.debug(
      "WS",
      `Sending message to ${this.username} (${this.uuid})\n${JSON.stringify(message, null, 2)}`
    );
    this.socket.send(JSON.stringify(message));
  }

  public close(): void {
    this.socket.close();
  }

  public async setStatus(online: false): Promise<void>;
  public async setStatus(
    online: true,
    data: {
      title: string;
      description: string;
      text?: string;
      icon?: string;
    }
  ): Promise<void>;
  public async setStatus(
    online: boolean,
    data: {
      title: string;
      description: string;
      text?: string;
      icon?: string;
    } = {
      title: "ONLINE",
      description: "IN_MENU"
    } 
  ): Promise<void> {
    const user = await db.getEntityManager().findOne(User, { uuid: this.uuid });
    if (!user) return;
    
    if (online) {
      this.status = {
        online,
        ...data
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

    this.broadcastToFriends<StatusUpdateServerToClient>(
      {
        type: "statusUpdate",
        id: null,
        data: {
          ... this.status,
          uuid: this.uuid,
          updateType: "online"
        },
        timestamp: Date.now()
      }
    );
  
  }

  public broadcastToFriends<T = ServerToClientChannelTypes>(message: T): void {
    this.onlineFriends.forEach((friend) => {
      friend.send(message);
    });
  }

  public get onlineFriends(): WebsocketConnection[] {
    return Array.from(socketServer.connectionManager.connections.values()).filter(
      (connection) =>
        connection.uuid !== this.uuid && connection.status.online && this.friends.includes(connection.uuid)
    );
  }

  get friends() {
    return this.user.friends;
  }

  get blocked() {
    return this.user.blocked;
  }

  get lastSeen() {
    return this.user.lastSeen;
  }

  get createdAt() {
    return this.user.createdAt;
  }

  get User() {
    return this.user;
  }
}
