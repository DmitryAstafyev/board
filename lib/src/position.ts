export interface IPosition {
    x: number;
    y: number;
    zoom: number;
    xLocked: boolean;
    yLocked: boolean;
}

export class Position {
    public x: number = 0;
    public y: number = 0;
    public zoom: number = 1;
    public xLocked: boolean = false;
    public yLocked: boolean = false;

    static from(pos: IPosition): Position {
        return new Position(pos);
    }

    constructor(from: IPosition | undefined = undefined) {
        if (from !== undefined) {
            this.x = from.x;
            this.y = from.y;
            this.zoom = from.zoom;
            this.xLocked = from.xLocked;
            this.yLocked = from.yLocked;
        }
    }
    public dropCoors(): void {
        this.x = 0;
        this.y = 0;
    }

    public update(
        used: [number, number],
        container: { height: number; width: number }
    ): void {
        this.xLocked = used[0] < container.width;
        this.yLocked = used[1] < container.height;
        if (used[0] < container.width) {
            this.x = (container.width - used[0]) / 2;
        }
        if (used[1] < container.height) {
            this.y = (container.height - used[1]) / 2;
        }
    }

    public clone(): IPosition {
        return Object.assign({}, this);
    }
}
