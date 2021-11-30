/**
 * filecoindot e2e tests
 */
import Runner from "./src/runner";

// run e2e
(async () => {
  const runner = new Runner({
    filecoindotRpc: "https://api.node.glif.io",
    id: "fdot",
    suri: "0x4ebb14295f95e62a865a457629a8e6d96ef5f3cf1896a9624d2e91e09f4cdc65",
    ws: "ws://0.0.0.0:9944",
  });

  await runner.run().catch((err) => {
    throw err;
  });
})();
