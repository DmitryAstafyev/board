import { Composition, Port, PortType, Signature, Representation } from "board";

import {
    IComposition,
    IElement,
    UNKNOWN,
    IComponentPrototype,
    getPortRef,
    getSignature,
    asComponentPrototype,
    asComponentType,
    asComposition,
    asConnection,
    asPPort,
    asRPort,
    find,
} from "./types";

function getPortType(
    pi: unknown | undefined,
    ri: unknown | undefined,
    pri: unknown | undefined
): PortType {
    return pri !== undefined
        ? PortType.Right
        : ri !== undefined
        ? PortType.Left
        : pi !== undefined
        ? PortType.Right
        : PortType.Left;
}

export function load(
    parent: IComposition,
    elements: IElement[],
    holder: Composition
) {
    function getDef(id: number | undefined | null): IElement | undefined {
        if (typeof id !== "number") {
            return undefined;
        }
        return elements.find((el) => el.id === id);
    }
    function getSignatureFromEl(el: IElement): Signature {
        return {
            id: el.id,
            class_name: el.className === undefined ? UNKNOWN : el.className,
            short_name: el.shortName === undefined ? UNKNOWN : el.shortName,
        };
    }
    parent.component.forEach((id: number) => {
        const prototype: IComponentPrototype | undefined = asComponentPrototype(
            find(id, elements)
        );
        if (prototype === undefined) {
            console.error(`Element ${id} isn't IComponentPrototype`);
            return;
        }
        const smth = find(prototype.rType, elements);
        const composition = asComposition(smth);
        const componentType = asComponentType(smth);
        if (composition !== undefined) {
            const nested: Composition = {
                sig: {
                    id,
                    class_name: prototype.className,
                    short_name:
                        prototype.shortName === undefined
                            ? UNKNOWN
                            : prototype.shortName,
                },
                components: [],
                connections: [],
                compositions: [],
                ports: {
                    Origin: {
                        ports: composition.port
                            .map((port: number) => {
                                const def = getDef(port);
                                if (def === undefined) {
                                    console.error(`Port ${port} isn't found`);
                                    return undefined;
                                }
                                const provided_interface = getDef(
                                    def.providedInterface
                                );
                                const provided_required_interface = getDef(
                                    def.providedRequiredInterface
                                );
                                const required_interface = getDef(
                                    def.requiredInterface
                                );
                                return {
                                    Origin: {
                                        sig: getSignatureFromEl(def),
                                        provided_interface:
                                            provided_interface !== undefined
                                                ? getSignatureFromEl(
                                                      provided_interface
                                                  )
                                                : null,
                                        provided_required_interface:
                                            provided_required_interface !==
                                            undefined
                                                ? getSignatureFromEl(
                                                      provided_required_interface
                                                  )
                                                : null,
                                        required_interface:
                                            required_interface !== undefined
                                                ? getSignatureFromEl(
                                                      required_interface
                                                  )
                                                : null,
                                        port_type: getPortType(
                                            provided_interface,
                                            required_interface,
                                            provided_required_interface
                                        ),
                                        visibility: true,
                                        connected: new Map(),
                                        contains: [],
                                    } as Port,
                                };
                            })
                            .filter(
                                (p) => p !== undefined
                            ) as unknown as Representation<Port>[],
                        hide_invisible: true,
                        sig: getSignature(),
                    },
                },
                parent: holder.sig.id,
            };
            load(composition, elements, nested);
            holder.compositions.push({
                Origin: nested,
            });
        } else if (componentType !== undefined) {
            holder.components.push({
                Origin: {
                    sig: {
                        id,
                        class_name: prototype.className,
                        short_name:
                            prototype.shortName === undefined
                                ? UNKNOWN
                                : prototype.shortName,
                    },
                    ports: {
                        Origin: {
                            ports: componentType.port
                                .map((port: number) => {
                                    const def = getDef(port);
                                    if (def === undefined) {
                                        console.error(
                                            `Port ${port} isn't found`
                                        );
                                        return undefined;
                                    }
                                    const provided_interface = getDef(
                                        def.providedInterface
                                    );
                                    const provided_required_interface = getDef(
                                        def.providedRequiredInterface
                                    );
                                    const required_interface = getDef(
                                        def.requiredInterface
                                    );
                                    return {
                                        Origin: {
                                            sig: getSignatureFromEl(def),
                                            provided_interface:
                                                provided_interface !== undefined
                                                    ? getSignatureFromEl(
                                                          provided_interface
                                                      )
                                                    : null,
                                            provided_required_interface:
                                                provided_required_interface !==
                                                undefined
                                                    ? getSignatureFromEl(
                                                          provided_required_interface
                                                      )
                                                    : null,
                                            required_interface:
                                                required_interface !== undefined
                                                    ? getSignatureFromEl(
                                                          required_interface
                                                      )
                                                    : null,
                                            port_type: getPortType(
                                                provided_interface,
                                                required_interface,
                                                provided_required_interface
                                            ),
                                            connected: new Map(),
                                            visibility: true,
                                            contains: [],
                                        } as Port,
                                    };
                                })
                                .filter(
                                    (p) => p !== undefined
                                ) as unknown as Representation<Port>[],
                            hide_invisible: true,
                            sig: getSignature(),
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
    const counts: Map<number, number> = new Map();
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
        const pPortRef = (() => {
            const portId =
                connection.outerPort !== undefined
                    ? connection.outerPort
                    : connection.provider;
            const pPort = asPPort(find(portId, elements));
            if (pPort !== undefined) {
                return getPortRef(
                    [
                        { Origin: holder },
                        holder.components,
                        holder.compositions,
                    ].flat(),
                    pPort.targetPPort
                );
            }
            const rPort = asRPort(find(portId, elements));
            if (rPort !== undefined) {
                return getPortRef(
                    [
                        { Origin: holder },
                        holder.components,
                        holder.compositions,
                    ].flat(),
                    rPort.targetRPort
                );
            }
            return getPortRef(
                [
                    holder.components,
                    holder.compositions,
                    { Origin: holder },
                ].flat(),
                portId
            );
        })();
        const rPortRef = (() => {
            const portId =
                connection.innerPort !== undefined
                    ? connection.innerPort
                    : connection.requester;
            const rPort = asRPort(find(portId, elements));
            if (rPort !== undefined) {
                return getPortRef(
                    [
                        { Origin: holder },
                        holder.components,
                        holder.compositions,
                    ].flat(),
                    rPort.targetRPort
                );
            }
            const pPort = asPPort(find(portId, elements));
            if (pPort !== undefined) {
                return getPortRef(
                    [
                        { Origin: holder },
                        holder.components,
                        holder.compositions,
                    ].flat(),
                    pPort.targetPPort
                );
            }
            return getPortRef(
                [
                    holder.components,
                    holder.compositions,
                    { Origin: holder },
                ].flat(),
                portId
            );
        })();
        if (pPortRef === undefined || rPortRef === undefined) {
            console.error(`Cannot create connection definition`);
            return;
        }
        // pPortRef[0].Origin.port_type = PortType.Right;
        pPortRef[0].Origin.visibility = true;
        // rPortRef[0].Origin.port_type = PortType.Left;
        rPortRef[0].Origin.visibility = true;
        let count = counts.get(pPortRef[0].Origin.sig.id);
        counts.set(
            pPortRef[0].Origin.sig.id,
            count === undefined ? 1 : count + 1
        );
        count = counts.get(rPortRef[0].Origin.sig.id);
        counts.set(
            rPortRef[0].Origin.sig.id,
            count === undefined ? 1 : count + 1
        );
        holder.connections.push({
            Origin: {
                sig: {
                    id: connectionId,
                    class_name: connection.className,
                    short_name:
                        connection.shortName === undefined
                            ? UNKNOWN
                            : connection.shortName,
                },
                joint_in: {
                    component: pPortRef[1],
                    port: pPortRef[0].Origin.sig.id,
                },
                joint_out: {
                    component: rPortRef[1],
                    port: rPortRef[0].Origin.sig.id,
                },
                visibility: true,
            },
        });
    });
    const updateCounts = (port: Port) => {
        const count = counts.get(port.sig.id);
        if (count !== undefined) {
            port.connected.set(holder.sig.id, count);
        }
    };
    holder.ports.Origin.ports.forEach((port) => {
        updateCounts(port.Origin);
    });
    holder.components.forEach((comp) => {
        comp.Origin.ports.Origin.ports.forEach((port) => {
            updateCounts(port.Origin);
        });
    });
    holder.compositions.forEach((comp) => {
        comp.Origin.ports.Origin.ports.forEach((port) => {
            updateCounts(port.Origin);
        });
    });
    if (notFoundConnectors > 0) {
        console.error(`Fail to find ${notFoundConnectors} connectors `);
    }
}
