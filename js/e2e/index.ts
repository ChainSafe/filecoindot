/**
 * filecoindot e2e tests
 */
import Runner from "./src/runner";

// run e2e
(async () => {
  const runner = new Runner({
    addr: "0x0676a4b19c66b31e12d15fe31ccbc775d3d2cda6e1c8686e395118f808eaa118",
    filecoindotRpc: "https://api.node.glif.io",
    id: "fdot",
    suri: "brief outside human axis reveal boat warm amateur dish sample enroll moment",
    ws: "ws://0.0.0.0:9944",
  });

  await runner.run().catch((err) => {
    throw err;
  });
})();
