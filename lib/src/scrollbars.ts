import { Subject } from "./subscriber";

export interface ScrollEvent {
    x: number;
    y: number;
}
export class ScrollBars {
    protected readonly filler: HTMLDivElement;
    protected readonly canvas: {
        width: number;
        height: number;
    } = {
        width: 0,
        height: 0,
    };
    protected readonly container: {
        width: number;
        height: number;
    } = {
        width: 0,
        height: 0,
    };
    protected readonly current: {
        x: number;
        y: number;
    } = {
        x: 0,
        y: 0,
    };
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

    public setSize(
        canvas: [number, number],
        container: { width: number; height: number }
    ) {
        this.canvas.width = canvas[0];
        this.canvas.height = canvas[1];
        this.container.width = container.width;
        this.container.height = container.height;
        this.update();
    }

    public x(): number {
        return this.current.x;
    }

    public y(): number {
        return this.current.y;
    }

    onScroll(_event: Event) {
        this.current.x =
            this.parent.scrollLeft + this.container.width >= this.canvas.width
                ? this.canvas.width - this.container.width
                : this.parent.scrollLeft;
        this.current.y =
            this.parent.scrollTop + this.container.height >= this.canvas.height
                ? this.canvas.height - this.container.height
                : this.parent.scrollTop;
        this.scroll.emit({
            x: this.current.x,
            y: this.current.y,
        });
    }

    update() {
        this.filler.style.width = `${this.canvas.width}px`;
        this.filler.style.height = `${this.canvas.height}px`;
    }
}
