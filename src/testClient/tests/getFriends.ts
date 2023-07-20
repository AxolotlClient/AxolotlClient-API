import Client from "../common/client";

console.log("Running tests...");

const client = new Client();

client.addMessageListener((message) => {
  console.log("Received message: ", message.toString());
});

client.socket.once("open", () => {
  client.send(
    JSON.stringify({
      id: Math.random().toString(36).substring(7),
      type: "handshake",
      data: {
        uuid: "422518f6c67547b1a5065b00cbdeaef9",
      },
    })
  );
});

setTimeout(() => {
    client.send(JSON.stringify({
        id: Math.random().toString(36).substring(7),
        type: "friends",
        data: {
            method: "get"
        },
        timestamp: Date.now()
    }))
}, 1000);


setTimeout(() => {
    process.exit(0);
    }
, 5000);
