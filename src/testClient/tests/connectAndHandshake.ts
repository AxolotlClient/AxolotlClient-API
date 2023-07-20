import Client from "../common/client";

console.log("Running tests...");

const client = new Client();

client.addMessageListener((message) => {
  console.log("Received message: ", message.toString());
});

client.socket.once("open", () => {
  client.send(
    JSON.stringify({
      id: `client-${Math.random().toString(16)}-handshake`,
      type: "handshake",
      data: {
        uuid: "422518f6c67547b1a5065b00cbdeaef9",
      },
    })
  );
});

// wait for 5 seconds
setTimeout(() => {
  process.exit(0);
}, 5000);
