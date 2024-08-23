"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.load = void 0;
const board_1 = require("board");
const types_1 = require("./types");
function load(parent, elements, holder) {
    function getDef(id) {
        if (typeof id !== "number") {
            return undefined;
        }
        return elements.find((el) => el.id === id);
    }
    function getSignatureFromEl(el) {
        return {
            id: el.id,
            class_name: el.className === undefined ? types_1.UNKNOWN : el.className,
            short_name: el.shortName === undefined ? types_1.UNKNOWN : el.shortName,
        };
    }
    parent.component.forEach((id) => {
        const prototype = (0, types_1.asComponentPrototype)((0, types_1.find)(id, elements));
        if (prototype === undefined) {
            console.error(`Element ${id} isn't IComponentPrototype`);
            return;
        }
        const smth = (0, types_1.find)(prototype.rType, elements);
        const composition = (0, types_1.asComposition)(smth);
        const componentType = (0, types_1.asComponentType)(smth);
        if (composition !== undefined) {
            const nested = {
                sig: {
                    id,
                    class_name: prototype.className,
                    short_name: prototype.shortName === undefined
                        ? types_1.UNKNOWN
                        : prototype.shortName,
                },
                components: [],
                connections: [],
                compositions: [],
                ports: {
                    Origin: {
                        ports: composition.port
                            .map((port) => {
                            const def = getDef(port);
                            if (def === undefined) {
                                console.error(`Port ${port} isn't found`);
                                return undefined;
                            }
                            const provided_interface = getDef(def.providedInterface);
                            const provided_required_interface = getDef(def.providedRequiredInterface);
                            const required_interface = getDef(def.requiredInterface);
                            return {
                                Origin: {
                                    sig: getSignatureFromEl(def),
                                    provided_interface: provided_interface !== undefined
                                        ? getSignatureFromEl(provided_interface)
                                        : null,
                                    provided_required_interface: provided_required_interface !==
                                        undefined
                                        ? getSignatureFromEl(provided_required_interface)
                                        : null,
                                    required_interface: required_interface !== undefined
                                        ? getSignatureFromEl(required_interface)
                                        : null,
                                    port_type: board_1.PortType.Unbound,
                                    visibility: true,
                                    connected: new Map(),
                                    contains: [],
                                },
                            };
                        })
                            .filter((p) => p !== undefined),
                        hide_invisible: true,
                        sig: (0, types_1.getSignature)(),
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
            holder.components.push({
                Origin: {
                    sig: {
                        id,
                        class_name: prototype.className,
                        short_name: prototype.shortName === undefined
                            ? types_1.UNKNOWN
                            : prototype.shortName,
                    },
                    ports: {
                        Origin: {
                            ports: componentType.port
                                .map((port) => {
                                const def = getDef(port);
                                if (def === undefined) {
                                    console.error(`Port ${port} isn't found`);
                                    return undefined;
                                }
                                const provided_interface = getDef(def.providedInterface);
                                const provided_required_interface = getDef(def.providedRequiredInterface);
                                const required_interface = getDef(def.requiredInterface);
                                return {
                                    Origin: {
                                        sig: getSignatureFromEl(def),
                                        provided_interface: provided_interface !== undefined
                                            ? getSignatureFromEl(provided_interface)
                                            : null,
                                        provided_required_interface: provided_required_interface !==
                                            undefined
                                            ? getSignatureFromEl(provided_required_interface)
                                            : null,
                                        required_interface: required_interface !== undefined
                                            ? getSignatureFromEl(required_interface)
                                            : null,
                                        port_type: board_1.PortType.Unbound,
                                        connected: new Map(),
                                        visibility: true,
                                        contains: [],
                                    },
                                };
                            })
                                .filter((p) => p !== undefined),
                            hide_invisible: true,
                            sig: (0, types_1.getSignature)(),
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
    const counts = new Map();
    parent.connector.forEach((connectionId) => {
        const connection = (() => {
            try {
                const smth = (0, types_1.find)(connectionId, elements);
                const connection = (0, types_1.asConnection)(smth);
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
        const pPortRef = (() => {
            const portId = connection.outerPort !== undefined
                ? connection.outerPort
                : connection.provider;
            const pPort = (0, types_1.asPPort)((0, types_1.find)(portId, elements));
            if (pPort !== undefined) {
                return (0, types_1.getPortRef)([
                    { Origin: holder },
                    holder.components,
                    holder.compositions,
                ].flat(), pPort.targetPPort);
            }
            const rPort = (0, types_1.asRPort)((0, types_1.find)(portId, elements));
            if (rPort !== undefined) {
                return (0, types_1.getPortRef)([
                    { Origin: holder },
                    holder.components,
                    holder.compositions,
                ].flat(), rPort.targetRPort);
            }
            return (0, types_1.getPortRef)([
                holder.components,
                holder.compositions,
                { Origin: holder },
            ].flat(), portId);
        })();
        const rPortRef = (() => {
            const portId = connection.innerPort !== undefined
                ? connection.innerPort
                : connection.requester;
            const rPort = (0, types_1.asRPort)((0, types_1.find)(portId, elements));
            if (rPort !== undefined) {
                return (0, types_1.getPortRef)([
                    { Origin: holder },
                    holder.components,
                    holder.compositions,
                ].flat(), rPort.targetRPort);
            }
            const pPort = (0, types_1.asPPort)((0, types_1.find)(portId, elements));
            if (pPort !== undefined) {
                return (0, types_1.getPortRef)([
                    { Origin: holder },
                    holder.components,
                    holder.compositions,
                ].flat(), pPort.targetPPort);
            }
            return (0, types_1.getPortRef)([
                holder.components,
                holder.compositions,
                { Origin: holder },
            ].flat(), portId);
        })();
        if (pPortRef === undefined || rPortRef === undefined) {
            console.error(`Cannot create connection definition`);
            return;
        }
        pPortRef[0].Origin.port_type = board_1.PortType.Out;
        pPortRef[0].Origin.visibility = true;
        rPortRef[0].Origin.port_type = board_1.PortType.In;
        rPortRef[0].Origin.visibility = true;
        let count = counts.get(pPortRef[0].Origin.sig.id);
        counts.set(pPortRef[0].Origin.sig.id, count === undefined ? 1 : count + 1);
        count = counts.get(rPortRef[0].Origin.sig.id);
        counts.set(rPortRef[0].Origin.sig.id, count === undefined ? 1 : count + 1);
        holder.connections.push({
            Origin: {
                sig: {
                    id: connectionId,
                    class_name: connection.className,
                    short_name: connection.shortName === undefined
                        ? types_1.UNKNOWN
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
    const updateCounts = (port) => {
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
exports.load = load;
//# sourceMappingURL=loader.js.map