import { Hover } from "./hover";
import { ScrollBars, ScrollEvent } from "./scrollbars";
import { Subject, Subjects, Subscriber } from "./subscriber";
import { Position, IPosition } from "./position";

import * as Core from "core";
import * as Types from "./types";
import * as DOM from "./dom";

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
const SCROLLBAR_SIZE = 16;

export interface ConnectionInfo {
    port: number;
    contains: number[];
    component: number;
}

type IncomeConnectionInfo = [number, number[], number];

export interface PortHoverEvent {
    id: number;
    contains: number[];
    x: number;
    y: number;
}

export interface HoverMouseEvent {
    id: number;
    x: number;
    y: number;
}

export class Board extends Subscriber {
    protected readonly board: Core.Board;
    protected readonly canvas: HTMLCanvasElement;
    protected readonly parent: HTMLElement;
    protected readonly scroll: ScrollBars;
    protected readonly id: string;
    protected readonly hover: {
        component: Hover;
        port: Hover;
    };
    protected readonly size: {
        height: number;
        width: number;
        ratio: number;
    } = {
        height: 0,
        width: 0,
        ratio: window !== undefined ? Math.ceil(window.devicePixelRatio) : 1,
    };
    protected position: Position = new Position();
    protected readonly history: Map<number, IPosition> = new Map();
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
    protected keyboard: {
        alt: boolean;
    } = {
        alt: false,
    };
    protected data: {
        composition: number | undefined;
        groupped: [number, number[]][];
        root: Types.Composition | undefined;
        history: number[];
    } = {
        composition: undefined,
        groupped: [],
        root: undefined,
        history: [],
    };
    protected readonly resize: ResizeObserver;

    constructor(parent: string | HTMLElement, options: Types.Options) {
        super();
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
        this.hover = {
            component: new Hover(),
            port: new Hover(),
        };
        this.scroll = new ScrollBars(node);
        this.id = getId();
        this.canvas = document.createElement("canvas");
        this.canvas.setAttribute("id", this.id);
        this.canvas.style.position = "sticky";
        this.canvas.style.top = "0px";
        this.canvas.style.left = "0px";
        this.parent.appendChild(this.canvas);
        this.setSize();
        this.board = new wasm.core.Board(options);
        this.board.attach(this.id);
        this.onMouseDown = this.onMouseDown.bind(this);
        this.onMouseMove = this.onMouseMove.bind(this);
        this.onHover = this.onHover.bind(this);
        this.onHoverOver = this.onHoverOver.bind(this);
        this.onMouseUp = this.onMouseUp.bind(this);
        this.onWheel = this.onWheel.bind(this);
        this.onClick = this.onClick.bind(this);
        this.onKeyDown = this.onKeyDown.bind(this);
        this.onKeyUp = this.onKeyUp.bind(this);
        this.onScroll = this.onScroll.bind(this);
        this.onResize = this.onResize.bind(this);
        this.parent.addEventListener("mousemove", this.onHover);
        this.parent.addEventListener("mouseleave", this.onHoverOver);
        this.parent.addEventListener("mousedown", this.onMouseDown);
        this.parent.addEventListener("wheel", this.onWheel);
        this.parent.addEventListener("click", this.onClick);
        window.addEventListener("keydown", this.onKeyDown);
        window.addEventListener("keyup", this.onKeyUp);
        this.hover.port.onHide((id: number) => {
            this.board.unhighlight_connection_by_port(id);
            this.board.unhover();
            this.board.render();
            this.subjects.get().onPortHoverOver.emit();
        });
        this.hover.port.onShow((id: number) => {
            this.board.highlight_connection_by_port(id);
            this.board.hover(id);
            this.board.render();
        });
        this.hover.component.onHide((_id: number) => {
            this.board.unhover();
            this.board.render();
            this.subjects.get().onComponentHoverOver.emit();
        });
        this.hover.component.onShow((id: number) => {
            this.board.hover(id);
            this.board.render();
        });
        this.register(this.scroll.scroll.subscribe(this.onScroll));
        this.resize = new ResizeObserver(this.onResize);
        this.resize.observe(this.parent);
    }

    protected onResize(_entries: ResizeObserverEntry[]): void {
        try {
            this.updateSize();
        } catch (e) {
            console.error(e);
        }
    }

    protected setSize(): void {
        const size = this.parent.getBoundingClientRect();
        this.size.width = size.width;
        this.size.height = size.height;
        this.canvas.style.width = `${size.width - SCROLLBAR_SIZE}px`;
        this.canvas.style.height = `${size.height - SCROLLBAR_SIZE}px`;
        this.canvas.width = (size.width - SCROLLBAR_SIZE) * this.size.ratio;
        this.canvas.height = (size.height - SCROLLBAR_SIZE) * this.size.ratio;
    }

    protected updateSize(): void {
        this.setSize();
        const used = this.board.get_size() as [number, number];
        this.scroll.setZoom(this.position.zoom);
        this.scroll.setSize(used, this.size);
        this.position.update(used, this.size);
        this.scroll.moveTo(
            -this.position.x * this.position.zoom,
            -this.position.y * this.position.zoom
        );
        this.render();
    }

    protected onKeyDown(event: KeyboardEvent) {
        if (event.key === "Alt") {
            this.keyboard.alt = true;
            this.scroll.locked(true);
        }
    }

    protected onKeyUp(_event: KeyboardEvent) {
        this.keyboard.alt = false;
        this.scroll.locked(false);
    }

    protected onMouseDown(event: MouseEvent): void {
        if (event.target == this.parent) {
            // Click on scroll bars
            return;
        }
        this.movement.x = event.offsetX;
        this.movement.y = event.offsetY;
        this.movement.dropClick = false;
        this.movement.clickTimer = setTimeout(() => {
            this.hover.component.hide();
            this.hover.port.hide();
            this.movement.processing = true;
            this.movement.dropClick = true;
            this.scroll.locked(true);
            window.addEventListener("mousemove", this.onMouseMove);
            window.addEventListener("mouseup", this.onMouseUp);
        }, CLICK_DURATION);
    }

    protected onMouseMove(event: MouseEvent): void {
        if (!this.movement.processing) {
            return;
        }
        const canvas = this.board.get_size();
        if (!this.position.xLocked) {
            this.position.x -=
                (this.movement.x - event.offsetX) / this.position.zoom;
            this.position.x = this.position.x > 0 ? 0 : this.position.x;
            this.position.x =
                -this.position.x >
                canvas[0] - this.size.width / this.position.zoom
                    ? -(canvas[0] - this.size.width / this.position.zoom)
                    : this.position.x;
            this.position.x = this.position.x > 0 ? 0 : this.position.x;
        }
        if (!this.position.yLocked) {
            this.position.y -=
                (this.movement.y - event.offsetY) / this.position.zoom;
            this.position.y = this.position.y > 0 ? 0 : this.position.y;
            this.position.y =
                -this.position.y >
                canvas[1] - this.size.height / this.position.zoom
                    ? -(canvas[1] - this.size.height / this.position.zoom)
                    : this.position.y;
            this.position.y = this.position.y > 0 ? 0 : this.position.y;
        }
        this.movement.x = event.offsetX;
        this.movement.y = event.offsetY;
        this.scroll.moveTo(
            -this.position.x * this.position.zoom,
            -this.position.y * this.position.zoom
        );
        this.render();
    }

    protected onMouseUp(_event: MouseEvent): void {
        this.movement.processing = false;
        this.scroll.locked(false);
        window.removeEventListener("mousemove", this.onMouseMove);
        window.removeEventListener("mouseup", this.onMouseUp);
        clearTimeout(this.movement.clickTimer);
    }

    protected getTargetsOnMouse(event: MouseEvent): {
        ports: Types.ElementCoors[];
        components: Types.ElementCoors[];
        compositions: Types.ElementCoors[];
        back: number | undefined;
    } {
        let x = event.offsetX - this.position.x * this.position.zoom;
        let y = event.offsetY - this.position.y * this.position.zoom;
        if (x < 0 || y < 0) {
            return {
                ports: [],
                components: [],
                compositions: [],
                back: undefined,
            };
        }
        this.board.set_view_state(
            this.position.x,
            this.position.y,
            this.position.zoom
        );
        const targets: Types.ElementCoors[] = this.board
            .who(x, y, 2)
            .filter(
                (element: Types.ElementCoors) =>
                    element[0] !== this.data.composition?.toString()
            );
        const back = targets.find((element: Types.ElementCoors) =>
            element[0].startsWith("back::")
        );
        if (back !== undefined) {
            return {
                ports: [],
                components: [],
                compositions: [],
                back: parseInt(back[0].replace("back::", ""), 10),
            };
        } else {
            return {
                components: targets.filter((t) => t[1] === "Component"),
                ports: targets.filter((t) => t[1] === "Port"),
                compositions: targets.filter((t) => t[1] === "Composition"),
                back: undefined,
            };
        }
    }

    protected onScroll(event: ScrollEvent) {
        this.position.x = -event.x / this.position.zoom;
        this.position.y = -event.y / this.position.zoom;
        this.render();
    }

    protected onHover(event: MouseEvent): void {
        if (this.movement.processing) {
            return;
        }
        const targets = this.getTargetsOnMouse(event);
        this.hover.port.hide();
        this.hover.component.hide();
        if (
            (targets.components.length === 1 ||
                targets.compositions.length === 1) &&
            targets.ports.length === 0
        ) {
            const id = parseInt(
                targets.components.length === 1
                    ? targets.components[0][0]
                    : targets.compositions[0][0],
                10
            );
            if (!this.hover.component.isActive(id)) {
                this.hover.component.show(id);
                this.subjects.get().onComponentHover.emit({
                    id,
                    x: event.offsetX,
                    y: event.offsetY,
                });
            }
        } else if (targets.ports.length === 1) {
            const id = parseInt(targets.ports[0][0], 10);
            if (!this.hover.port.isActive(id)) {
                this.hover.port.show(id);
                const groupped = this.data.groupped.find(
                    (groupped) => groupped[0] === id
                );
                this.subjects.get().onPortHover.emit({
                    id,
                    contains: groupped === undefined ? [] : groupped[1],
                    x: event.offsetX,
                    y: event.offsetY,
                });
            }
        }
    }

    protected onHoverOver(_board_canvas_id_$event: MouseEvent): void {
        this.hover.component.hide();
        this.hover.port.hide();
    }

    protected onWheel(event: WheelEvent): void {
        if (!this.keyboard.alt) {
            return;
        } else {
            DOM.stop(event);
            this.zoom(event.deltaY);
        }
    }

    protected onClick(event: MouseEvent): void {
        this.hover.component.hide();
        this.hover.port.hide();
        clearTimeout(this.movement.clickTimer);
        if (this.movement.processing || this.movement.dropClick) {
            return;
        }
        if (event.button == 0) {
            const targets = this.getTargetsOnMouse(event);
            if (targets.back !== undefined) {
                this.data.history.pop();
                this.goToComposition(targets.back);
            } else if (targets.ports.length === 1) {
                const targetId = parseInt(targets.ports[0][0], 10);
                this.board.toggle_port(targetId);
                this.subjects.get().onPortClick.emit(targetId);
            } else if (targets.components.length === 1) {
                const targetId = parseInt(targets.components[0][0], 10);
                this.board.toggle_component(targetId);
                this.subjects.get().onComponentClick.emit(targetId);
            } else if (targets.compositions.length === 1) {
                const targetId = parseInt(targets.compositions[0][0], 10);
                this.data.composition !== undefined &&
                    this.data.history.push(this.data.composition);
                this.goToComposition(targetId);
            }
        }
    }

    protected zoom(deltaY: number) {
        this.position.zoom += deltaY > 0 ? 0.05 : -0.05;
        this.position.zoom =
            this.position.zoom < 0.1 ? 0.1 : this.position.zoom;
        this.position.zoom = this.position.zoom > 2 ? 2 : this.position.zoom;
        this.scroll.setZoom(this.position.zoom);
        this.scroll.moveTo(
            -this.position.x * this.position.zoom,
            -this.position.y * this.position.zoom
        );
        this.hover.component.hide();
        this.hover.port.hide();
        this.render();
        this.scroll.calc();
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
        this.board.bind(composition, Uint32Array.from([]));
        this.data.composition !== undefined &&
            this.history.set(this.data.composition, this.position.clone());
        this.data.composition = id;
        this.data.groupped = this.board.get_groupped_ports();
        const recent = this.history.get(id);
        if (recent !== undefined) {
            this.position = Position.from(recent);
        } else {
            this.position.dropCoors();
        }
        this.updateSize();
    }

    public readonly subjects: Subjects<{
        onComponentHover: Subject<HoverMouseEvent>;
        onComponentClick: Subject<number>;
        onPortHover: Subject<PortHoverEvent>;
        onComponentHoverOver: Subject<void>;
        onPortHoverOver: Subject<void>;
        onPortClick: Subject<number>;
    }> = new Subjects({
        onComponentHover: new Subject<HoverMouseEvent>(),
        onComponentClick: new Subject<number>(),
        onComponentHoverOver: new Subject<void>(),
        onPortHover: new Subject<PortHoverEvent>(),
        onPortHoverOver: new Subject<void>(),
        onPortClick: new Subject<number>(),
    });

    public destroy(): void {
        this.resize.unobserve(this.parent);
        this.parent.removeEventListener("mousedown", this.onMouseDown);
        this.parent.removeEventListener("wheel", this.onWheel);
        this.parent.removeEventListener("mousemove", this.onHover);
        this.parent.removeEventListener("mouseleave", this.onHoverOver);
        window.removeEventListener("mousemove", this.onMouseMove);
        window.removeEventListener("mouseup", this.onMouseUp);
        window.removeEventListener("keydown", this.onKeyDown);
        window.removeEventListener("keyup", this.onKeyUp);
        this.subjects.destroy();
        this.unsubscribe();
    }

    public bind(composition: Types.Composition, expanded: number[]) {
        this.board.bind(composition, Uint32Array.from(expanded));
        this.updateSize();
        this.data.composition = composition.sig.id;
        this.data.root = composition;
        this.data.groupped = this.getGrouppedPorts();
    }

    public refresh() {
        this.board.recalc();
        this.updateSize();
    }

    public render() {
        this.board.set_view_state(
            this.position.x,
            this.position.y,
            this.position.zoom
        );
        this.board.render();
    }

    public setFilter(filter: string | undefined) {
        this.board.set_filter(filter);
    }

    public getGrouppedPorts(): [number, number[]][] {
        return this.board.get_groupped_ports() as [number, number[]][];
    }

    public getCoorsByIds(ids: number[]): Types.ElementCoors[] {
        this.board.set_view_state(
            this.position.x,
            this.position.y,
            this.position.zoom
        );
        return this.board.get_coors_by_ids(Uint32Array.from(ids));
    }

    public getConnectionInfo(port: number):
        | {
              outter: ConnectionInfo;
              inner: ConnectionInfo;
          }
        | undefined {
        const info:
            | [IncomeConnectionInfo, IncomeConnectionInfo]
            | undefined
            | string = this.board.get_connection_info(port);
        if (typeof info === "string") {
            console.error(info);
            return undefined;
        }
        if (info === undefined || info === null) {
            return undefined;
        }
        return {
            outter: {
                port: info[0][0],
                contains: info[0][1],
                component: info[0][2],
            },
            inner: {
                port: info[1][0],
                contains: info[1][1],
                component: info[1][2],
            },
        };
    }

    public getConnectionsInfoByPort(port: number):
        | {
              outter: ConnectionInfo;
              inner: ConnectionInfo;
          }[]
        | undefined {
        const info:
            | [IncomeConnectionInfo, IncomeConnectionInfo][]
            | undefined
            | string = this.board.get_connections_info_by_port(port);
        if (typeof info === "string") {
            console.error(info);
            return undefined;
        }
        if (info === undefined || info === null) {
            return undefined;
        }
        return info.map((slot) => {
            return {
                outter: {
                    port: slot[0][0],
                    contains: slot[0][1],
                    component: slot[0][2],
                },
                inner: {
                    port: slot[1][0],
                    contains: slot[1][1],
                    component: slot[1][2],
                },
            };
        });
    }

    public getConnectionsInfoByComponent(component: number):
        | {
              outter: ConnectionInfo;
              inner: ConnectionInfo;
          }[]
        | undefined {
        const info:
            | [IncomeConnectionInfo, IncomeConnectionInfo][]
            | undefined
            | string = this.board.get_connections_info_by_component(component);
        if (typeof info === "string") {
            console.error(info);
            return undefined;
        }
        if (info === undefined || info === null) {
            return undefined;
        }
        return info.map((slot) => {
            return {
                outter: {
                    port: slot[0][0],
                    contains: slot[0][1],
                    component: slot[0][2],
                },
                inner: {
                    port: slot[1][0],
                    contains: slot[1][1],
                    component: slot[1][2],
                },
            };
        });
    }

    public offsetX(x: number): number {
        return this.position.x + x;
    }

    public offsetY(y: number): number {
        return this.position.y + y;
    }

    public offset(): { x: number; y: number } {
        return {
            x: this.position.x,
            y: this.position.y,
        };
    }
}
