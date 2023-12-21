import {
    Board,
    Composition,
    Component,
    Port,
    Connection,
    PortType,
    Signature,
    Representation,
} from "board";

enum Types {
    PPortPrototype = "PPortPrototype",
    AssemblySwConnector = "AssemblySwConnector",
    PPortInCompositionInstanceRef = "PPortInCompositionInstanceRef",
    RPortInCompositionInstanceRef = "RPortInCompositionInstanceRef",
    RPortPrototype = "RPortPrototype",
    SwComponentPrototype = "SwComponentPrototype",
    ApplicationSwComponentType = "ApplicationSwComponentType",
    ServiceSwComponentType = "ServiceSwComponentType",
    ComplexDeviceDriverSwComponentType = "ComplexDeviceDriverSwComponentType",
    CompositionSwComponentType = "CompositionSwComponentType",
}
interface IElement {
    id: number;
    className: string;
    shortName: string;
}

interface IConnection extends IElement {
    provider: number;
    requester: number;
}

interface IComposition extends IElement {
    component: number[];
    connector: number[];
    port: number[];
}

interface IComponentPrototype extends IElement {
    rType: number;
}

interface IComponentType extends IElement {
    port: number[];
}

interface IRPort extends IElement {
    targetRPort: number;
    contextComponent: number;
}

interface IPPort extends IElement {
    targetPPort: number;
    contextComponent: number;
}

function asComponentPrototype(el: IElement): IComponentPrototype | undefined {
    return el.className == Types.SwComponentPrototype
        ? (el as IComponentPrototype)
        : undefined;
}

function asComposition(el: IElement): IComposition | undefined {
    return el.className === Types.CompositionSwComponentType
        ? (el as IComposition)
        : undefined;
}

function asComponentType(el: IElement): IComposition | undefined {
    return [
        Types.ComplexDeviceDriverSwComponentType,
        Types.ApplicationSwComponentType,
        Types.ServiceSwComponentType,
    ].includes(el.className as Types)
        ? (el as IComposition)
        : undefined;
}

function asConnection(el: IElement): IConnection | undefined {
    return el.className === Types.AssemblySwConnector
        ? (el as IConnection)
        : undefined;
}

function asRPort(el: IElement): IRPort | undefined {
    return el.className === Types.RPortInCompositionInstanceRef
        ? (el as IRPort)
        : undefined;
}

function asPPort(el: IElement): IPPort | undefined {
    return el.className === Types.PPortInCompositionInstanceRef
        ? (el as IPPort)
        : undefined;
}

function getPortRef(
    components: Representation<Component>[],
    id: number
): Representation<Port> | undefined {
    const component = components.find(
        (c) =>
            c.Origin.ports.Origin.ports.find((p) => p.Origin.sig.id == id) !==
            undefined
    );
    if (component === undefined) {
        return undefined;
    }
    return component.Origin.ports.Origin.ports.find(
        (p) => p.Origin.sig.id == id
    );
}
const comps: number[] = [];
const all_ports: number[] = [];

function load(parent: IComposition, elements: IElement[], holder: Composition) {
    parent.component.forEach((id: number) => {
        const compPrototype: IComponentPrototype | undefined =
            asComponentPrototype(find(id, elements));
        if (compPrototype === undefined) {
            console.error(`Element ${id} isn't IComponentPrototype`);
            return;
        }
        const smth = find(compPrototype.rType, elements);
        const composition = asComposition(smth);
        const componentType = asComponentType(smth);
        if (composition !== undefined) {
            all_ports.push(...composition.port);
            const nested: Composition = {
                sig: {
                    id,
                    class_name: composition.className,
                },
                components: [],
                connections: [],
                compositions: [],
                ports: {
                    Origin: {
                        ports: composition.port.map((port: number) => {
                            return {
                                Origin: {
                                    sig: {
                                        id: port,
                                        class_name: "unknown",
                                    },
                                    port_type: PortType.In,
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
        } else if (componentType !== undefined) {
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
                            ports: componentType.port.map((port: number) => {
                                return {
                                    Origin: {
                                        sig: {
                                            id: port,
                                            class_name: "unknown",
                                        },
                                        port_type: PortType.In,
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
        } else {
            console.error(
                `Fail to detect type of ${smth.id}/${
                    smth.className
                }: ${JSON.stringify(smth)}`
            );
        }
    });
    let notFoundConnectors = 0;
    parent.connector.forEach((connectionId: number) => {
        const connection = (() => {
            try {
                const smth = find(connectionId, elements);
                const connection = asConnection(smth);
                if (connection === undefined) {
                    console.error(
                        `Entity ${connectionId} isn't connection: ${JSON.stringify(
                            connection
                        )}`
                    );
                }
                return connection;
            } catch (_e) {
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
            pPortRef.Origin.port_type = PortType.Out;
            pPortRef.Origin.visibility = true;
        }
        if (rPortRef !== undefined) {
            rPortRef.Origin.port_type = PortType.In;
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

function find(id: number, elements: IElement[]): IElement {
    const target = elements.find((el) => el.id === id);
    if (target === undefined) {
        throw new Error(`Fail to find element: ${id}`);
    }
    return target;
}

let signature: number = 1;

function getSignature(): Signature {
    const id = signature++;
    return { id, class_name: `class_name_${id}` };
}

function getDummyComposition(
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
                visibility: true,
                port_type: Math.random() > 0.5 ? PortType.In : PortType.Out,
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
            });
        }
    }
    const ports: Representation<Port>[] = [];
    for (let p = 0; p <= portsPerComp; p += 1) {
        ports.push({
            Origin: {
                port_type: Math.random() > 0.5 ? PortType.In : PortType.Out,
                sig: getSignature(),
                visibility: true,
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
        ports: { Origin: { ports, hide_invisible: true } },
        parent,
    };
}

function real() {
    setTimeout(() => {
        import("../resources/example.json").then((data: any) => {
            const compositionId: number = data[0];
            const elements: IElement[] = data[1];
            const rootElement = find(compositionId, elements);
            const root: Composition = {
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
            const unique: string[] = [];
            elements.forEach((el) => {
                !unique.includes(el.className) && unique.push(el.className);
            });
            load(rootElement as IComposition, elements, root);
            console.log(root);
            // console.log(JSON.stringify(root));
            // console.log(elements);
            const board = new Board(`div#container`);
            board.bind(root, []);
            board.render();
            board.subjects.get().onPortHover.subscribe((event) => {
                console.log(event);
            });
            board.subjects.get().onComponentHover.subscribe((event) => {
                console.log(event);
            });
        });
    }, 200);
}

function dummy() {
    setTimeout(() => {
        const composition = getDummyComposition(10, 5, 2, undefined);
        const board = new Board(`div#container`);
        board.bind(composition, []);
        board.render();
        board.subjects.get().onPortHover.subscribe((event) => {
            console.log(event);
        });
        board.subjects.get().onComponentHover.subscribe((event) => {
            console.log(event);
        });
    }, 200);
}

real();
