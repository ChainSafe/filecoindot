/**
 * setup filecoindot pallet
 *
 * 0. insert singer account
 * 1. set filecoindot rpc endpoint
 * 2. deposit fund into signer account
 * 3. add signer account as relayer
 */
import Runner, { IRunnerConfig } from "./src/runner";

// setup
(async () => {
  if (process.argv.length < 2) {
    throw "Error: config file not found";
  }

  // construct runner with `argv[2]`
  const runner = new Runner(await import(process.argv[2]));
  await runner.setup().catch((err) => {
    throw err;
  });
})();
