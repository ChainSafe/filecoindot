/**
 * Filecoindot api
 */
import { ApiPromise, WsProvider } from "@polkadot/api";
import { Keyring } from "@polkadot/keyring";
import { KeyringPair } from "@polkadot/keyring/types";
import { rpc as filecoindotRpc, types } from "@filecoindot/types";
import { Text } from "@polkadot/types";
import { EventRecord, Event, Phase } from "@polkadot/types/interfaces";
import { hexToU8a } from "@polkadot/util";

// testing account
const SURI: string =
  "0x4ebb14295f95e62a865a457629a8e6d96ef5f3cf1896a9624d2e91e09f4cdc65";

/**
 * filecoindot api
 */
export default class Api {
  _: ApiPromise;
  signer: KeyringPair;
  suri: KeyringPair;

  /**
   * new filecoindot api
   */
  static async New(ws: string): Promise<Api> {
    const provider = new WsProvider(ws);
    const api = await ApiPromise.create({
      provider,
      types,
      rpc: filecoindotRpc,
    });

    const keyring = new Keyring({ type: "sr25519" });
    const signer = keyring.createFromUri("//Alice");
    const suri = keyring.addFromSeed(hexToU8a(SURI));

    return new Api(api, signer, suri);
  }

  constructor(api: ApiPromise, signer: KeyringPair, suri: KeyringPair) {
    this._ = api;
    this.signer = signer;
    this.suri = suri;
  }

  /**
   * traverse events
   */
  public async events(handler: (event: Event, phase: Phase) => void) {
    this._.query.system.events((events: EventRecord[]) => {
      events.forEach((record) => {
        const { event, phase } = record;
        handler(event, phase);
      });
    });
  }

  /**
   * 0. insert author key
   */
  public async insertAuthor(id: string, suri: string) {
    return await this._.rpc.author.insertKey("fdot", suri, this.suri.address);
  }

  /**
   * 1. set filecoindot rpc endpoint
   */
  public async setEndpoint(url: string) {
    return await (this._.rpc as any).filecoindot.setRpcEndpoint(url);
  }

  /**
   * 2. add relayer
   */
  public async addRelayer(addr: string) {
    return await this._.tx.filecoindot
      .addRelayer(addr)
      .signAndSend(this.signer);
  }
}
