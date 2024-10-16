"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.getDummyComposition = void 0;
const board_1 = require("board");
const types_1 = require("./types");
function getDummyComposition(comps, portsPerComp, deep, parent) {
    const components = [];
    for (let c = 0; c <= comps; c += 1) {
        const ports = [];
        for (let p = 0; p <= portsPerComp; p += 1) {
            ports.push({
                provided_interface: null,
                required_interface: null,
                provided_required_interface: null,
                visibility: true,
                port_type: Math.random() > 0.5 ? board_1.PortType.Left : board_1.PortType.Right,
                sig: (0, types_1.getSignature)(),
                connected: new Map(),
                contains: [],
            });
        }
        components.push({
            sig: (0, types_1.getSignature)(),
            ports: {
                Origin: {
                    ports: ports.map((p) => {
                        return {
                            Origin: p,
                        };
                    }),
                    hide_invisible: true,
                    sig: (0, types_1.getSignature)(),
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
                sig: (0, types_1.getSignature)(),
                joint_in: {
                    component: ports_a[c].comp,
                    port: ports_a[c].port,
                },
                joint_out: {
                    component: ports_b[c].comp,
                    port: ports_b[c].port,
                },
                visibility: true,
            });
        }
    }
    const ports = [];
    for (let p = 0; p <= portsPerComp; p += 1) {
        ports.push({
            Origin: {
                provided_interface: null,
                required_interface: null,
                provided_required_interface: null,
                port_type: Math.random() > 0.5 ? board_1.PortType.Left : board_1.PortType.Right,
                sig: (0, types_1.getSignature)(),
                visibility: true,
                connected: new Map(),
                contains: [],
            },
        });
    }
    const sig = (0, types_1.getSignature)();
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
        ports: { Origin: { ports, hide_invisible: true, sig: (0, types_1.getSignature)() } },
        parent,
    };
}
exports.getDummyComposition = getDummyComposition;
//# sourceMappingURL=dummy.js.map