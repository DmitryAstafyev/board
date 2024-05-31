import { Composition, Component, Port, Signature, Representation } from "board";
export declare const UNKNOWN = "unknown";
export declare enum Types {
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
    DelegationSwConnector = "DelegationSwConnector"
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
export declare function asComponentPrototype(el: IElement): IComponentPrototype | undefined;
export declare function asComposition(el: IElement): IComposition | undefined;
export declare function asComponentType(el: IElement): IComposition | undefined;
export declare function asConnection(el: IElement): IConnection | undefined;
export declare function asRPort(el: IElement): IRPort | undefined;
export declare function asPPort(el: IElement): IPPort | undefined;
export declare function getPortRef(entries: Representation<Component | Composition>[], id: number): [Representation<Port>, number] | undefined;
export declare function find(id: number, elements: IElement[]): IElement;
export declare function getSignature(): Signature;
