# Packet Identifier

A unique integer (uint32) to indentify packets and correctly
connect a response to the respective request to handle it.

This field should *never* equal `0` when this packet is sent from the client to the server and when this packet is a response from the server to a request from a client, while it should *always* equal `0` when this is packet is sent from the server to the client without a previous request from the client.
