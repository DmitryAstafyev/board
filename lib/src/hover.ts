export type ActionOnHide = () => void;

export class Hover {
    protected node: HTMLDivElement;
    protected top: number = 0;
    protected left: number = 0;
    protected width: number = 0;
    protected height: number = 0;
    protected visible: boolean = false;
    protected id: number | undefined;
    protected _onHideAction: ActionOnHide | undefined;

    constructor(color: string) {
        this.node = document.createElement("div") as HTMLDivElement;
        document.body.appendChild(this.node);
        this.node.className = "hover";
        this.node.style.display = "none";
        this.node.style.position = "absolute";
        this.node.style.background = color;
        this.node.style.pointerEvents = "none";
        this.node.style.userSelect = "none";
        this.node.style.zIndex = `10`;
    }

    public destroy() {
        this.node.parentNode?.removeChild(this.node);
    }

    public isActive(id: number): boolean {
        return this.id === id;
    }

    public onHide(action: ActionOnHide) {
        this._onHideAction = action;
    }

    public show(
        id: number,
        left: number,
        top: number,
        width: number,
        height: number
    ): void {
        this.id = id;
        this.node.style.top = `${top}px`;
        this.node.style.left = `${left}px`;
        this.node.style.width = `${width}px`;
        this.node.style.height = `${height}px`;
        this.node.style.display = "block";
    }

    public hide(): void {
        if (this.id === undefined) {
            return;
        }
        this.id = undefined;
        this.node.style.display = "none";
        this._onHideAction !== undefined && this._onHideAction();
    }
}
