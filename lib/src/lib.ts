import { Hover } from "./hover";

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

const CLICK_DURATION = 250;

export class Board {
    protected readonly board: Core.Board;
    protected readonly canvas: HTMLCanvasElement;
    protected readonly parent: HTMLElement;
    protected readonly id: string;
    protected readonly hover: Hover = new Hover();
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
        clickTimer: any;
        dropClick: boolean;
    } = {
        x: 0,
        y: 0,
        processing: false,
        clickTimer: -1,
        dropClick: false,
    };
    protected data: {
        composition: number | undefined;
        root: Types.Composition | undefined;
        history: number[];
    } = {
        composition: undefined,
        root: undefined,
        history: [],
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
        this.onHover = this.onHover.bind(this);
        this.onHoverOver = this.onHoverOver.bind(this);
        this.onMouseUp = this.onMouseUp.bind(this);
        this.onWheel = this.onWheel.bind(this);
        this.onClick = this.onClick.bind(this);
        this.parent.addEventListener("mousemove", this.onHover);
        this.parent.addEventListener("mouseleave", this.onHoverOver);
        this.parent.addEventListener("mousedown", this.onMouseDown);
        this.parent.addEventListener("wheel", this.onWheel);
        this.parent.addEventListener("click", this.onClick);
    }

    protected setSize(): void {
        const size = this.parent.getBoundingClientRect();
        this.size.width = size.width;
        this.size.height = size.height;
        this.canvas.width = size.width;
        this.canvas.height = size.height;
    }

    protected onMouseDown(event: MouseEvent): void {
        this.movement.x = event.clientX;
        this.movement.y = event.clientY;
        this.movement.dropClick = false;
        this.movement.clickTimer = setTimeout(() => {
            this.hover.hide();
            this.movement.processing = true;
            this.movement.dropClick = true;
            window.addEventListener("mousemove", this.onMouseMove);
            window.addEventListener("mouseup", this.onMouseUp);
        }, CLICK_DURATION);
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
        window.removeEventListener("mousemove", this.onMouseMove);
        window.removeEventListener("mouseup", this.onMouseUp);
        clearTimeout(this.movement.clickTimer);
    }

    protected onHover(event: MouseEvent): void {
        if (this.movement.processing) {
            return;
        }
        if (
            event.clientX - this.position.x < 0 ||
            event.clientY - this.position.y < 0
        ) {
            return;
        }
        const targets = this.board
            .who(
                this.position.x,
                this.position.y,
                event.clientX - this.position.x,
                event.clientY - this.position.y,
                2,
                this.position.zoom
            )
            .filter(
                (element: Types.ElementCoors) =>
                    element[0] !== this.data.composition?.toString()
            );

        if (targets.length === 1) {
            this.hover.show(
                targets[0][1][0] + this.position.x,
                targets[0][1][1] + this.position.y,
                targets[0][1][2] - targets[0][1][0],
                targets[0][1][3] - targets[0][1][1]
            );
            console.log(`hovering: ${targets[0]}`);
        } else {
            this.hover.hide();
        }
    }

    protected onHoverOver(event: MouseEvent): void {}

    protected onWheel(event: WheelEvent): void {
        this.position.zoom += event.deltaY > 0 ? 0.05 : -0.05;
        this.position.zoom =
            this.position.zoom < 0.1 ? 0.1 : this.position.zoom;
        this.position.zoom = this.position.zoom > 2 ? 2 : this.position.zoom;
        this.render();
    }

    protected onClick(event: MouseEvent): void {
        clearTimeout(this.movement.clickTimer);
        if (this.movement.processing || this.movement.dropClick) {
            return;
        }
        if (
            event.clientX - this.position.x < 0 ||
            event.clientY - this.position.y < 0
        ) {
            return;
        }
        if (event.button == 0) {
            const targets = this.board
                .who(
                    this.position.x,
                    this.position.y,
                    event.clientX - this.position.x,
                    event.clientY - this.position.y,
                    2,
                    this.position.zoom
                )
                .filter(
                    (element: Types.ElementCoors) =>
                        element[0] !== this.data.composition?.toString()
                );
            const back = targets.find((element: Types.ElementCoors) =>
                element[0].startsWith("back::")
            );
            if (back !== undefined) {
                const target = parseInt(back[0].replace("back::", ""), 10);
                this.data.history.pop();
                this.goToComposition(target);
            } else if (targets.length > 1) {
                console.log(
                    `Cannot detect target too many ids: ${targets.join(", ")}`
                );
                return;
            } else if (targets.length === 1) {
                this.data.composition !== undefined &&
                    this.data.history.push(this.data.composition);
                this.goToComposition(parseInt(targets[0], 10));
            }
        }
    }

    protected goToComposition(id: number) {
        if (this.data.root === undefined) {
            return;
        }
        const composition = Types.getComposition(this.data.root, id);
        if (composition === undefined) {
            console.log(`Fail to find composition ID: ${id}`);
            return;
        }
        this.board.init(composition, Uint32Array.from([]));
        this.data.composition = id;
        this.render();
    }

    public destroy(): void {
        this.parent.removeEventListener("mousedown", this.onMouseDown);
        this.parent.removeEventListener("wheel", this.onWheel);
        this.parent.removeEventListener("mousemove", this.onHover);
        this.parent.removeEventListener("mouseleave", this.onHoverOver);
        window.removeEventListener("mousemove", this.onMouseMove);
        window.removeEventListener("mouseup", this.onMouseUp);
        this.hover.destroy();
    }

    public bind(composition: Types.Composition, expanded: number[]) {
        this.board.init(composition, Uint32Array.from(expanded));
        this.data.composition = composition.sig.id;
        this.data.root = composition;
    }

    public render() {
        this.board.render(this.position.x, this.position.y, this.position.zoom);
    }
}
