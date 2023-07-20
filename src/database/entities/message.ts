import { Entity, ManyToOne, PrimaryKey, Property } from "@mikro-orm/core";
import { randomUUID } from "crypto";
import { Channel } from "./channel";
import { User } from "./user";

@Entity()
export class Message {

    @PrimaryKey()
    id: string = randomUUID()

    @ManyToOne(() => User)
    user!: User;

    @ManyToOne(() => Channel)
    channel!: Channel;

    @Property()
    content!: string;

    @Property()
    timestamp: number = Date.now();

}
