/**
 * e2e runner
 */
import Api from "./api";
import Launch from "./launch";
import { ChildProcess } from "child_process";
import { Event, Phase, DispatchError } from "@polkadot/types/interfaces";
import { ApiPromise } from "@polkadot/api";
import BN from "bn.js";

const OCW = "filecoindot";
const OCW_PREPARED = "haven't set filecoin rpc yet";

// Kill subprocesses
function killAll(ps: ChildProcess, exitCode: number) {
  try {
    if (ps.send && !ps.killed) {
      ps.send("exit");
    }
    ps.kill("SIGINT");
  } catch (e) {
    if ((e as any).code !== "EPERM") {
      process.stdout.write(JSON.stringify(e));
      process.exit(2);
    }
  }

  process.exit(exitCode);
}

/**
 * e2e runner config
 */
export interface RunnerConfig {
  filecoindotRpc: string[];
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
    console.log("bootstrap filecoindot template...");
    const ps = await Launch.launch("pipe");

    // bootstrap testing
    let started: boolean = false;
    this.listenStdout(ps);
    this.listenStderr(ps, started);
    this.listenExit(ps);
  }

  /**
   * check filecoindot events
   */
  private checkEvents(api: ApiPromise, event: Event, phase: Phase) {
    const maybeErr = event.data[0];
    if (maybeErr && (maybeErr as DispatchError).isModule) {
      const error = api.registry.findMetaError(
        (event.data[0] as DispatchError).asModule.toU8a()
      );
      console.log(`${error.section}.${error.method}: ${error.docs}`);
      process.exit(1);
    }

    if (event.method == "VoteCasted") {
      console.log(
        `\t${event.section}:${event.method}:: (phase=${phase.toString()})`
      );
      console.log(`\t\t${event.meta.docs.toString()}`);
      console.log("votes from ocw has been accepted!");
      process.exit(0);
    }
  }

  /**
   * init offchain worker
   */
  private async tests() {
    const { ws, filecoindotRpc, id, suri } = this.config;
    const api = await Api.New(ws, suri);
    await api.insertAuthor(id);
    await api.setEndpoint(filecoindotRpc);
    await api.addRelayer();
    await api.depositFund(1000);
    api.events(this.checkEvents);
  }

  /**
   * listen stderr
   */
  private listenStderr(ps: ChildProcess, started: boolean) {
    if (ps.stderr) {
      ps.stderr.on("data", async (chunk: Buffer) => {
        // chunk.includes(OCW) &&
          process.stderr.write(chunk.toString());
        if (!started && chunk.includes(OCW_PREPARED)) {
          await this.tests();
          started = true;
        }
      });
    }
  }

  /**
   * listen stdout
   */
  private listenStdout(ps: ChildProcess) {
    if (ps.stdout) {
      ps.stdout.on("data", async (chunk: Buffer) => {
        process.stdout.write(chunk.toString());
      });
    }
  }

  /**
   * listen the exit signal and kill all processes
   */
  private listenExit(ps: ChildProcess) {
    // kill all processes when exiting.
    process.on("exit", () => {
      killAll(ps, Number(process.exitCode));
    });

    // handle ctrl+c to trigger `exit`.
    process.on("SIGINT", () => killAll(ps, 0));
  }
}
