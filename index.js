import init from "./pkg/hello_world.js";

const runWasm = async () => {
  const helloWorld = await init("./pkg/hello_world_bg.wasm");
  const addResult = helloWorld.add(24, 24);
  document.body.textContent = `Hello World! addResult: ${addResult}`;
};

runWasm();
