/**
 * Filecoindot api
 */
import { ApiPromise, WsProvider } from "@polkadot/api";
import { Keyring } from "@polkadot/keyring";
import { KeyringPair } from "@polkadot/keyring/types";
import { rpc as filecoindotRpc, types } from "@chainsafe/filecoindot-types";
import { EventRecord, Event, Phase } from "@polkadot/types/interfaces";
import { Balance } from "@polkadot/types/interfaces/runtime";
import { BN, u8aToHex } from "@polkadot/util";

/**
 * filecoindot api
 */
export default class Api {
  _: ApiPromise;
  suri: string;
  keyring: Keyring;
  signer: KeyringPair;
  testSigner: KeyringPair;

  /**
   * new filecoindot api
   */
  static async New(ws: string, suri: string): Promise<Api> {
    const provider = new WsProvider(ws);
    const api = await ApiPromise.create({
      provider,
      types,
      rpc: filecoindotRpc,
    });

    const keyring = new Keyring({ type: "sr25519" });
    const signer = keyring.createFromUri("//Alice");
    const testSigner = keyring.addFromMnemonic(suri);

    return new Api(api, suri, keyring, signer, testSigner);
  }

  constructor(
    api: ApiPromise,
    suri: string,
    keyring: Keyring,
    signer: KeyringPair,
    testSigner: KeyringPair
  ) {
    this._ = api;
    this.suri = suri;
    this.keyring = keyring;
    this.signer = signer;
    this.testSigner = testSigner;
  }

  /**
   * traverse events
   */
  public async events(
    handler: (api: ApiPromise, event: Event, phase: Phase) => void
  ) {
    this._.query.system.events((events: EventRecord[]) => {
      events.forEach((record) => {
        const { event, phase } = record;
        handler(this._, event, phase);
      });
    });
  }

  /**
   * 0. insert author key
   */
  public async insertAuthor(id: string) {
    return await this._.rpc.author.insertKey(
      id,
      this.suri,
      u8aToHex(this.testSigner.addressRaw)
    );
  }

  /**
   * 1. set filecoindot rpc endpoint
   */
  public async setEndpoint(url: string) {
    return await (this._.rpc as any).filecoindot.setRpcEndpoint(url);
  }

  /**
   * 2. depoit some fund to the testing account
   */
  public async depositFund(value: number) {
    const UNIT: Balance = this._.createType(
      "Balance",
      Math.pow(10, this._.registry.chainDecimals[0])
    );
    return await this._.tx.balances
      .transfer(this.testSigner.address, UNIT.mul(new BN(value)))
      .signAndSend(this.signer, { nonce: 0 });
  }

  /**
   * 3. add relayer
   */
  public async addRelayer() {
    return await this._.tx.sudo
      .sudo(this._.tx.filecoindot.addRelayer(this.testSigner.address))
      .signAndSend(this.signer, { nonce: 1 });
  }
}
