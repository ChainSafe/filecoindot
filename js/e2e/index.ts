/**
 * filecoindot e2e tests
 */
import Runner from "./src/runner";

// run e2e
(async () => {
  const runner = new Runner({
    address:
      "0x0676a4b19c66b31e12d15fe31ccbc775d3d2cda6e1c8686e395118f808eaa118",
    filecoindotRpc:
      "https://1zU9AC0P1BFAsHqQbf6GkeZlruY:4dd07f0d703733799dc8b220fd351cf8@filecoin.infura.io",
    id: "fdot",
    suri: "0x4ebb14295f95e62a865a457629a8e6d96ef5f3cf1896a9624d2e91e09f4cdc65",
    ws: "ws://0.0.0.0:9944",
  });

  await runner.run().catch((err) => {
    throw err;
  });
})();
