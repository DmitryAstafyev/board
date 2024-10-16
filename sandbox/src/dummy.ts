import {
    Composition,
    Port,
    PortType,
    Representation,
    Connection,
    Component,
} from "board";

import { getSignature } from "./types";

export function getDummyComposition(
    comps: number,
    portsPerComp: number,
    deep: number,
    parent: number | undefined
): Composition {
    const components: Component[] = [];
    for (let c = 0; c <= comps; c += 1) {
        const ports: Port[] = [];
        for (let p = 0; p <= portsPerComp; p += 1) {
            ports.push({
                provided_interface: null,
                required_interface: null,
                provided_required_interface: null,
                visibility: true,
                port_type: Math.random() > 0.5 ? PortType.Left : PortType.Right,
                sig: getSignature(),
                connected: new Map(),
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
                    sig: getSignature(),
                },
            },
            composition: false,
        });
    }
    const connections: Connection[] = [];
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
                visibility: true,
            });
        }
    }
    const ports: Representation<Port>[] = [];
    for (let p = 0; p <= portsPerComp; p += 1) {
        ports.push({
            Origin: {
                provided_interface: null,
                required_interface: null,
                provided_required_interface: null,
                port_type: Math.random() > 0.5 ? PortType.Left : PortType.Right,
                sig: getSignature(),
                visibility: true,
                connected: new Map(),
                contains: [],
            },
        });
    }
    const sig = getSignature();
    const compositions: Composition[] = [];
    if (deep > 0) {
        for (let i = 0; i <= comps / 2; i += 1) {
            compositions.push(
                getDummyComposition(comps, portsPerComp, deep - 1, sig.id)
            );
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
        ports: { Origin: { ports, hide_invisible: true, sig: getSignature() } },
        parent,
    };
}
