export type Action = (id: number) => void;

export class Hover {
    protected node: HTMLDivElement;
    protected top: number = 0;
    protected left: number = 0;
    protected width: number = 0;
    protected height: number = 0;
    protected visible: boolean = false;
    protected id: number = -1;
    protected _onHideAction: Action | undefined;
    protected _onShowAction: Action | undefined;

    constructor(color: string, parent: HTMLElement) {
        this.node = document.createElement("div") as HTMLDivElement;
        parent.appendChild(this.node);
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

    public getId(): number {
        return this.id;
    }

    public isActive(id: number): boolean {
        return this.id === id;
    }

    public onHide(action: Action) {
        this._onHideAction = action;
    }

    public onShow(action: Action) {
        this._onShowAction = action;
    }

    public show(
        id: number,
        left: number,
        top: number,
        width: number,
        height: number
    ): void {
        this.hide();
        this.id = id;
        this.node.style.top = `${top}px`;
        this.node.style.left = `${left}px`;
        this.node.style.width = `${width}px`;
        this.node.style.height = `${height}px`;
        this.node.style.display = "block";
        this._onShowAction !== undefined && this._onShowAction(id);
    }

    public hide(): void {
        if (this.id === -1) {
            return;
        }
        let id = this.id;
        this.id = -1;
        this.node.style.display = "none";
        this._onHideAction !== undefined && this._onHideAction(id);
    }
}
