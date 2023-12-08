export class Hover {
    protected node: HTMLDivElement;
    protected top: number = 0;
    protected left: number = 0;
    protected width: number = 0;
    protected height: number = 0;
    protected visible: boolean = false;

    constructor() {
        this.node = document.createElement("div") as HTMLDivElement;
        document.body.appendChild(this.node);
        this.node.className = "hover";
        this.node.style.display = "none";
        this.node.style.position = "absolute";
        this.node.style.opacity = "0.15";
        this.node.style.background = `rgb(0, 0, 0)`;
        this.node.style.pointerEvents = "none";
        this.node.style.userSelect = "none";
        this.node.style.zIndex = `10`;
    }

    public destroy() {
        this.node.parentNode?.removeChild(this.node);
    }

    public show(
        left: number,
        top: number,
        width: number,
        height: number
    ): void {
        this.node.style.top = `${top}px`;
        this.node.style.left = `${left}px`;
        this.node.style.width = `${width}px`;
        this.node.style.height = `${height}px`;
        this.node.style.display = "block";
    }

    public hide(): void {
        this.node.style.display = "none";
    }
}
