"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.getSignature = exports.find = exports.getPortRef = exports.asPPort = exports.asRPort = exports.asConnection = exports.asComponentType = exports.asComposition = exports.asComponentPrototype = exports.Types = exports.UNKNOWN = void 0;
exports.UNKNOWN = "unknown";
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
    Types["DelegationSwConnector"] = "DelegationSwConnector";
})(Types || (exports.Types = Types = {}));
function asComponentPrototype(el) {
    return el.className == Types.SwComponentPrototype
        ? el
        : undefined;
}
exports.asComponentPrototype = asComponentPrototype;
function asComposition(el) {
    return el.className === Types.CompositionSwComponentType
        ? el
        : undefined;
}
exports.asComposition = asComposition;
function asComponentType(el) {
    return [
        Types.ComplexDeviceDriverSwComponentType,
        Types.ApplicationSwComponentType,
        Types.ServiceSwComponentType,
    ].includes(el.className)
        ? el
        : undefined;
}
exports.asComponentType = asComponentType;
function asConnection(el) {
    return el.className === Types.AssemblySwConnector ||
        el.className == Types.DelegationSwConnector
        ? el
        : undefined;
}
exports.asConnection = asConnection;
function asRPort(el) {
    return el.className === Types.RPortInCompositionInstanceRef
        ? el
        : undefined;
}
exports.asRPort = asRPort;
function asPPort(el) {
    return el.className === Types.PPortInCompositionInstanceRef
        ? el
        : undefined;
}
exports.asPPort = asPPort;
function getPortRef(entries, id) {
    const target = entries.find((c) => c.Origin.ports.Origin.ports.find((p) => p.Origin.sig.id == id) !==
        undefined);
    if (target === undefined) {
        return undefined;
    }
    const port = target.Origin.ports.Origin.ports.find((p) => p.Origin.sig.id == id);
    if (port === undefined) {
        return undefined;
    }
    return [port, target.Origin.sig.id];
}
exports.getPortRef = getPortRef;
function find(id, elements) {
    const target = elements.find((el) => el.id === id);
    if (target === undefined) {
        throw new Error(`Fail to find element: ${id}`);
    }
    return target;
}
exports.find = find;
let signature = 1;
function getSignature() {
    const id = signature++;
    return {
        id,
        class_name: `class_name_${id}`,
        short_name: `short_name_${id}`,
    };
}
exports.getSignature = getSignature;
//# sourceMappingURL=types.js.map