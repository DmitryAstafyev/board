export type Action = (id: number) => void;

export class Hover {
    protected id: number = -1;
    protected _onHideAction: Action | undefined;
    protected _onShowAction: Action | undefined;

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

    public show(id: number): void {
        if (this.id === id) {
            return;
        }
        this.hide();
        this.id = id;
        this._onShowAction !== undefined && this._onShowAction(id);
    }

    public hide(): void {
        if (this.id === -1) {
            return;
        }
        let id = this.id;
        this.id = -1;
        this._onHideAction !== undefined && this._onHideAction(id);
    }
}
