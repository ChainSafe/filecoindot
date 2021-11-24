/**
 * Filecoindot api
 */
import { ApiPromise, WsProvider } from "@polkadot/api";
import { Keyring } from "@polkadot/keyring";
import { KeyringPair } from "@polkadot/keyring/types";
import { rpc, types } from "@filecoindot/types";
import { EventRecord, Event, Phase } from "@polkadot/types/interfaces";

/**
 * filecoindot api
 */
export default class Api {
  _: ApiPromise;
  signer: KeyringPair;

  /**
   * new filecoindot api
   */
  static async New(ws: string): Promise<Api> {
    const provider = new WsProvider(ws);
    const api = await ApiPromise.create({ provider, types, rpc });

    const keyring = new Keyring({ type: "sr25519" });
    const signer = keyring.addFromUri("//Alice");

    return new Api(api, signer);
  }

  constructor(api: ApiPromise, signer: KeyringPair) {
    this._ = api;
    this.signer = signer;
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
  public async insertAuthor(addr: string, id: string, suri: string) {
    return await this._.rpc.author.insertKey(id, suri, addr);
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
