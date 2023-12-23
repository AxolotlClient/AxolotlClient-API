import { socketServer } from "../..";
import Logger from "../../../util/logger";
import {
  ErrorServerToClient,
  Status,
  StatusUpdateClientToServer,
  UserClientToServer,
  UserServerToClient,
} from "../../types";
import WebsocketConnection from "../resources/socketConnection";
import WebsocketChannel from "../resources/websocketChannel";

export default class UserChannel extends WebsocketChannel<UserServerToClient, UserClientToServer> {
  /**
   *
   *  UserChannel
   *  provides user information
   *
   */

  constructor() {
    super("statusUpdate");
  }

  public async onMessage(socket: WebsocketConnection, message: UserClientToServer): Promise<void> {
    if (!message.data.hasOwnProperty("method")) {
      Logger.error(
        "UserChannel",
        `Received message from ${socket.uuid}, ID: ${socket.connectionId}, but message does not have property "method"!`
      );

      socket.send<ErrorServerToClient>({
        type: "error",
        data: {
          message: `MALFORMED_PACKET:Message missing property "method"!`,
        },
        timestamp: Date.now(),
        id: message.id,
      });
    }

    switch (message.data.method) {
      case "get": {
        const id = message.id;
        const users = message.data.users.map((user) => {
          return {
            uuid: user,
            online: Array.from(socketServer.connectionManager.connections.values())
              .map((connection) => connection.uuid)
              .includes(user),
          };
        });

        socket.send<UserServerToClient>({
          type: "user",
          data: {
            method: "get",
            users,
          },
          timestamp: Date.now(),
          id: id,
        });
      }
    }
  }
}
