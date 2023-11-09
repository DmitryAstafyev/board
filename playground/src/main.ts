require("core")
    .then((core: any) => {
        const COMPS = 8;
        const PORTS = 8;
        const started = Date.now();
        core.dummy("playground", COMPS, PORTS);
        console.log(
            `Done for ${COMPS} and ${PORTS * COMPS} ports in: ${
                Date.now() - started
            }ms`
        );
    })
    .catch((err: Error) => {
        console.error(err.message);
        // To drop trace also
        throw err;
    });
