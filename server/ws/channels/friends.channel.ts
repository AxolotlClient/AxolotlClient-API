import { WebSocket } from "ws";
import WebsocketServer from "..";
import { socketServer } from "../..";
import { db } from "../../..";
import { FriendInvite } from "../../../database/entities/friendInvite";
import { User } from "../../../database/entities/user";
import TimeUtils from "../../../util/timeUtils";
import { FriendsClientToServer, FriendsServerToClient, Status } from "../../types";
import WebsocketConnection from "../resources/socketConnection";
import WebsocketChannel from "../resources/websocketChannel";

export default class FriendsChannel extends WebsocketChannel<FriendsServerToClient, FriendsClientToServer> {
  /**
   *
   *  FriendsChannel
   *  friends, friend requests, blocked users
   *
   */

  public constructor() {
    super("friends");
  }

  public async onMessage(socket: WebsocketConnection, message: FriendsClientToServer): Promise<void> {
    const id = message.id;

    switch (message.data.method) {
      case "get": {
        const user = await db
          .getEntityManager()
          .findOne(User, { uuid: socket.uuid });
        if (!user)
          return socket.send({
            type: "error",
            id,
            data: {
              message: `User (${socket.uuid}) not found.`,
            },
          });

        const friends = (await db.getEntityManager().find(User, { uuid: socket.uuid })).map(
          async (friend) => {
            if (socketServer.connectionManager.isUserOnline(friend.uuid)) {
              const connection = socketServer.connectionManager.connections.get(
                friend.uuid
              ) as WebsocketConnection;
              const status = connection.status;

              return {
                uuid: friend.uuid,
                username: friend.username,
                status,
              };
            } else {
              return {
                uuid: friend.uuid,
                username: friend.username,
                status: {
                  online: false,
                  title: "Offline",
                  description: `Last seen ${TimeUtils.lastSeen(friend.lastSeen.getTime())}`,
                  text: "",
                  icon: "offline",
                },
              };
            }
          }
        );

        socket.send({
          type: "friends",
          id,
          data: {
            friends: await Promise.all(friends),
          },
        });
        break;
      }
      case "add": {
        const user = await db.getEntityManager().findOne(User, { uuid: socket.uuid });
        if (!user)
          return socket.send({
            type: "error",
            id,
            data: {
              message: `User (${socket.uuid}) not found.`,
            },
          });

        const friend = await db.getEntityManager().findOne(User, { uuid: message.data.uuid });
        if (!friend)
          return socket.send({
            type: "error",
            id,
            data: {
              message: `User (${message.data.uuid}) not found.`,
            },
          });

        const friends = await db.getEntityManager().find(User, { uuid: socket.uuid });

        if (friends.find((f) => f.uuid === friend.uuid))
          return socket.send({
            type: "error",
            id,
            data: {
              message: `User (${friend.uuid}) is already your friend.`,
            },
          });

        const blocked = await db.getEntityManager().find(User, { uuid: socket.uuid });

        if (blocked.find((f) => f.uuid === friend.uuid))
          return socket.send({
            type: "error",
            id,
            data: {
              message: `User (${friend.uuid}) is blocked.`,
            },
          });

        // create friend request

        const friendRequest = db.getEntityManager().create(FriendInvite, {
          from: user.uuid,
          to: friend.uuid,
          createdAt: new Date(),
        });

        await db.getEntityManager().persistAndFlush(friendRequest);

        socket.send({
          type: "friends",
          id,
          data: {
            method: "add",
            success: true,
          },
        });

        // if user is online, send friend request notification
        if (socketServer.connectionManager.isUserOnline(friend.uuid)) {
          const connection = socketServer.connectionManager.connections.get(
            friend.uuid
          ) as WebsocketConnection;

          connection.send({
            type: "friends",
            id: null,
            data: {
              method: "request",
              from: user.uuid,
              username: user.username,
            },
          });
        }
        break;
      }
    }
  }
}
