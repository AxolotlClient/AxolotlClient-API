import rsa from "node-rsa";
import crypto from "crypto";

export default class AuthenticationManager {
  private _publicKey: Buffer;
  private _privateKey: Buffer;

  constructor() {
    // generate a 1024-bit RSA key pair, store it as ASN.1 DER
    const key = new rsa({
      b: 1024,
    });

    this._publicKey = key.exportKey("pkcs8-public-der");
    this._privateKey = key.exportKey("pkcs8-private-der");
  }

  public get publicKey(): Buffer {
    return this._publicKey;
  }

  public static hexDigest(input: string): string {
    const hash = crypto.createHash("sha1").update(input).digest();

    return BigInt.asIntN(
      160,
      hash.reduce((a, x) => (a << 8n) | BigInt(x), 0n)
    ).toString(16);
  }
}
