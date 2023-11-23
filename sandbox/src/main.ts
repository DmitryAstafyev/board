import {
    Board,
    Composition,
    Component,
    Port,
    Connection,
    PortType,
    Signature,
} from "board";

function getDummyComposition(comps: number, portsPerComp: number): Composition {
    let signature: number = 1;
    const getSignature = (): Signature => {
        const id = signature++;
        return { id, class_name: `class_name_${id}` };
    };
    const components: Component[] = [];
    for (let c = 0; c <= comps; c += 1) {
        const ports: Port[] = [];
        for (let p = 0; p <= portsPerComp; p += 1) {
            ports.push({
                port_type: Math.random() > 0.5 ? PortType.In : PortType.Out,
                sig: getSignature(),
            });
        }
        components.push({
            sig: getSignature(),
            ports: {
                ports: ports.map((p) => {
                    return {
                        Origin: p,
                    };
                }),
            },
        });
    }
    const connections: Connection[] = [];
    for (let i = 0; i <= components.length - 1; i += 2) {
        const a = components[i];
        const b = components[i + 1];
        if (a === undefined || b === undefined) {
            break;
        }
        const ports_a = a.ports.ports.map((p) => {
            return { port: p.Origin.sig.id, comp: a.sig.id };
        });
        const ports_b = b.ports.ports.map((p) => {
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
    return {
        sig: getSignature(),
        components: components.map((c) => {
            return { Origin: c };
        }),
        connections: connections.map((c) => {
            return { Origin: c };
        }),
    };
}
setTimeout(() => {
    const composition = getDummyComposition(10, 10);
    console.log(composition);
    const board = new Board(`div#container`);
    board.init(composition);
    board.render();
}, 200);
