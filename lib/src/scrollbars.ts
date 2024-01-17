import { Subject } from "./subscriber";

import * as DOM from "./dom";

export interface ScrollEvent {
    x: number;
    y: number;
}
export class ScrollBars {
    protected readonly filler: HTMLDivElement;
    protected readonly canvas: {
        width: number;
        height: number;
        zWidth: number;
        zHeight: number;
    } = {
        width: 0,
        height: 0,
        zWidth: 0,
        zHeight: 0,
    };
    protected readonly container: {
        width: number;
        height: number;
        zoom: number;
    } = {
        width: 0,
        height: 0,
        zoom: 0,
    };
    protected readonly current: {
        x: number;
        y: number;
        locked: boolean;
    } = {
        x: 0,
        y: 0,
        locked: false,
    };

    protected onScroll(event: Event) {
        if (this.current.locked) {
            DOM.stop(event);
        } else {
            this.calc();
        }
    }

    protected update() {
        this.filler.style.width = `${this.canvas.zWidth}px`;
        this.filler.style.height = `${this.canvas.zHeight}px`;
    }

    public scroll: Subject<ScrollEvent> = new Subject();

    constructor(protected readonly parent: HTMLElement) {
        this.filler = document.createElement("div");
        this.filler.style.top = "0px";
        this.filler.style.left = "0px";
        this.filler.style.opacity = "0.001";
        this.filler.style.width = "0px";
        this.filler.style.height = "0px";
        this.filler.style.pointerEvents = "none";
        this.filler.style.userSelect = "none";
        this.filler.style.position = "absolute";
        parent.appendChild(this.filler);
        this.onScroll = this.onScroll.bind(this);
        parent.addEventListener("scroll", this.onScroll);
    }

    public destroy() {
        this.scroll.destroy();
        this.parent.removeEventListener("scroll", this.onScroll);
        this.filler.parentNode?.removeChild(this.filler);
    }

    public calc() {
        const x = this.current.x;
        const y = this.current.y;
        this.current.x =
            this.parent.scrollLeft + this.container.width >= this.canvas.zWidth
                ? this.canvas.zWidth - this.container.width
                : this.parent.scrollLeft;
        this.current.y =
            this.parent.scrollTop + this.container.height >= this.canvas.zHeight
                ? this.canvas.zHeight - this.container.height
                : this.parent.scrollTop;
        if (x !== this.current.x || y !== this.current.y) {
            this.scroll.emit({
                x: this.current.x,
                y: this.current.y,
            });
        }
    }

    public setSize(
        canvas: [number, number],
        container: { width: number; height: number }
    ) {
        this.canvas.width = canvas[0];
        this.canvas.height = canvas[1];
        this.container.width = container.width;
        this.container.height = container.height;
        this.canvas.zWidth = this.canvas.width * this.container.zoom;
        this.canvas.zHeight = this.canvas.height * this.container.zoom;
        this.update();
    }

    public setZoom(zoom: number) {
        this.container.zoom = zoom;
        this.canvas.zWidth = this.canvas.width * this.container.zoom;
        this.canvas.zHeight = this.canvas.height * this.container.zoom;
        this.update();
    }

    public moveTo(x: number, y: number) {
        this.parent.scrollLeft = x;
        this.parent.scrollTop = y;
        this.current.x = x;
        this.current.y = y;
    }

    public x(): number {
        return this.current.x;
    }

    public y(): number {
        return this.current.y;
    }

    public locked(locked: boolean) {
        this.current.locked = locked;
    }
}
