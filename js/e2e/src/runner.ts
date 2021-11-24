/**
 * e2e runner
 */
import Api from "./api";
import Launch from "./launch";

const OCW_PREPARED = "haven't set filecoin rpc yet";

/**
 * e2e runner config
 */
export interface RunnerConfig {
  address: string;
  filecoindotRpc: string;
  id: string;
  suri: string;
  ws: string;
}

/**
 * e2e runner
 */
export default class Runner {
  config: RunnerConfig;

  constructor(config: RunnerConfig) {
    this.config = config;
  }

  public async run() {
    const { ws, filecoindotRpc, id, suri, address } = this.config;
    const ps = await Launch.launch("pipe");
    if (ps.stdout) {
      ps.stdout.on("data", async (chunk: Buffer) => {
        process.stdout.write(chunk.toString());
      });
    }

    let started: boolean = false;
    if (ps.stderr) {
      ps.stderr.on("data", async (chunk: Buffer) => {
        process.stderr.write(chunk.toString());
        if (chunk.includes(OCW_PREPARED)) {
          if (!started) {
            started = true;
            const api = await Api.New(ws);
            console.log(await api.insertAuthor(address, id, suri));
            console.log(await api.setEndpoint(filecoindotRpc));
            console.log(await api.addRelayer(address));
          }
        }
      });
    }
  }
}
