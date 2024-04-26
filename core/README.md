# Board

## What is it?

Render for autosar diagram, shows relations between components/compositions

## General functionality

- Shows map of components & compositions with relatations (connections)
- Supports scrolling & zooming
- For best performance renders only part of diagram in viewport
- Allows navigation between compositions

## Features

- Supports filtering of ports
- Highlights of selected connections
- Group ports

## Developing

> Note. Build scripts currently works only with linux and mac.

### Build lib

```
cd lib
sh ./build.sh
```

Result:
- build wasm module (./core)
- build npm-package with lib (./lib)

### Build sandbox

Sandbox project is used for testing library. By default it uses example-data `sandbox/resources/example.json`, but by legacy and security reasons this data cannot be included into repository. In addition sandbox can be switched to using of `dummy` data.

To switch sandbox into `dummy` mode, change file `sandbox/src/main.ts` (end of file):

```
// real();     <- comment this
dummy();    // <- add this
```

Building:

```
cd sandbox
sh ./start.sh
```

Result:
- build wasm module (./core)
- build npm-package with lib (./lib)
- build playground project
- run local server (http://localhost:8888) with playground project