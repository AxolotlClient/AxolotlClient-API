/*
 * CHANNEL TYPES
 */

export interface BaseChannelType {
  id: string;
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
  data: Status
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
        updateType: "offline";
        uuid: string;
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
  error: {
    clientToServer: ErrorClientToServer;
    serverToClient: ErrorServerToClient;
  };
}

export type ClientToServerChannelTypes = ChannelTypes[keyof ChannelTypes]["clientToServer"];
export type ServerToClientChannelTypes = ChannelTypes[keyof ChannelTypes]["serverToClient"];

export interface Status {
  online: boolean;
  icon: string;
  title: string;
  description: string;
  text: string;
  startedAt?: number;
}
