require("core")
    .then((core: any) => {
        const COMPS = 500;
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
        board.render(0, 0);
        console.log(`Board rendered in: ${Date.now() - started}ms`);
        let movement = {
            progress: false,
            x: 0,
            y: 0,
            prev_x: 0,
            prev_y: 0,
        };
        window.addEventListener("mousedown", (event: MouseEvent) => {
            movement.progress = true;
            movement.prev_x = event.screenX;
            movement.prev_y = event.screenY;

            return true;
        });
        window.addEventListener("mousemove", (event: MouseEvent) => {
            if (!movement.progress) {
                return;
            }
            movement.x -= movement.prev_x - event.screenX;
            movement.y -= movement.prev_y - event.screenY;
            movement.prev_x = event.screenX;
            movement.prev_y = event.screenY;
            let started = Date.now();
            board.render(movement.x, movement.y);
            console.log(`Board rendered in: ${Date.now() - started}ms`);
        });
        window.addEventListener("mouseup", (event: MouseEvent) => {
            movement.progress = false;
        });
    })
    .catch((err: Error) => {
        console.error(err.message);
        // To drop trace also
        throw err;
    });
