import { memory } from "gol/gol_bg";
import { Universe, Cell } from "gol";
import fps from "./fps";

const CELL_SIZE = 8; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const universe = Universe.new();
const width = universe.width();
const height = universe.height();

const getIndex = (row, column) => row * width + column;

const canvas = document.getElementById("gol-canvas");
canvas.width = (CELL_SIZE + 1) * width + 1;
canvas.height = (CELL_SIZE + 1) * height + 1;
const ctx = canvas.getContext('2d');
ctx.strokeStyle = GRID_COLOR;

const drawGrid = () => {
    ctx.beginPath();

    // vertical lines
    const bottom = (CELL_SIZE + 1) * height + 1;
    for (let i=0; i<=width; i++) {
        const x = (CELL_SIZE + 1) * i
        ctx.moveTo(x, 0);
        ctx.lineTo(x, bottom);
    }

    //horization lines
    const right = (CELL_SIZE + 1) * width + 1;
    for (let j = 0; j <= height; j++) {
        const y = j * (CELL_SIZE + 1) + 1;
        ctx.moveTo(0, y);
        ctx.lineTo(right, y);
    }

    ctx.stroke();
};

const isBitSet = (n, arr) => {
    const byte = Math.floor(n / 8);
    const mask = 1 << (n % 8);
    return (arr[byte] & mask) === mask;
  };

const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height / 8);

    ctx.beginPath();

    for (let row=0; row < height; row++) {
        for (let col=0; col < width; col++) {
            const idx = getIndex(row, col);

            ctx.fillStyle = isBitSet(idx, cells)
                ? ALIVE_COLOR
                : DEAD_COLOR;

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            )
        }
    }

    ctx.stroke();
};

let animationId = null;
const isPaused = () => animationId === null;
const playPauseButton = document.getElementById("play-pause");
const play = () => {
    playPauseButton.textContent = "⏸";
    renderLoop();
}
const pause = () => {
    playPauseButton.textContent = "▶";
    cancelAnimationFrame(animationId);
    animationId = null;
};

playPauseButton.addEventListener("click", event => {
    const fn = isPaused() ? play : pause;
    fn();
});

const ticksPerFrameInput = document.getElementById('ticks-per-frame');
let ticksPerFrame = parseInt(ticksPerFrameInput.value, 10);
ticksPerFrameInput.addEventListener("input", event => {
    ticksPerFrame = parseInt(ticksPerFrameInput.value, 10);
});

const reset = document.getElementById('reset')
reset.addEventListener('click', () => {
    universe.reset();
});

const renderLoop = () => {
    // debugger;
    fps.render()

    drawGrid();
    drawCells();

    for (let i=0; i<ticksPerFrame; i++) {
        universe.tick();
    }

    animationId = requestAnimationFrame(renderLoop);
};

canvas.addEventListener("click", event => {
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;

    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

    universe.toggle_cell(row, col);

    drawGrid();
    drawCells();
});

play();
