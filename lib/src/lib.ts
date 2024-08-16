import { Hover } from "./hover";
import { ScrollBars, ScrollEvent } from "./scrollbars";
import { Subject, Subjects, Subscriber } from "./subscriber";
import { Position, IPosition } from "./position";
import { ZoomLabel } from "./zoomlabel";

import * as Core from "core";
import * as Types from "./types";
import * as DOM from "./dom";

export * from "./types";

export { DEVICE_PIXEL_RATIO } from "./dom";

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

export interface ConnectionSide {
    port: number;
    contains: number[];
    component: number;
}
export interface ConnectionInfo {
    outter: ConnectionSide;
    inner: ConnectionSide;
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

export interface MatchesEvent {
    total: number;
    current: number;
    id: number | undefined;
}
export interface ILocation {
    id: number;
    sig: Types.Signature;
}
export interface SelectionEvent {
    components: number[];
    ports: number[];
    connections: ConnectionInfo[];
}
export interface Match {
    id: number;
    holder: number | undefined;
}

export class Board extends Subscriber {
    protected readonly board: Core.Board;
    protected readonly canvas: HTMLCanvasElement;
    protected readonly parent: HTMLElement;
    protected readonly scroll: ScrollBars;
    protected readonly zoomLabel: ZoomLabel;
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
        ratio: DOM.DEVICE_PIXEL_RATIO,
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
    protected state: {
        ctrl: boolean;
        focused: boolean;
    } = {
        ctrl: false,
        focused: false,
    };
    protected data: {
        composition: number | undefined;
        grouped: [number, number[]][];
        root: Types.Composition | undefined;
        history: ILocation[];
    } = {
        composition: undefined,
        grouped: [],
        root: undefined,
        history: [],
    };
    protected _matches: {
        ids: number[];
        extended: Match[];
        filter: string | undefined;
        currentIndex: number;
        currentId: number | undefined;
    } = {
        ids: [],
        extended: [],
        filter: undefined,
        currentIndex: -1,
        currentId: undefined,
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
        this.parent.setAttribute("tabindex", "0");
        this.zoomLabel = new ZoomLabel(node);
        this.setSize();
        this.board = new wasm.core.Board(
            options,
            // [components_id[], ports_id[]]
            (event: [number[], number[]]) => {
                setTimeout(() => {
                    const components = event[0];
                    const ports = event[1];
                    let connections: ConnectionInfo[] = [];
                    components.forEach((id) => {
                        const data = this.getConnectionsByComponent(id);
                        data !== undefined &&
                            (connections = connections.concat(
                                connections,
                                data
                            ));
                    });
                    ports.forEach((id) => {
                        const data = this.getConnections(id);
                        data !== undefined &&
                            (connections = connections.concat(
                                connections,
                                data
                            ));
                    });
                    this.subjects.get().onSelectionChange.emit({
                        components,
                        ports,
                        connections,
                    });
                }, 0);
            }
        );
        this.board.attach(this.id);
        this.onMouseDown = this.onMouseDown.bind(this);
        this.onMouseMove = this.onMouseMove.bind(this);
        this.onHover = this.onHover.bind(this);
        this.onHoverOver = this.onHoverOver.bind(this);
        this.onMouseUp = this.onMouseUp.bind(this);
        this.onWheel = this.onWheel.bind(this);
        this.onClick = this.onClick.bind(this);
        this.onDblClick = this.onDblClick.bind(this);
        this.onKeyDown = this.onKeyDown.bind(this);
        this.onKeyUp = this.onKeyUp.bind(this);
        this.onScroll = this.onScroll.bind(this);
        this.onResize = this.onResize.bind(this);
        this.onFocus = this.onFocus.bind(this);
        this.onBlur = this.onBlur.bind(this);
        this.parent.addEventListener("focus", this.onFocus);
        this.parent.addEventListener("blur", this.onBlur);
        this.parent.addEventListener("mousemove", this.onHover);
        this.parent.addEventListener("mouseleave", this.onHoverOver);
        this.parent.addEventListener("mousedown", this.onMouseDown);
        document.addEventListener("wheel", this.onWheel, { passive: false });
        this.parent.addEventListener("click", this.onClick);
        this.parent.addEventListener("dblclick", this.onDblClick);
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

    protected onFocus() {
        this.state.focused = true;
    }

    protected onBlur() {
        this.state.focused = false;
    }

    protected onKeyDown(event: KeyboardEvent) {
        if (event.key === "Control" || event.key === "Shift") {
            this.state.ctrl = true;
            this.scroll.locked(true);
        }
    }

    protected onKeyUp(_event: KeyboardEvent) {
        this.state.ctrl = false;
        this.scroll.locked(false);
    }

    protected onMouseDown(event: MouseEvent): void {
        if (event.target == this.parent) {
            // Click on scroll bars
            return;
        }
        this.movement.x = event.screenX;
        this.movement.y = event.screenY;
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

    protected validate(): {
        x: () => void;
        y: () => void;
    } {
        const canvas = this.board.get_size();
        return {
            x: () => {
                this.position.x = this.position.x > 0 ? 0 : this.position.x;
                this.position.x =
                    -this.position.x >
                    canvas[0] - this.size.width / this.position.zoom
                        ? -(canvas[0] - this.size.width / this.position.zoom)
                        : this.position.x;
                this.position.x = this.position.x > 0 ? 0 : this.position.x;
            },
            y: () => {
                this.position.y = this.position.y > 0 ? 0 : this.position.y;
                this.position.y =
                    -this.position.y >
                    canvas[1] - this.size.height / this.position.zoom
                        ? -(canvas[1] - this.size.height / this.position.zoom)
                        : this.position.y;
                this.position.y = this.position.y > 0 ? 0 : this.position.y;
            },
        };
    }
    protected validatePositionX() {
        const canvas = this.board.get_size();
        this.position.x = this.position.x > 0 ? 0 : this.position.x;
        this.position.x =
            -this.position.x > canvas[0] - this.size.width / this.position.zoom
                ? -(canvas[0] - this.size.width / this.position.zoom)
                : this.position.x;
        this.position.x = this.position.x > 0 ? 0 : this.position.x;
    }
    protected onMouseMove(event: MouseEvent): void {
        if (!this.movement.processing) {
            return;
        }
        if (!this.position.xLocked) {
            this.position.x -=
                (this.movement.x - event.screenX) / this.position.zoom;
            this.validate().x();
        }
        if (!this.position.yLocked) {
            this.position.y -=
                (this.movement.y - event.screenY) / this.position.zoom;
            this.validate().y();
        }
        this.movement.x = event.screenX;
        this.movement.y = event.screenY;
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
        if (y < 0) {
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
        event.horizontal && (this.position.x = -event.x / this.position.zoom);
        event.vertical && (this.position.y = -event.y / this.position.zoom);
        this.render();
    }

    protected onHover(event: MouseEvent): void {
        if (this.movement.processing) {
            return;
        }
        const targets = this.getTargetsOnMouse(event);
        if (
            (targets.components.length === 1 ||
                targets.compositions.length === 1) &&
            targets.ports.length === 0
        ) {
            this.hover.port.hide();
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
            this.hover.component.hide();
            const id = parseInt(targets.ports[0][0], 10);
            if (!this.hover.port.isActive(id)) {
                this.hover.port.show(id);
                const grouped = this.data.grouped.find(
                    (grouped) => grouped[0] === id
                );
                this.subjects.get().onPortHover.emit({
                    id,
                    contains: grouped === undefined ? [] : grouped[1],
                    x: event.offsetX,
                    y: event.offsetY,
                });
            }
        } else {
            this.hover.port.hide();
            this.hover.component.hide();
        }
    }

    protected onHoverOver(_event: MouseEvent): void {
        this.hover.component.hide();
        this.hover.port.hide();
    }

    protected onWheel(event: WheelEvent): void {
        if (!this.state.ctrl || !this.state.focused) {
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
                this.goToComposition(targets.back);
            } else if (targets.ports.length === 1) {
                const targetId = parseInt(targets.ports[0][0], 10);
                this.board.toggle_port(targetId, !this.state.ctrl);
                this.subjects.get().onPortClick.emit(targetId);
            } else if (targets.components.length === 1) {
                const targetId = parseInt(targets.components[0][0], 10);
                this.board.toggle_component(targetId, !this.state.ctrl);
                this.subjects.get().onComponentClick.emit(targetId);
            } else if (targets.compositions.length === 1) {
                const targetId = parseInt(targets.compositions[0][0], 10);
                this.board.toggle_component(targetId, !this.state.ctrl);
            }
        }
    }

    protected onDblClick(event: MouseEvent): void {
        this.hover.component.hide();
        this.hover.port.hide();
        clearTimeout(this.movement.clickTimer);
        if (this.movement.processing || this.movement.dropClick) {
            return;
        }
        if (event.button == 0) {
            const targets = this.getTargetsOnMouse(event);
            if (targets.compositions.length === 1) {
                const targetId = parseInt(targets.compositions[0][0], 10);
                this.goToComposition(targetId);
            } else if (
                targets.back === undefined &&
                targets.ports.length === 0 &&
                targets.components.length === 0
            ) {
                this.board.unselect_all();
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
        this.scroll.calc();
        this.zoomLabel.show(this.position.zoom);
        this.updateSize();
    }

    public readonly subjects: Subjects<{
        onComponentHover: Subject<HoverMouseEvent>;
        onComponentClick: Subject<number>;
        onPortHover: Subject<PortHoverEvent>;
        onComponentHoverOver: Subject<void>;
        onPortHoverOver: Subject<void>;
        onPortClick: Subject<number>;
        onSelectionChange: Subject<SelectionEvent>;
        onLocationChange: Subject<ILocation[]>;
        bound: Subject<void>;
        onMatches: Subject<MatchesEvent | undefined>;
    }> = new Subjects({
        onComponentHover: new Subject<HoverMouseEvent>(),
        onComponentClick: new Subject<number>(),
        onComponentHoverOver: new Subject<void>(),
        onPortHover: new Subject<PortHoverEvent>(),
        onPortHoverOver: new Subject<void>(),
        onPortClick: new Subject<number>(),
        onSelectionChange: new Subject<SelectionEvent>(),
        onLocationChange: new Subject<ILocation[]>(),
        bound: new Subject<void>(),
        onMatches: new Subject<MatchesEvent | undefined>(),
    });

    public destroy(): void {
        this.resize.unobserve(this.parent);
        this.parent.removeEventListener("mousedown", this.onMouseDown);
        document.removeEventListener("wheel", this.onWheel);
        this.parent.removeEventListener("mousemove", this.onHover);
        this.parent.removeEventListener("mouseleave", this.onHoverOver);
        window.removeEventListener("mousemove", this.onMouseMove);
        window.removeEventListener("mouseup", this.onMouseUp);
        window.removeEventListener("keydown", this.onKeyDown);
        window.removeEventListener("keyup", this.onKeyUp);
        this.subjects.destroy();
        this.unsubscribe();
    }

    public bind(composition: Types.Composition) {
        this.board.bind(composition);
        this.updateSize();
        this.data.composition = composition.sig.id;
        this.data.root = composition;
        this.data.history = [{ id: composition.sig.id, sig: composition.sig }];
        this.data.grouped = this.getGroupedPorts();
        this.subjects.get().bound.emit();
        this.subjects
            .get()
            .onLocationChange.emit([
                { id: this.data.root.sig.id, sig: this.data.root.sig },
            ]);
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

    public goToComposition(id: number) {
        if (this.data.root === undefined) {
            return;
        }
        this.matches().drop();
        const composition = Types.getComposition(this.data.root, id);
        if (composition === undefined) {
            console.log(`Fail to find composition ID: ${id}`);
            return;
        }
        this.board.unselect_all();
        this.board.bind(composition);
        this.data.composition !== undefined &&
            this.history.set(this.data.composition, this.position.clone());
        const pos = this.data.history.findIndex((el) => el.id === id);
        pos !== -1 && this.data.history.splice(pos, this.data.history.length);
        this.data.composition !== undefined &&
            this.data.history.push({
                id,
                sig: composition.sig,
            });
        this.data.composition = id;
        this.data.grouped = this.board.get_grouped_ports();
        const recent = this.history.get(id);
        if (recent !== undefined) {
            this.position = Position.from(recent);
        } else {
            this.position.dropCoors();
        }
        this.updateSize();
        this.subjects.get().bound.emit();
        this.subjects.get().onLocationChange.emit(this.data.history);
    }

    public toPrevComposition() {
        if (this.data.history.length <= 1) {
            return;
        }
        this.goToComposition(
            this.data.history[this.data.history.length - 2].id
        );
    }

    public filter(): {
        set(filter: string | undefined): void;
        get(): number[];
    } {
        return {
            set: (filter: string | undefined): void => {
                this.board.set_filter(filter);
                this.matches().update();
            },
            get: (): number[] => {
                return this.board.get_filtered();
            },
        };
    }

    public highlight(): {
        set(ids: number[]): void;
        get(): number[];
    } {
        return {
            set: (ids: number[]): void => {
                this.board.set_highlighted(Uint32Array.from(ids));
            },
            get: (): number[] => {
                return Array.from(this.board.get_highlighted());
            },
        };
    }

    public matches(): {
        set(filter: string | undefined): void;
        get(): number[];
        extended(): Match[];
        next(): number | undefined;
        prev(): number | undefined;
        drop(): void;
        update(): void;
    } {
        return {
            set: (filter: string | undefined): void => {
                this.highlight().set([]);
                this._matches.filter =
                    filter === undefined
                        ? undefined
                        : filter.trim() === ""
                        ? undefined
                        : filter.trim();
                this.board.set_matches(this._matches.filter);
                this._matches.currentIndex = -1;
                this._matches.currentId = undefined;
                if (this._matches.filter === undefined) {
                    this._matches.ids = [];
                } else {
                    this._matches.ids = this.matches().get();
                }
                this.matches().next();
            },
            get: (): number[] => {
                return this.board.get_matches();
            },
            extended: (): Match[] => {
                return (
                    this.board.get_extended_matches() as [
                        number,
                        number | null | undefined
                    ][]
                ).map((match: [number, number | null | undefined]) => {
                    return {
                        id: match[0],
                        holder:
                            typeof match[1] === "number" ? match[1] : match[0],
                    };
                });
            },
            next: (): number | undefined => {
                if (this._matches.ids.length === 0) {
                    this.subjects.get().onMatches.emit(undefined);
                    this.render();
                    return undefined;
                }
                this._matches.currentIndex += 1;
                this._matches.currentIndex =
                    this._matches.currentIndex < 0
                        ? 0
                        : this._matches.currentIndex >
                          this._matches.ids.length - 1
                        ? 0
                        : this._matches.currentIndex;
                this._matches.currentId =
                    this._matches.currentIndex === -1
                        ? undefined
                        : this._matches.ids[this._matches.currentIndex];
                this._matches.currentId !== undefined &&
                    this.alignTo(this._matches.currentId);
                this.subjects.get().onMatches.emit({
                    total: this._matches.ids.length,
                    current: this._matches.currentIndex,
                    id: this._matches.currentId,
                });
                return this._matches.currentId;
            },
            prev: (): number | undefined => {
                if (this._matches.ids.length === 0) {
                    this.subjects.get().onMatches.emit(undefined);
                    this.render();
                    return undefined;
                }
                this._matches.currentIndex -= 1;
                this._matches.currentIndex =
                    this._matches.currentIndex < 0
                        ? this._matches.ids.length - 1
                        : this._matches.currentIndex >
                          this._matches.ids.length - 1
                        ? this._matches.ids.length - 1
                        : this._matches.currentIndex;
                this._matches.currentId =
                    this._matches.currentIndex === -1
                        ? undefined
                        : this._matches.ids[this._matches.currentIndex];
                this._matches.currentId !== undefined &&
                    this.alignTo(this._matches.currentId);
                this.subjects.get().onMatches.emit({
                    total: this._matches.ids.length,
                    current: this._matches.currentIndex,
                    id: this._matches.currentId,
                });
                return this._matches.currentId;
            },
            drop: (): void => {
                this.board.set_matches(undefined);
                this._matches.currentIndex = -1;
                this._matches.currentId = undefined;
                this._matches.filter = undefined;
                this._matches.ids = [];
                this.subjects.get().onMatches.emit(undefined);
                this.highlight().set([]);
            },
            update: (): void => {
                if (
                    this._matches.filter === undefined ||
                    this._matches.currentId === undefined
                ) {
                    return;
                }
                this._matches.ids = this.matches().get();
                if (!this._matches.ids.includes(this._matches.currentId)) {
                    this._matches.currentIndex -= 1;
                    this._matches.currentIndex =
                        this._matches.ids.length > 0
                            ? 0
                            : this._matches.currentIndex;
                    this._matches.currentId =
                        this._matches.currentIndex === -1
                            ? undefined
                            : this._matches.ids[this._matches.currentIndex];
                }
                this.subjects.get().onMatches.emit({
                    total: this._matches.ids.length,
                    current: this._matches.currentIndex,
                    id: this._matches.currentId,
                });
            },
        };
    }
    public getMatches(): number[] {
        return this.board.get_matches();
    }

    public getGroupedPorts(): [number, number[]][] {
        return this.board.get_grouped_ports() as [number, number[]][];
    }

    public getPort(id: number): Types.Port | undefined {
        const port = this.board.get_port(id);
        return port === null ? undefined : port;
    }

    public getPortsProps(): Types.EntityProps {
        return this.board.get_ports_props();
    }

    public getCompsProps(): Types.EntityProps {
        return this.board.get_comps_props();
    }

    public getCoorsByIds(ids: number[]): Types.ElementCoors[] {
        this.board.set_view_state(
            this.position.x,
            this.position.y,
            this.position.zoom
        );
        return this.board.get_coors_by_ids(Uint32Array.from(ids));
    }

    public alignTo(id: number) {
        this.highlight().set([]);
        const coors = this.getCoorsByIds([id]);
        if (coors.length === 0) {
            return;
        }
        const coor = coors[0][2] as unknown as [number, number, number, number];
        const used = this.board.get_size() as [number, number];
        if (used[0] > this.size.width) {
            const left = coor[0] + (coor[2] - coor[0]) / 2;
            const x_middle = this.size.width / 2;
            if (left > x_middle) {
                this.position.x -= (left - x_middle) / this.position.zoom;
            } else {
                this.position.x += (x_middle - left) / this.position.zoom;
            }
            this.validate().x();
        }
        if (used[1] > this.size.height) {
            const top = coor[1]; // do not consider height, because if it's component or composition, it might be too high
            const y_middle = this.size.height / 2;
            if (top > y_middle) {
                this.position.y -= (top - y_middle) / this.position.zoom;
            } else {
                this.position.y += (y_middle - top) / this.position.zoom;
            }
            this.validate().y();
        }
        this.scroll.moveTo(
            -this.position.x * this.position.zoom,
            -this.position.y * this.position.zoom
        );
        this.highlight().set([id]);
        this.render();
    }

    public getConnection(port: number): ConnectionInfo | undefined {
        const info:
            | [IncomeConnectionInfo, IncomeConnectionInfo]
            | undefined
            | string = this.board.get_connection(port);
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

    public getConnections(port: number): ConnectionInfo[] | undefined {
        const info:
            | [IncomeConnectionInfo, IncomeConnectionInfo][]
            | undefined
            | string = this.board.get_connections(port);
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

    public getConnectionsByComponent(
        component: number
    ): ConnectionInfo[] | undefined {
        const info:
            | [IncomeConnectionInfo, IncomeConnectionInfo][]
            | undefined
            | string = this.board.get_connections_by_component(component);
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
