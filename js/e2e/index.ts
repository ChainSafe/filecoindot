/**
 * filecoindot e2e tests
 */
import { Api } from "./src";

(async () => {
  const _ = await Api.New("ws://0.0.0.0:9944");
})();
