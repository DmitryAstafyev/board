import * as Core from "core";

const wasm: {
    core: typeof Core | undefined;
} = {
    core: undefined,
};

import("core")
    .then((core: typeof Core) => {
        wasm.core = core;
    })
    .catch((err: Error) => {
        console.error(`Fail to core load wasm module`);
    });
import("core")
    .then((core: typeof Core) => {
        const COMPS = 200;
        const PORTS = 8;
        let started = Date.now();
        const board = core.Board.dummy(COMPS, PORTS);
        board.bind("playground");
        console.log(
            `Board craeted for ${COMPS} and ${PORTS * COMPS} ports in: ${
                Date.now() - started
            }ms`
        );
        started = Date.now();
        board.init();
        console.log(`Board inited (calculated) in: ${Date.now() - started}ms`);
        started = Date.now();
        board.render(0, 0, 1);
        console.log(`Board rendered in: ${Date.now() - started}ms`);
        let state = {
            progress: false,
            x: 0,
            y: 0,
            prev_x: 0,
            prev_y: 0,
            zoom: 1,
            max: 0,
            hover: -1,
        };
        const update = (ms: number) => {
            if (state.max < ms) {
                state.max = ms;
                (
                    document.querySelector("#max") as HTMLElement
                ).innerHTML = `${ms}ms`;
            }
            (
                document.querySelector("#last") as HTMLElement
            ).innerHTML = `${ms}ms`;
        };
        window.addEventListener("mousedown", (event: MouseEvent) => {
            state.progress = true;
            state.prev_x = event.screenX;
            state.prev_y = event.screenY;
            return true;
        });
        window.addEventListener("mousemove", (event: MouseEvent) => {
            if (!state.progress) {
                const ids = board.who(
                    event.clientX,
                    event.clientY,
                    state.x,
                    state.y,
                    state.zoom
                );
                if (state.hover !== -1) {
                    board.draw_by_id(
                        state.hover,
                        undefined,
                        undefined,
                        state.x,
                        state.y,
                        state.zoom
                    );
                    state.hover = -1;
                }
                if (ids.length === 1) {
                    // console.log(
                    //     `(${event.clientX}, ${event.clientY}): ${ids[0]}`
                    // );
                    state.hover = ids[0];
                    board.draw_by_id(
                        state.hover,
                        "rgb(0,250,0)",
                        "rgb(0,250,0)",
                        state.x,
                        state.y,
                        state.zoom
                    );
                }
                return;
            }
            state.x -=
                (state.prev_x - event.screenX) *
                (state.zoom < 0 ? 1 / state.zoom : 1);
            state.y -=
                (state.prev_y - event.screenY) *
                (state.zoom < 0 ? 1 / state.zoom : 1);
            state.prev_x = event.screenX;
            state.prev_y = event.screenY;
            let started = Date.now();
            board.render(state.x, state.y, state.zoom);
            update(Date.now() - started);
        });
        window.addEventListener("mouseup", (event: MouseEvent) => {
            state.progress = false;
        });
        window.addEventListener("wheel", (event: WheelEvent) => {
            state.zoom += event.deltaY > 0 ? 0.05 : -0.05;
            state.zoom = state.zoom < 0.1 ? 0.1 : state.zoom;
            state.zoom = state.zoom > 2 ? 2 : state.zoom;
            let started = Date.now();
            board.render(state.x, state.y, state.zoom);
            update(Date.now() - started);
        });
    })
    .catch((err: Error) => {
        console.error(err.message);
        // To drop trace also
        throw err;
    });
