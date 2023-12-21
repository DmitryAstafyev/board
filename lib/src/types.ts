//                         [ID    , Type, [x     , y     , x1    , y1    ]]
export type ElementCoors = [string, string, [number, number, number, number]];

export interface Signature {
    id: number;
    class_name: string;
}

export enum PortType {
    In = "In",
    Out = "Out",
}

export interface Port {
    sig: Signature;
    port_type: PortType;
    visibility: boolean;
    contains: number[];
}

export interface Representation<T> {
    Origin: T;
}

export interface Ports {
    ports: Representation<Port>[];
    hide_invisible: boolean;
}

export interface Joint {
    port: number;
    component: number;
}

export interface Connection {
    sig: Signature;
    joint_in: Joint;
    joint_out: Joint;
}

export interface Component {
    sig: Signature;
    ports: Representation<Ports>;
    composition: boolean;
}

export interface Composition {
    sig: Signature;
    components: Representation<Component>[];
    connections: Representation<Connection>[];
    compositions: Representation<Composition>[];
    ports: Representation<Ports>;
    parent: number | undefined;
}

export function getComposition(
    composition: Composition,
    target: number
): Composition | undefined {
    if (composition.sig.id === target) {
        return composition;
    }
    for (let nested of composition.compositions) {
        const found = getComposition(nested.Origin, target);
        if (found !== undefined) {
            return found;
        }
    }
    return undefined;
}
