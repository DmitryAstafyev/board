const HIDE_IN_MS: number = 2000;

export class ZoomLabel {
    protected readonly label: HTMLSpanElement;
    protected timer: number = -1;

    constructor(protected readonly parent: HTMLElement) {
        this.label = document.createElement("span");
        this.label.style.top = "50%";
        this.label.style.left = "50%";
        this.label.style.marginLeft = "-30px";
        this.label.style.marginTop = "-11px";
        this.label.style.display = "none";
        this.label.style.position = "fixed";
        this.label.style.width = "60px";
        this.label.style.textAlign = "center";
        this.label.style.fontSize = "18px";
        this.label.style.lineHeight = "18px";
        this.label.style.fontWeight = "bold";
        this.label.style.userSelect = "none";
        this.label.style.background = "rgba(255,255,255,0.4)";
        this.label.style.padding = "8px 4px 0px 4px";
        this.label.style.borderRadius = "3px";
        parent.appendChild(this.label);
    }

    public destroy() {
        this.label.parentNode?.removeChild(this.label);
    }

    public show(factor: number) {
        this.label.textContent = `${(factor * 100).toFixed(0)}%`;
        this.label.style.display = "block";
        clearTimeout(this.timer);
        this.timer = setTimeout(() => {
            // this.label.style.display = "none";
        }, HIDE_IN_MS) as unknown as number;
    }
}
