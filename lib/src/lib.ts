import { Hover } from './hover';
import { Connection } from './connection';
import { ScrollBars, ScrollEvent } from './scrollbars';
import { Subject, Subjects, Subscriber } from './subscriber';

import * as Core from 'core';
import * as Types from './types';
import * as DOM from './dom';

export * from './types';

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
import('core')
    .then((core: typeof Core) => {
        wasm.core = core;
    })
    .catch((err: Error) => {
        console.error(`Fail to core load wasm module: ${err.message}`);
    });

const CLICK_DURATION = 250;

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
    protected connection: Connection;
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
            if (typeof parent === 'string') {
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
                    typeof parent === 'string' ? `selector: ${parent} isn't valid` : ''
                }`,
            );
        }
        this.parent = node;
        this.hover = {
            component: new Hover(`rgba(0,0,0,0.25)`, node),
            port: new Hover(`rgba(0,0,0,0.25)`, node),
        };
        this.connection = new Connection(node);
        this.scroll = new ScrollBars(node);
        this.id = getId();
        this.canvas = document.createElement('canvas');
        this.canvas.setAttribute('id', this.id);
        this.canvas.style.position = 'absolute';
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
        this.parent.addEventListener('mousemove', this.onHover);
        this.parent.addEventListener('mouseleave', this.onHoverOver);
        this.parent.addEventListener('mousedown', this.onMouseDown);
        this.parent.addEventListener('wheel', this.onWheel);
        this.parent.addEventListener('click', this.onClick);
        window.addEventListener('keydown', this.onKeyDown);
        window.addEventListener('keyup', this.onKeyUp);
        this.hover.component.onHide(() => {
            this.subjects.get().onComponentHoverOver.emit();
        });
        this.hover.port.onHide(() => {
            this.subjects.get().onPortHoverOver.emit();
        });
        this.register(this.scroll.scroll.subscribe(this.onScroll));
        this.resize = new ResizeObserver(this.onResize);
        this.resize.observe(this.parent);
    }

    protected onResize(entries: ResizeObserverEntry[]): void {
        this.updateSize();
    }

    protected setSize(): void {
        const size = this.parent.getBoundingClientRect();
        this.size.width = size.width;
        this.size.height = size.height;
        this.canvas.width = size.width;
        this.canvas.height = size.height;
    }

    protected updateSize(): void {
        this.setSize();
        this.board.update_size();
        this.scroll.setZoom(this.position.zoom);
        this.scroll.setSize(this.board.get_size() as [number, number], this.size);
        this.scroll.moveTo(0, 0);
        this.canvas.style.left = `0px`;
        this.canvas.style.top = `0px`;
        this.position.x = 0;
        this.position.y = 0;
        this.render();
    }

    protected onKeyDown(event: KeyboardEvent) {
        if (event.key === 'Alt') {
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
            window.addEventListener('mousemove', this.onMouseMove);
            window.addEventListener('mouseup', this.onMouseUp);
        }, CLICK_DURATION);
    }

    protected onMouseMove(event: MouseEvent): void {
        if (!this.movement.processing) {
            return;
        }
        this.position.x -= (this.movement.x - event.offsetX) / this.position.zoom;
        this.position.y -= (this.movement.y - event.offsetY) / this.position.zoom;
        this.position.x = this.position.x > 0 ? 0 : this.position.x;
        this.position.y = this.position.y > 0 ? 0 : this.position.y;
        const canvas = this.board.get_size();
        this.position.x =
            -this.position.x > canvas[0] - this.size.width / this.position.zoom
                ? -(canvas[0] - this.size.width / this.position.zoom)
                : this.position.x;
        this.position.y =
            -this.position.y > canvas[1] - this.size.height / this.position.zoom
                ? -(canvas[1] - this.size.height / this.position.zoom)
                : this.position.y;
        this.movement.x = event.offsetX;
        this.movement.y = event.offsetY;
        this.scroll.moveTo(
            -this.position.x * this.position.zoom,
            -this.position.y * this.position.zoom,
        );
        this.canvas.style.left = `${-this.position.x * this.position.zoom}px`;
        this.canvas.style.top = `${-this.position.y * this.position.zoom}px`;
        this.render();
    }

    protected onMouseUp(_event: MouseEvent): void {
        this.movement.processing = false;
        this.scroll.locked(false);
        window.removeEventListener('mousemove', this.onMouseMove);
        window.removeEventListener('mouseup', this.onMouseUp);
        clearTimeout(this.movement.clickTimer);
    }

    protected getTargetsOnMouse(event: MouseEvent): Types.ElementCoors[] {
        let x = event.offsetX - this.position.x * this.position.zoom;
        let y = event.offsetY - this.position.y * this.position.zoom;
        if (x < 0 || y < 0) {
            return [];
        }
        this.board.set_view_state(this.position.x, this.position.y, this.position.zoom);
        return this.board
            .who(x, y, 2)
            .filter(
                (element: Types.ElementCoors) => element[0] !== this.data.composition?.toString(),
            );
    }

    protected onScroll(event: ScrollEvent) {
        this.position.x = -event.x / this.position.zoom;
        this.position.y = -event.y / this.position.zoom;
        this.canvas.style.left = `${event.x}px`;
        this.canvas.style.top = `${event.y}px`;
        this.render();
    }

    protected onHover(event: MouseEvent): void {
        if (this.movement.processing) {
            return;
        }
        const targets: Types.ElementCoors[] = this.getTargetsOnMouse(event);
        const component = targets.filter((t) => t[1] === 'Component' || t[1] === 'Composition');
        if (component.length === 1) {
            const id = parseInt(component[0][0], 10);
            if (!this.hover.component.isActive(id)) {
                this.hover.component.show(
                    id,
                    component[0][2][0] + this.scroll.x() + this.position.x * this.position.zoom,
                    component[0][2][1] + this.scroll.y() + this.position.y * this.position.zoom,
                    component[0][2][2] - component[0][2][0],
                    component[0][2][3] - component[0][2][1],
                );
                this.subjects.get().onComponentHover.emit({
                    id,
                    x: event.offsetX,
                    y: event.offsetY,
                });
            }
        } else {
            this.hover.component.hide();
        }
        const port = targets.filter((t) => t[1] === 'Port');
        if (port.length === 1) {
            const id = parseInt(port[0][0], 10);
            if (!this.hover.port.isActive(id)) {
                this.hover.port.show(
                    id,
                    port[0][2][0] + this.scroll.x() + this.position.x * this.position.zoom,
                    port[0][2][1] + this.scroll.y() + this.position.y * this.position.zoom,
                    port[0][2][2] - port[0][2][0],
                    port[0][2][3] - port[0][2][1],
                );
                const groupped = this.data.groupped.find((groupped) => groupped[0] === id);
                const connection = this.getConnectionInfo(id);
                if (connection !== undefined) {
                    const target =
                        connection.inner.port === id ? connection.inner : connection.outter;
                    const coors = this.getCoorsByIds([target.port, target.component]);
                    const port = coors.find((coor) => coor[1] === 'Port');
                    const component = coors.find((coor) => coor[1] === 'Component');
                    if (port !== undefined && component !== undefined) {
                        this.connection.show(
                            {
                                left: port[2][0] + this.scroll.x(),
                                top: port[2][1] + this.scroll.y(),
                                width: port[2][2] - port[2][0],
                                height: port[2][3] - port[2][1],
                            },
                            {
                                left: component[2][0] + this.scroll.x(),
                                top: component[2][1] + this.scroll.y(),
                                width: component[2][2] - component[2][0],
                                height: component[2][3] - component[2][1],
                            },
                        );
                    } else {
                        this.connection.hide();
                    }
                } else {
                    this.connection.hide();
                }
                this.subjects.get().onPortHover.emit({
                    id,
                    contains: groupped === undefined ? [] : groupped[1],
                    x: event.offsetX,
                    y: event.offsetY,
                });
            }
        } else {
            this.hover.port.hide();
            this.connection.hide();
        }
    }

    protected onHoverOver(event: MouseEvent): void {
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
            const targets: Types.ElementCoors[] = this.getTargetsOnMouse(event);
            const back = targets.find((element: Types.ElementCoors) =>
                element[0].startsWith('back::'),
            );
            if (back !== undefined) {
                const target = parseInt(back[0].replace('back::', ''), 10);
                this.data.history.pop();
                this.goToComposition(target);
            } else if (targets.length > 1) {
                console.log(`Cannot detect target too many ids: ${targets.join(', ')}`);
                return;
            } else if (targets.length === 1) {
                const element = targets[0] as Types.ElementCoors;
                const targetId = parseInt(element[0], 10);
                const elementType = element[1];
                if (elementType === 'Port') {
                    this.board.toggle_port(targetId);
                    this.subjects.get().onPortClick.emit(targetId);
                } else if (elementType === 'Component') {
                    this.board.toggle_component(targetId);
                } else if (elementType === 'Composition') {
                    this.data.composition !== undefined &&
                        this.data.history.push(this.data.composition);
                    this.goToComposition(targetId);
                }
            }
        }
    }

    protected zoom(deltaY: number) {
        this.position.zoom += deltaY > 0 ? 0.05 : -0.05;
        this.position.zoom = this.position.zoom < 0.1 ? 0.1 : this.position.zoom;
        this.position.zoom = this.position.zoom > 2 ? 2 : this.position.zoom;
        this.scroll.setZoom(this.position.zoom);
        this.scroll.moveTo(
            -this.position.x * this.position.zoom,
            -this.position.y * this.position.zoom,
        );
        this.canvas.style.left = `${-this.position.x * this.position.zoom}px`;
        this.canvas.style.top = `${-this.position.y * this.position.zoom}px`;
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
        this.data.composition = id;
        this.data.groupped = this.board.get_groupped_ports();
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
        this.parent.removeEventListener('mousedown', this.onMouseDown);
        this.parent.removeEventListener('wheel', this.onWheel);
        this.parent.removeEventListener('mousemove', this.onHover);
        this.parent.removeEventListener('mouseleave', this.onHoverOver);
        window.removeEventListener('mousemove', this.onMouseMove);
        window.removeEventListener('mouseup', this.onMouseUp);
        window.removeEventListener('keydown', this.onKeyDown);
        window.removeEventListener('keyup', this.onKeyUp);
        this.hover.component.destroy();
        this.hover.port.destroy();
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

    public render() {
        this.board.set_view_state(this.position.x, this.position.y, this.position.zoom);
        this.board.render();
    }

    public getGrouppedPorts(): [number, number[]][] {
        return this.board.get_groupped_ports() as [number, number[]][];
    }

    public getCoorsByIds(ids: number[]): Types.ElementCoors[] {
        this.board.set_view_state(this.position.x, this.position.y, this.position.zoom);
        return this.board.get_coors_by_ids(Uint32Array.from(ids));
    }

    public getConnectionInfo(port: number):
        | {
              outter: { port: number; contains: number[]; component: number };
              inner: { port: number; contains: number[]; component: number };
          }
        | undefined {
        const info: [[number, number[], number], [number, number[], number]] | undefined | string =
            this.board.get_connection_info(port);
        if (typeof info === 'string') {
            console.error(info);
            return undefined;
        }
        if (info === undefined || info === null) {
            return undefined;
        }
        return {
            outter: { port: info[0][0], contains: info[0][1], component: info[0][2] },
            inner: { port: info[1][0], contains: info[1][1], component: info[1][2] },
        };
    }
}
