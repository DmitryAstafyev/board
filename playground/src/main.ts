require("core")
    .then((core: any) => {
        const COMPS = 250;
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
        };
        window.addEventListener("mousedown", (event: MouseEvent) => {
            state.progress = true;
            state.prev_x = event.screenX;
            state.prev_y = event.screenY;
            return true;
        });
        window.addEventListener("mousemove", (event: MouseEvent) => {
            if (!state.progress) {
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
            console.log(`Board rendered in: ${Date.now() - started}ms`);
        });
        window.addEventListener("mouseup", (event: MouseEvent) => {
            state.progress = false;
        });
        window.addEventListener("wheel", (event: WheelEvent) => {
            state.zoom += event.deltaY > 0 ? 0.05 : -0.05;
            state.zoom = state.zoom < 0.1 ? 0.1 : state.zoom;
            state.zoom = state.zoom > 2 ? 2 : state.zoom;
            console.log(`Zooming: ${state.zoom}`);
            board.render(state.x, state.y, state.zoom);
        });
    })
    .catch((err: Error) => {
        console.error(err.message);
        // To drop trace also
        throw err;
    });
