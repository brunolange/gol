import { memory } from "gol/gol_bg";
import { Universe, Cell } from "gol";

const CELL_SIZE = 5; // px
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

const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    for (let row=0; row < height; row++) {
        for (let col=0; col < width; col++) {
            const idx = getIndex(row, col);

            ctx.fillStyle = cells[idx] === Cell.Dead
                ? DEAD_COLOR
                : ALIVE_COLOR
            ;

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

const renderLoop = () => {
    // debugger;
    drawGrid();
    drawCells();

    universe.tick();

    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
