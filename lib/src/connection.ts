export class Connection {
    protected node: {
        port: HTMLDivElement;
        component: HTMLDivElement;
    };
    protected port: {
        top: number;
        left: number;
        width: number;
        height: number;
    } = {
        top: 0,
        left: 0,
        width: 0,
        height: 0,
    };
    protected component: {
        top: number;
        left: number;
        width: number;
        height: number;
    } = {
        top: 0,
        left: 0,
        width: 0,
        height: 0,
    };
    protected visible: boolean = false;
    protected id: number | undefined;

    constructor(parent: HTMLElement) {
        this.node = {
            port: document.createElement("div") as HTMLDivElement,
            component: document.createElement("div") as HTMLDivElement,
        };
        parent.appendChild(this.node.port);
        parent.appendChild(this.node.component);
        this.node.port.className = "port-highlight";
        this.node.port.style.display = "none";
        this.node.port.style.position = "absolute";
        this.node.port.style.background = "rgba(0,0,0,0.25)";
        this.node.port.style.pointerEvents = "none";
        this.node.port.style.userSelect = "none";
        this.node.port.style.zIndex = `10`;
        this.node.component.className = "component-highlight";
        this.node.component.style.display = "none";
        this.node.component.style.position = "absolute";
        this.node.component.style.background = "rgba(0,0,0, 0.25)";
        this.node.component.style.pointerEvents = "none";
        this.node.component.style.userSelect = "none";
        this.node.component.style.zIndex = `10`;
    }

    public destroy() {
        this.node.port.parentNode?.removeChild(this.node.port);
        this.node.component.parentNode?.removeChild(this.node.component);
    }

    public show(
        port: { left: number; top: number; width: number; height: number },
        component: { left: number; top: number; width: number; height: number }
    ): void {
        this.node.port.style.top = `${port.top}px`;
        this.node.port.style.left = `${port.left}px`;
        this.node.port.style.width = `${port.width}px`;
        this.node.port.style.height = `${port.height}px`;
        this.node.port.style.display = "block";
        this.node.component.style.top = `${component.top}px`;
        this.node.component.style.left = `${component.left}px`;
        this.node.component.style.width = `${component.width}px`;
        this.node.component.style.height = `${component.height}px`;
        this.node.component.style.display = "block";
    }

    public hide(): void {
        this.node.port.style.display = "none";
        this.node.component.style.display = "none";
    }
}
