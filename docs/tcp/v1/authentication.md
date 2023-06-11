# Authentication

### Steps

- C → S: Client requests the server's public key (only used to satisfy mojang)
- S → C: Server provides its public key (at least 1024 bit, RSA, DER format)
- C :    Client authenticates with mojang according to the specification, abort if unsuccessful
- C → S: Handshake
- S :    Server authenticates with mojang according to the specification
- S → C: Handshake response

### Respources

https://wiki.vg/Protocol_Encryption
