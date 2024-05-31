import { Composition, Component, Port, Signature, Representation } from "board";

export const UNKNOWN = "unknown";

export enum Types {
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
    DelegationSwConnector = "DelegationSwConnector",
}

export interface IElement {
    id: number;
    className: string;
    shortName: string;
    providedInterface: number | null;
    providedRequiredInterface: number | null;
    requiredInterface: number | null;
}

export interface IConnection extends IElement {
    provider: number;
    requester: number;
    outerPort: number;
    innerPort: number;
}

export interface IComposition extends IElement {
    component: number[];
    connector: number[];
    port: number[];
}

export interface IComponentPrototype extends IElement {
    rType: number;
}

export interface IComponentType extends IElement {
    port: number[];
}

export interface IRPort extends IElement {
    targetRPort: number;
    contextComponent: number;
}

export interface IPPort extends IElement {
    targetPPort: number;
    contextComponent: number;
}

export function asComponentPrototype(
    el: IElement
): IComponentPrototype | undefined {
    return el.className == Types.SwComponentPrototype
        ? (el as IComponentPrototype)
        : undefined;
}

export function asComposition(el: IElement): IComposition | undefined {
    return el.className === Types.CompositionSwComponentType
        ? (el as IComposition)
        : undefined;
}

export function asComponentType(el: IElement): IComposition | undefined {
    return [
        Types.ComplexDeviceDriverSwComponentType,
        Types.ApplicationSwComponentType,
        Types.ServiceSwComponentType,
    ].includes(el.className as Types)
        ? (el as IComposition)
        : undefined;
}

export function asConnection(el: IElement): IConnection | undefined {
    return el.className === Types.AssemblySwConnector ||
        el.className == Types.DelegationSwConnector
        ? (el as IConnection)
        : undefined;
}

export function asRPort(el: IElement): IRPort | undefined {
    return el.className === Types.RPortInCompositionInstanceRef
        ? (el as IRPort)
        : undefined;
}

export function asPPort(el: IElement): IPPort | undefined {
    return el.className === Types.PPortInCompositionInstanceRef
        ? (el as IPPort)
        : undefined;
}

export function getPortRef(
    entries: Representation<Component | Composition>[],
    id: number
): [Representation<Port>, number] | undefined {
    const target = entries.find(
        (c) =>
            c.Origin.ports.Origin.ports.find((p) => p.Origin.sig.id == id) !==
            undefined
    );
    if (target === undefined) {
        return undefined;
    }
    const port = target.Origin.ports.Origin.ports.find(
        (p) => p.Origin.sig.id == id
    );
    if (port === undefined) {
        return undefined;
    }
    return [port, target.Origin.sig.id];
}

export function find(id: number, elements: IElement[]): IElement {
    const target = elements.find((el) => el.id === id);
    if (target === undefined) {
        throw new Error(`Fail to find element: ${id}`);
    }
    return target;
}

let signature: number = 1;

export function getSignature(): Signature {
    const id = signature++;
    return {
        id,
        class_name: `class_name_${id}`,
        short_name: `short_name_${id}`,
    };
}
