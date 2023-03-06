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
        const user = await db.getEntityManager().findOne(User, { uuid: socket.uuid });
        if (!user)
          return socket.send({
            type: "error",
            id,
            data: {
              message: `USER_NOT_FOUND:${socket.uuid}`,
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
      case "getRequests": {
        const user = await db.getEntityManager().findOne(User, { uuid: socket.uuid });
        if (!user)
          return socket.send({
            type: "error",
            id,
            data: {
              message: `USER_NOT_FOUND:${socket.uuid}`,
            },
          });

        const incomingFriendRequests = await db.getEntityManager().find(FriendInvite, { to: socket.uuid });
        const outgoingFriendRequests = await db.getEntityManager().find(FriendInvite, { from: socket.uuid });

        socket.send({
          type: "friends",
          id,
          data: {
            method: "getRequests",
            incoming: incomingFriendRequests,
            outgoing: outgoingFriendRequests,
          },
        });
        break;
      }
      case "getBlocked": {
        const user = await db.getEntityManager().findOne(User, { uuid: socket.uuid });
        if (!user)
          return socket.send({
            type: "error",
            id,
            data: {
              message: `USER_NOT_FOUND:${socket.uuid}`,
            },
          });

        const blocked = await db.getEntityManager().find(User, {
          uuid: socket.blocked
        })

        socket.send({
          type: "friends",
          id,
          data: {
            method: "getBlocked",
            blocked: blocked.map((b) => b.uuid),
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
              message: `USER_NOT_FOUND:${socket.uuid}`,
            },
          });

        const friend = await db.getEntityManager().findOne(User, { uuid: message.data.uuid });
        if (!friend)
          return socket.send({
            type: "error",
            id,
            data: {
              message: `USER_NOT_FOUND:${message.data.uuid}`,
            },
          });

        const friends = await db.getEntityManager().find(User, { uuid: socket.uuid });

        if (friends.find((f) => f.uuid === friend.uuid))
          return socket.send({
            type: "error",
            id,
            data: {
              message: `USER_ALREADY_FRIENDS:${friend.uuid}`,
            },
          });

        const blocked = await db.getEntityManager().find(User, { uuid: socket.uuid });

        if (blocked.find((f) => f.uuid === friend.uuid))
          return socket.send({
            type: "error",
            id,
            data: {
              message: `USER_BLOCKED:${friend.uuid}`,
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
            },
          });
        }
        break;
      }
      case "accept": {
        const user = await db.getEntityManager().findOne(User, { uuid: socket.uuid });

        if (!user)
          return socket.send({
            type: "error",
            id,
            data: {
              message: `USER_NOT_FOUND:${socket.uuid}`,
            },
          });

        const request = await db
          .getEntityManager()
          .findOne(FriendInvite, { from: message.data.uuid, to: socket.uuid });
        if (!request)
          return socket.send({
            type: "error",
            id,
            data: {
              message: `FRIEND_REQUEST_NOT_FOUND:${message.data.uuid}`,
            },
          });

        const friend = await db.getEntityManager().findOne(User, { uuid: message.data.uuid });
        if (!friend)
          return socket.send({
            type: "error",
            id,
            data: {
              message: `USER_NOT_FOUND:${message.data.uuid}`,
            },
          });

        const friends = await db.getEntityManager().find(User, { uuid: socket.uuid });

        if (friends.find((f) => f.uuid === friend.uuid))
          return socket.send({
            type: "error",
            id,
            data: {
              message: `USER_ALREADY_FRIENDS:${friend.uuid}`,
            },
          });

        const blocked = await db.getEntityManager().find(User, { uuid: socket.uuid });

        if (blocked.find((f) => f.uuid === friend.uuid))
          return socket.send({
            type: "error",
            id,
            data: {
              message: `USER_BLOCKED:${friend.uuid}`,
            },
          });

        // add friend

        user.friends.push(friend.uuid);
        friend.friends.push(user.uuid);

        await db.getEntityManager().persistAndFlush([user, friend]);

        // delete friend request

        await db.getEntityManager().removeAndFlush(request);

        socket.send({
          type: "friends",
          id,
          data: {
            method: "accept",
            success: true,
          },
        });

        // if user is online, send friend request accepted notification
        if (socketServer.connectionManager.isUserOnline(friend.uuid)) {
          const connection = socketServer.connectionManager.connections.get(
            friend.uuid
          ) as WebsocketConnection;

          connection.send({
            type: "friends",
            id: null,
            data: {
              method: "accept",
              from: user.uuid,
            },
          });
        }
        break;
      }
      case "decline": {
        const user = await db.getEntityManager().findOne(User, { uuid: socket.uuid });

        if (!user)
          return socket.send({
            type: "error",
            id,
            data: {
              message: `USER_NOT_FOUND:${socket.uuid}`,
            },
          });

        const request = await db
          .getEntityManager()
          .findOne(FriendInvite, { from: message.data.uuid, to: socket.uuid });
        if (!request)
          return socket.send({
            type: "error",
            id,
            data: {
              message: `FRIEND_REQUEST_NOT_FOUND:${message.data.uuid}`,
            },
          });

        const friend = await db.getEntityManager().findOne(User, { uuid: message.data.uuid });
        if (!friend)
          return socket.send({
            type: "error",
            id,
            data: {
              message: `USER_NOT_FOUND:${message.data.uuid}`,
            },
          });

        // delete friend request
        db.getEntityManager().removeAndFlush(request);

        socket.send({
          type: "friends",
          id,
          data: {
            method: "decline",
            success: true,
          },
        });

        // if user is online, send friend request declined notification
        if (socketServer.connectionManager.isUserOnline(friend.uuid)) {
          const connection = socketServer.connectionManager.connections.get(
            friend.uuid
          ) as WebsocketConnection;

          connection.send({
            type: "friends",
            id: null,
            data: {
              method: "decline",
              from: user.uuid,
            },
          });
        }
      }
      case "remove": {
        const user = await db.getEntityManager().findOne(User, { uuid: socket.uuid });

        if (!user)
          return socket.send({
            type: "error",
            id,
            data: {
              message: `USER_NOT_FOUND:${socket.uuid}`,
            },
          });

        const friend = await db.getEntityManager().findOne(User, { uuid: message.data.uuid });
        if (!friend)
          return socket.send({
            type: "error",
            id,
            data: {
              message: `USER_NOT_FOUND:${message.data.uuid}`,
            },
          });

        const friends = await db.getEntityManager().find(User, { uuid: socket.uuid });

        if (!friends.find((f) => f.uuid === friend.uuid))
          return socket.send({
            type: "error",
            id,
            data: {
              message: `USER_NOT_FRIENDS:${friend.uuid}`,
            },
          });

        // remove friend

        user.friends = user.friends.filter((f) => f !== friend.uuid);
        friend.friends = friend.friends.filter((f) => f !== user.uuid);

        await db.getEntityManager().persistAndFlush([user, friend]);

        socket.send({
          type: "friends",
          id,
          data: {
            method: "remove",
            success: true,
          },
        });

        // don't notify removed user
        break;
      }
      case "block": {
        const user = await db.getEntityManager().findOne(User, { uuid: socket.uuid });

        if (!user)
          return socket.send({
            type: "error",
            id,
            data: {
              message: `USER_NOT_FOUND:${socket.uuid}`,
            },
          });

        const userToBlock = await db.getEntityManager().findOne(User, { uuid: message.data.uuid });

        if (!userToBlock)
          return socket.send({
            type: "error",
            id,
            data: {
              message: `USER_NOT_FOUND:${message.data.uuid}`,
            },
          });

        if (user.blocked.includes(userToBlock.uuid))
          return socket.send({
            type: "error",
            id,
            data: {
              message: `USER_ALREADY_BLOCKED:${userToBlock.uuid}`,
            },
          });

        // block

        user.blocked.push(userToBlock.uuid);

        await db.getEntityManager().persistAndFlush(user);

        socket.send({
          type: "friends",
          id,
          data: {
            method: "block",
            success: true,
          },
        });

        // don't notify blocked user
        break;
      }

      case "unblock": {
        const user = await db.getEntityManager().findOne(User, { uuid: socket.uuid });

        if (!user)
          return socket.send({
            type: "error",
            id,
            data: {
              message: `USER_NOT_FOUND:${socket.uuid}`,
            },
          });

        const userToUnblock = await db.getEntityManager().findOne(User, { uuid: message.data.uuid });

        if (!userToUnblock)
          return socket.send({
            type: "error",
            id,
            data: {
              message: `USER_NOT_FOUND:${message.data.uuid}`,
            },
          });

        if (!user.blocked.includes(userToUnblock.uuid))
          return socket.send({
            type: "error",
            id,
            data: {
              message: `USER_NOT_BLOCKED:${userToUnblock.uuid}`,
            },
          });

        // unblock

        user.blocked = user.blocked.filter((f) => f !== userToUnblock.uuid);

        await db.getEntityManager().persistAndFlush(user);

        socket.send({
          type: "friends",
          id,
          data: {
            method: "unblock",
            success: true,
          },
        });

        // don't notify unblocked user
        break;
      }
    }
  }
}
