import { Socket } from "net";
import { db } from "../../..";
import { User } from "../../database/entities/user";
import OnlineUser from "../resources/onlineUser";
import TempUser from "../resources/tempUser";
import Logger from "../../util/logger";

export default class UserManager {

  public onlineUsers: OnlineUser[] = [];
  public tempUsers: TempUser[] = [];


  constructor() {
    this.onlineUsers = [];
  }

  public async onSocketCreate(socket: Socket): Promise<void> {
    const tempUser = new TempUser(socket);
    this.tempUsers.push(tempUser);

    Logger.info("UserManager", `Created temp user with ID ${tempUser.uuid}`);

  }
 
}
