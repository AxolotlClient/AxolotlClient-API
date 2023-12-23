import { Collection, Entity, ManyToMany, PrimaryKey, Property } from "@mikro-orm/core";
import { Channel } from "./channel";

@Entity()
export class User {
  @PrimaryKey()
  uuid!: string;

  @Property()
  username!: string;

  @Property()
  friends: string[] = [];

  @Property()
  blocked: string[] = [];

  @ManyToMany(() => Channel, channel => channel.users, {
    mappedBy: "users",
  })
  channels = new Collection<Channel>(this);
  

  @Property()
  createdAt = new Date();

  @Property()
  lastSeen = new Date();
}
