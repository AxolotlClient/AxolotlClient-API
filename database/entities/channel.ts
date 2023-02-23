import { Collection, Entity, ManyToMany, ManyToOne, OneToMany, PrimaryKey } from "@mikro-orm/core";
import { randomUUID } from "crypto";
import { Message } from "./message";
import { User } from "./user";

@Entity()
export class Channel {
    
    @PrimaryKey()
    id: string = randomUUID()

    @OneToMany(() => Message, message => message.channel)
    messages = new Collection<Message>(this);

    @ManyToMany(() => User, user => user.channels, {
        mappedBy: user => user.channels,
        owner: true
    })
    members = new Collection<User>(this);

}
