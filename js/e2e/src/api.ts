/**
 * Filecoindot api
 */
import { ApiPromise, WsProvider } from "@polkadot/api";
import { Keyring } from "@polkadot/keyring";
import { KeyringPair } from "@polkadot/keyring/types";
import types from "@filecoindot/types";

export default class Api {
  _: ApiPromise;
  signer: KeyringPair;

  /**
   * new filecoindot api
   */
  static async New(ws: string): Promise<Api> {
    const provider = new WsProvider(ws);
    const api = await ApiPromise.create({ provider, types });

    const keyring = new Keyring({ type: "sr25519" });
    const signer = keyring.addFromUri("//Alice");

    return new Api(api, signer);
  }

  constructor(api: ApiPromise, signer: KeyringPair) {
    this._ = api;
    this.signer = signer;
  }
}
