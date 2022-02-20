import App from './App.svelte';
import wasm from '../wasm-syn-zeug/Cargo.toml';

const app = (async () => {
  const exports = await wasm();
  new App({
    target: document.body,
    props: {
      wasm: exports
    }
  })
})();

export default app;
