require("core")
    .then((core: any) => {
        console.log(core.hello("Hello From Wasm"));
    })
    .catch((err: Error) => {
        console.error(err.message);
        // To drop trace also
        throw err;
    });
