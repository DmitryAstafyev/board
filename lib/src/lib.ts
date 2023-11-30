import * as Core from "core";
import * as Types from "./types";

export * from "./types";

export const wasm: {
    core: typeof Core | undefined;
} = {
    core: undefined,
};

const GLOBALS: {
    id: number;
} = {
    id: 0,
};

function getId(): string {
    GLOBALS.id += 1;
    return `__board_canvas_id_${GLOBALS.id}`;
}

// Loading wasm module
import("core")
    .then((core: typeof Core) => {
        wasm.core = core;
    })
    .catch((err: Error) => {
        console.error(`Fail to core load wasm module: ${err.message}`);
    });

export class Board {
    protected readonly board: Core.Board;
    protected readonly canvas: HTMLCanvasElement;
    protected readonly parent: HTMLElement;
    protected readonly id: string;
    protected readonly size: {
        height: number;
        width: number;
    } = {
        height: 0,
        width: 0,
    };
    protected readonly position: {
        x: number;
        y: number;
        zoom: number;
    } = {
        x: 0,
        y: 0,
        zoom: 1,
    };
    protected readonly movement: {
        x: number;
        y: number;
        processing: boolean;
    } = {
        x: 0,
        y: 0,
        processing: false,
    };

    constructor(parent: string | HTMLElement) {
        const node: HTMLElement | null = (() => {
            if (typeof parent === "string") {
                return document.querySelector(parent);
            } else {
                return parent;
            }
        })();
        if (wasm.core === undefined) {
            throw new Error(`wasm module isn't yet loaded`);
        }
        if (node === null || node === undefined) {
            throw new Error(
                `Cannot get access to parent HTMLElement; selector type: ${typeof parent}; ${
                    typeof parent === "string"
                        ? `selector: ${parent} isn't valid`
                        : ""
                }`
            );
        }
        this.parent = node;
        this.id = getId();
        this.canvas = document.createElement("canvas");
        this.canvas.setAttribute("id", this.id);
        this.parent.appendChild(this.canvas);
        this.setSize();
        this.board = new wasm.core.Board();
        this.board.bind(this.id);
        this.onMouseDown = this.onMouseDown.bind(this);
        this.onMouseMove = this.onMouseMove.bind(this);
        this.onMouseUp = this.onMouseUp.bind(this);
        this.onWheel = this.onWheel.bind(this);
        this.parent.addEventListener("mousedown", this.onMouseDown);
        this.parent.addEventListener("wheel", this.onWheel);
        window.addEventListener("mousemove", this.onMouseMove);
        window.addEventListener("mouseup", this.onMouseUp);
    }

    protected setSize(): void {
        const size = this.parent.getBoundingClientRect();
        this.size.width = size.width;
        this.size.height = size.height;
        this.canvas.width = size.width;
        this.canvas.height = size.height;
    }

    protected onMouseDown(event: MouseEvent): void {
        this.movement.processing = true;
        this.movement.x = event.clientX;
        this.movement.y = event.clientY;
    }

    protected onMouseMove(event: MouseEvent): void {
        if (!this.movement.processing) {
            return;
        }
        this.position.x -=
            (this.movement.x - event.clientX) / this.position.zoom;
        this.position.y -=
            (this.movement.y - event.clientY) / this.position.zoom;
        this.movement.x = event.clientX;
        this.movement.y = event.clientY;
        this.render();
    }

    protected onMouseUp(event: MouseEvent): void {
        this.movement.processing = false;
    }

    protected onWheel(event: WheelEvent): void {
        this.position.zoom += event.deltaY > 0 ? 0.05 : -0.05;
        this.position.zoom =
            this.position.zoom < 0.1 ? 0.1 : this.position.zoom;
        this.position.zoom = this.position.zoom > 2 ? 2 : this.position.zoom;
        this.render();
    }

    public destroy(): void {
        this.parent.removeEventListener("mousedown", this.onMouseDown);
        this.parent.removeEventListener("wheel", this.onWheel);
        window.removeEventListener("mousemove", this.onMouseMove);
        window.removeEventListener("mouseup", this.onMouseUp);
    }

    public init(composition: Types.Composition) {
        this.board.init(composition);
    }

    public render() {
        this.board.render(this.position.x, this.position.y, this.position.zoom);
    }
}
