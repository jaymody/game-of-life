import init, { Universe } from "./pkg/cgol_wasm.js";

const FPS = 10;
const CELL_SIZE = 5; // px
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const runWasm = async () => {
  const { memory } = await init("./pkg/cgol_wasm_bg.wasm");

  const universe = Universe.new(64, 64);
  const width = universe.nrows();
  const height = universe.ncols();

  const canvas = document.getElementById("canvas");
  canvas.height = (CELL_SIZE + 1) * height + 1;
  canvas.width = (CELL_SIZE + 1) * width + 1;

  const ctx = canvas.getContext('2d');

  const getIndex = (row, column) => {
    return row * width + column;
  };

  const draw = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
      for (let col = 0; col < width; col++) {
        const idx = getIndex(row, col);

        ctx.fillStyle = cells[idx] ? ALIVE_COLOR : DEAD_COLOR;
        ctx.fillRect(
          col * (CELL_SIZE + 1) + 1,
          row * (CELL_SIZE + 1) + 1,
          CELL_SIZE,
          CELL_SIZE
        );
      }
    }

    ctx.stroke();
  }

  const renderLoop = () => {
    setTimeout(() => {
      universe.tick();
      draw();
      requestAnimationFrame(renderLoop)
    }, 1000 / FPS);
  };

  draw();
  requestAnimationFrame(renderLoop);
};

runWasm();
