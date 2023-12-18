"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const board_1 = require("board");
var Types;
(function (Types) {
    Types["PPortPrototype"] = "PPortPrototype";
    Types["AssemblySwConnector"] = "AssemblySwConnector";
    Types["PPortInCompositionInstanceRef"] = "PPortInCompositionInstanceRef";
    Types["RPortInCompositionInstanceRef"] = "RPortInCompositionInstanceRef";
    Types["RPortPrototype"] = "RPortPrototype";
    Types["SwComponentPrototype"] = "SwComponentPrototype";
    Types["ApplicationSwComponentType"] = "ApplicationSwComponentType";
    Types["ServiceSwComponentType"] = "ServiceSwComponentType";
    Types["ComplexDeviceDriverSwComponentType"] = "ComplexDeviceDriverSwComponentType";
    Types["CompositionSwComponentType"] = "CompositionSwComponentType";
})(Types || (Types = {}));
function asComponentPrototype(el) {
    return el.className == Types.SwComponentPrototype
        ? el
        : undefined;
}
function asComposition(el) {
    return el.className === Types.CompositionSwComponentType
        ? el
        : undefined;
}
function asComponentType(el) {
    return [
        Types.ComplexDeviceDriverSwComponentType,
        Types.ApplicationSwComponentType,
        Types.ServiceSwComponentType,
    ].includes(el.className)
        ? el
        : undefined;
}
function asConnection(el) {
    return el.className === Types.AssemblySwConnector
        ? el
        : undefined;
}
function asRPort(el) {
    return el.className === Types.RPortInCompositionInstanceRef
        ? el
        : undefined;
}
function asPPort(el) {
    return el.className === Types.PPortInCompositionInstanceRef
        ? el
        : undefined;
}
function getPortRef(components, id) {
    const component = components.find((c) => c.Origin.ports.Origin.ports.find((p) => p.Origin.sig.id == id) !==
        undefined);
    if (component === undefined) {
        return undefined;
    }
    return component.Origin.ports.Origin.ports.find((p) => p.Origin.sig.id == id);
}
const comps = [];
const all_ports = [];
function load(parent, elements, holder) {
    parent.component.forEach((id) => {
        const compPrototype = asComponentPrototype(find(id, elements));
        if (compPrototype === undefined) {
            console.error(`Element ${id} isn't IComponentPrototype`);
            return;
        }
        const smth = find(compPrototype.rType, elements);
        const composition = asComposition(smth);
        const componentType = asComponentType(smth);
        if (composition !== undefined) {
            all_ports.push(...composition.port);
            const nested = {
                sig: {
                    id,
                    class_name: composition.className,
                },
                components: [],
                connections: [],
                compositions: [],
                ports: {
                    Origin: {
                        ports: composition.port.map((port) => {
                            return {
                                Origin: {
                                    sig: {
                                        id: port,
                                        class_name: "unknown",
                                    },
                                    port_type: board_1.PortType.In,
                                    visibility: false,
                                    contains: [],
                                },
                            };
                        }),
                        hide_invisible: true,
                    },
                },
                parent: holder.sig.id,
            };
            load(composition, elements, nested);
            holder.compositions.push({
                Origin: nested,
            });
        }
        else if (componentType !== undefined) {
            comps.push(id);
            all_ports.push(...componentType.port);
            holder.components.push({
                Origin: {
                    sig: {
                        id,
                        class_name: componentType.className,
                    },
                    ports: {
                        Origin: {
                            ports: componentType.port.map((port) => {
                                return {
                                    Origin: {
                                        sig: {
                                            id: port,
                                            class_name: "unknown",
                                        },
                                        port_type: board_1.PortType.In,
                                        visibility: false,
                                        contains: [],
                                    },
                                };
                            }),
                            hide_invisible: true,
                        },
                    },
                    composition: false,
                },
            });
        }
        else {
            console.error(`Fail to detect type of ${smth.id}/${smth.className}: ${JSON.stringify(smth)}`);
        }
    });
    let notFoundConnectors = 0;
    parent.connector.forEach((connectionId) => {
        const connection = (() => {
            try {
                const smth = find(connectionId, elements);
                const connection = asConnection(smth);
                if (connection === undefined) {
                    console.error(`Entity ${connectionId} isn't connection: ${JSON.stringify(connection)}`);
                }
                return connection;
            }
            catch (_e) {
                notFoundConnectors += 1;
                return undefined;
            }
        })();
        if (connection === undefined) {
            return;
        }
        const pPort = asPPort(find(connection.provider, elements));
        const rPort = asRPort(find(connection.requester, elements));
        if (pPort === undefined || rPort === undefined) {
            console.error(`No ports`);
            return;
        }
        const pPortRef = getPortRef(holder.components, pPort.targetPPort);
        const rPortRef = getPortRef(holder.components, rPort.targetRPort);
        if (pPortRef !== undefined) {
            pPortRef.Origin.port_type = board_1.PortType.Out;
            pPortRef.Origin.visibility = true;
        }
        if (rPortRef !== undefined) {
            rPortRef.Origin.port_type = board_1.PortType.In;
            rPortRef.Origin.visibility = true;
        }
        holder.connections.push({
            Origin: {
                sig: {
                    id: connectionId,
                    class_name: connection.className,
                },
                joint_in: {
                    component: pPort.contextComponent,
                    port: pPort.targetPPort,
                },
                joint_out: {
                    component: rPort.contextComponent,
                    port: rPort.targetRPort,
                },
            },
        });
        // if (
        //     !parent.component.includes(pPort.contextComponent) ||
        //     !parent.component.includes(rPort.contextComponent)
        // ) {
        //     console.error(`contextComponent isn't found in components IDs`);
        // }
        // if (
        //     !comps.includes(pPort.contextComponent) ||
        //     !comps.includes(rPort.contextComponent)
        // ) {
        //     console.error(`contextComponent isn't found in comps IDs`);
        // }
    });
    if (notFoundConnectors > 0) {
        console.error(`Fail to find ${notFoundConnectors} connectors `);
    }
}
function find(id, elements) {
    const target = elements.find((el) => el.id === id);
    if (target === undefined) {
        throw new Error(`Fail to find element: ${id}`);
    }
    return target;
}
let signature = 1;
function getSignature() {
    const id = signature++;
    return { id, class_name: `class_name_${id}` };
}
function getDummyComposition(comps, portsPerComp, deep, parent) {
    const components = [];
    for (let c = 0; c <= comps; c += 1) {
        const ports = [];
        for (let p = 0; p <= portsPerComp; p += 1) {
            ports.push({
                visibility: true,
                port_type: Math.random() > 0.5 ? board_1.PortType.In : board_1.PortType.Out,
                sig: getSignature(),
                contains: [],
            });
        }
        components.push({
            sig: getSignature(),
            ports: {
                Origin: {
                    ports: ports.map((p) => {
                        return {
                            Origin: p,
                        };
                    }),
                    hide_invisible: true,
                },
            },
            composition: false,
        });
    }
    const connections = [];
    for (let i = 0; i <= components.length - 1; i += 2) {
        const a = components[i];
        const b = components[i + 1];
        if (a === undefined || b === undefined) {
            break;
        }
        const ports_a = a.ports.Origin.ports.map((p) => {
            return { port: p.Origin.sig.id, comp: a.sig.id };
        });
        const ports_b = b.ports.Origin.ports.map((p) => {
            return { port: p.Origin.sig.id, comp: b.sig.id };
        });
        const count = Math.round(ports_a.length / 2);
        for (let c = 0; c <= count; c += 1) {
            connections.push({
                sig: getSignature(),
                joint_in: {
                    component: ports_a[c].comp,
                    port: ports_a[c].port,
                },
                joint_out: {
                    component: ports_b[c].comp,
                    port: ports_b[c].port,
                },
            });
        }
    }
    const ports = [];
    for (let p = 0; p <= portsPerComp; p += 1) {
        ports.push({
            Origin: {
                port_type: Math.random() > 0.5 ? board_1.PortType.In : board_1.PortType.Out,
                sig: getSignature(),
                visibility: true,
                contains: [],
            },
        });
    }
    const sig = getSignature();
    const compositions = [];
    if (deep > 0) {
        for (let i = 0; i <= comps / 2; i += 1) {
            compositions.push(getDummyComposition(comps, portsPerComp, deep - 1, sig.id));
        }
    }
    return {
        sig,
        components: components.map((c) => {
            return { Origin: c };
        }),
        connections: connections.map((c) => {
            return { Origin: c };
        }),
        compositions: compositions.map((c) => {
            return { Origin: c };
        }),
        ports: { Origin: { ports, hide_invisible: true } },
        parent,
    };
}
function real() {
    setTimeout(() => {
        Promise.resolve().then(() => require("../resources/example.json")).then((data) => {
            const compositionId = data[0];
            const elements = data[1];
            const rootElement = find(compositionId, elements);
            const root = {
                sig: {
                    id: rootElement.id,
                    class_name: rootElement.className,
                },
                components: [],
                connections: [],
                compositions: [],
                ports: { Origin: { ports: [], hide_invisible: true } },
                parent: undefined,
            };
            const unique = [];
            elements.forEach((el) => {
                !unique.includes(el.className) && unique.push(el.className);
            });
            load(rootElement, elements, root);
            console.log(root);
            // console.log(JSON.stringify(root));
            // console.log(elements);
            const board = new board_1.Board(`div#container`);
            board.bind(root, []);
            board.render();
        });
    }, 200);
}
function dummy() {
    console.log("started");
    setTimeout(() => {
        console.log("point 0");
        const composition = getDummyComposition(10, 5, 2, undefined);
        console.log("point 1");
        console.log(composition);
        const board = new board_1.Board(`div#container`);
        console.log("point 2");
        board.bind(composition, []);
        console.log("point 3");
        board.render();
        console.log("point 4");
    }, 200);
}
console.log("zero");
dummy();
//# sourceMappingURL=main.js.map