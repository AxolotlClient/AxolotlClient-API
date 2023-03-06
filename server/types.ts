/*
 * CHANNEL TYPES
 */

export interface BaseChannelType {
  id: string | null;
  timestamp: number;
}

export interface FriendsServerToClient extends BaseChannelType {
  type: "friends";
  data:
    | {
        method: "get";
        friends: {
          uuid: string;
          status: Status;
        }[];
      }
    | {
        method: "getRequests";
        requests: {
          incoming: {
            uuid: string;
          }[];
          outgoing: {
            uuid: string;
          }[];
        };
      }
    | {
        method: "getBlocked";
        blocked: {
          uuid: string;
          username: string;
        }[];
      }
    | {
        method: "remove";
        uuid: string;
      }
    | {
        method: "add";
        success: true;
      }
    | {
        method: "add";
        from: string;
      }
    | {
        method: "accept";
        success: true;
      }
    | {
        method: "accept";
        from: string;
      }
    | {
        method: "decline";
        success: true;
      }
    | {
        method: "decline";
        from: string;
      }
    | {
        method: "block";
        uuid: string;
      }
    | {
        method: "unblock";
        uuid: string;
      }
    | {
        method: "error";
        message: string;
      };
}

export interface FriendsClientToServer extends BaseChannelType {
  type: "friends";
  data:
    | {
        method: "get";
      }
    | {
        method: "getRequests";
      }
    | {
        method: "getBlocked";
      }
    | {
        method: "remove";
        uuid: string;
      }
    | {
        method: "add";
        uuid: string;
      }
    | {
        method: "accept";
        uuid: string;
      }
    | {
        method: "decline";
        uuid: string;
      }
    | {
        method: "block";
        uuid: string;
      }
    | {
        method: "unblock";
        uuid: string;
      };
}

export interface StatusUpdateServerToClient extends BaseChannelType {
  type: "statusUpdate";
  data: Status & {
    updateType: "online" | "inGame" | "inGameUnknown";
    uuid: string;
  };
}

export interface StatusUpdateClientToServer extends BaseChannelType {
  type: "statusUpdate";
  data:
    | {
        updateType: "online";
        uuid: string;
        update: {
          location: string;
        };
      }
    | {
        updateType: "inGame";
        uuid: string;
        update: {
          server: string;
          gameType: string;
          gameMode: string;
          map: string;
          players: number;
          maxPlayers: number;
          startedAt: number;
        };
      }
    | {
        updateType: "inGameUnknown";
        uuid: string;
        update: {
          server: string;
          worldType: string;
          worldName: string;
          gamemode: string;
          startedAt: number;
        };
      };
}

export interface UserServerToClient extends BaseChannelType {
  type: "user";
  data: {
    method: "get";
    users: {
      uuid: string;
      online: boolean;
    }[];
  };
}

export interface UserClientToServer extends BaseChannelType {
  type: "user";
  data: {
    method: "get";
    users: string[];
  };
}

export interface ChannelServerToClient extends BaseChannelType {
  type: "channel";
  data: {
    method: "get"; // gets channel by id
    channel: Channel<"dm" | "group", "user" | "user.status" | "messages">;
  } | {
    method: "get"; // gets channels that the user is in, intended for channel list
    channels: Channel<"dm" | "group", "user" | "user.status" | "messages">[];
  } | {
    method: "get"; // gets channels that contain all users
    channels: Channel<"dm" | "group", "user" | "user.status" | "messages">[];
  } | {
    method: "getDM"; // gets DM channel between the sender and another user
    channel: Channel<"dm", "user" | "user.status" | "messages">;
  } | {
    method: "messages"; // gets latest messages from channel
    messages: Message[];
  } | {
    method: "messages"; // gets messages from channel before a certain timestamp
    messages: Message[];
  } | {
    method: "create"; // creates a new channel
    channel: Channel<"dm" | "group", "user">;
  };
}

export interface ChannelClientToServer extends BaseChannelType {
  type: "channel";
  data:
    | {
        method: "get"; // gets channel by id
        id: string;
        include?: ("users" | "users.status" | "messages")[]; // optional, defaults to []
      }
    | {
        method: "get"; // gets channels that the user is in, intended for channel list
        user: string;
        sortBy?: "alphabetical" | "lastMessage"; // optional, defaults to "lastMessage"
        include?: ("users" | "users.status" | "messages")[]; // optional, defaults to []
      }
    | {
        method: "get"; // gets channels that contain all users
        users: string[];
        sortBy: "alphabetical" | "lastMessage";
        include?: ("users" | "users.status" | "messages")[]; // optional, defaults to []
      }
    | {
        method: "getDM"; // gets DM channel between the sender and another user
        user: string;
        include?: ("users" | "users.status" | "messages")[]; // optional, defaults to []
      }
    | {
        method: "messages"; // gets latest messages from channel
        limit: number; // max 100, default 100
        include?: ("user" | "user.status")[]; // optional, defaults to []
      }
    | {
        method: "messages"; // gets messages from channel before a certain timestamp
        limit: number; // max 100, default 100
        before: number; // timestamp
        include?: ("user" | "user.status")[]; // optional, defaults to []
      }
    | {
        method: "messages"; // gets messages from channel after a certain timestamp
        limit: number; // max 100, default 100
        after: number; // timestamp
        include?: ("user" | "user.status")[]; // optional, defaults to []
      }
    | {
        method: "create"; // creates a DM channel
        type: "dm";
        users: [string, string];
      }
    | {
        method: "create"; // creates a group channel
        type: "group";
        users: string[]; // must include the user who is creating the channel
        name?: string; // optional, defaults to "name1, name2, name3, ..." | limited to 64 characters
      };
}

export interface ChatServerToClient extends BaseChannelType {
  type: "chat";
  data: {
    method: "message";
  };
}

export interface ChatClientToServer extends BaseChannelType {
  type: "chat";
  data: {
    method: "message";
    channel: string;
    message: string;
  };
}

export interface ErrorServerToClient extends BaseChannelType {
  type: "error";
  data: {
    message: string;
  };
}

export interface ErrorClientToServer extends BaseChannelType {
  type: "error";
  data: {
    message: string;
  };
}

export interface ChannelTypes {
  friends: {
    clientToServer: FriendsClientToServer;
    serverToClient: FriendsServerToClient;
  };
  statusUpdate: {
    clientToServer: StatusUpdateClientToServer;
    serverToClient: StatusUpdateServerToClient;
  };
  user: {
    clientToServer: UserClientToServer;
    serverToClient: UserServerToClient;
  };
  channel: {
    clientToServer: ChannelClientToServer;
    serverToClient: ChannelServerToClient;
  };
  chat: {
    clientToServer: ChatClientToServer;
    serverToClient: ChatServerToClient;
  };
  error: {
    clientToServer: ErrorClientToServer;
    serverToClient: ErrorServerToClient;
  };
}

export type ClientToServerChannelTypes = ChannelTypes[keyof ChannelTypes]["clientToServer"];
export type ServerToClientChannelTypes = ChannelTypes[keyof ChannelTypes]["serverToClient"];

// Other types
export interface Status {
  online: boolean;
  icon?: string;
  title: string;
  description: string;
  text?: string;
  startedAt?: number;
}

// User Type
// IncludeStatus is a boolean that determines whether or not to include the status in the user object
// if it's false, it will only include the uuid and online status
// if it's true, it will include the uuid, and status, but not the online status, as it's already included in the status object

export interface User<IncludeStatus extends boolean = false> {
  uuid: string;
  online: IncludeStatus extends false ? boolean : never;
  status: IncludeStatus extends true ? Status : never;
}

// Message Type
// Include is a string that determines what to include in the message object

// "user" - includes the user object, but not their status
// "user.status" - includes the user object with their status
// "channel" - includes the channel object

export interface Message<Include extends "user" | "user.status" | "channel" | void = void > {
  user: Include extends "user" ? User<Include extends "user.status" ? true : false> : string;
  channel: Include extends "channel" ? Channel<"dm" | "group"> : string;
  content: string;
  timestamp: number;
}

// Represents a channel, sorry for the nested ternaries, it's the only way to do it with TS

// channel types
// "dm" - direct message | users is an array of 2 users, and name is not included
// "group" - group message | users is an array of users, and name is included

// include types
// "user" - includes the user objects, but not their status
// "user.status" - includes the user objects with their status

// if neither are included, it will just be an array of user ids

// "messages" - includes the messages in the channel
// "messages.user" - includes the user objects in the messages

export type Channel<Type extends "group" | "dm", Include extends "user" | "user.status" | "messages" | "messages.user" | void = void> = {
  id: string;
  type: Type;
  name: Type extends "group" ? string : never;
  users: Type extends "group"
    ? Include extends "user"
      ? User<Include extends "user.status" ? true : false>[]
      : never
    : Include extends "user"
    ? [User<Include extends "user.status" ? true : false>, User<Include extends "user.status" ? true : false>]
    : never;
  messages: Include extends "messages" ? Message<Include extends "messages.user" ? "user" : void>[] : never;
};
