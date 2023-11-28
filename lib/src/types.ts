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
}

export interface Representation<T> {
    Origin: T;
}

export interface Ports {
    ports: Representation<Port>[];
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
}

export interface Composition {
    sig: Signature;
    components: Representation<Component>[];
    connections: Representation<Connection>[];
    compositions: Representation<Composition>[];
    ports: Representation<Ports>;
}
