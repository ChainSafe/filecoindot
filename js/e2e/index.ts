/**
 * filecoindot e2e tests
 */
import Runner from "./src/runner";

// run e2e
(async () => {
  const runner = new Runner({
    filecoindotRpc: "https://api.node.glif.io",
    id: "fdot",
    suri: "brief outside human axis reveal boat warm amateur dish sample enroll moment",
    ws: "ws://0.0.0.0:9944",
  });

  await runner.run().catch((err) => {
    throw err;
  });
})();
