//                         [ID    , Type, [x     , y     , x1    , y1    ]]
export type ElementCoors = [string, string, [number, number, number, number]];

export interface Signature {
    id: number;
    class_name: string;
    short_name: string;
}

export enum PortType {
    In = "In",
    Out = "Out",
    Unbound = "Unbound",
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
    sig: Signature;
}

export interface Joint {
    port: number;
    component: number;
}

export interface Connection {
    sig: Signature;
    joint_in: Joint;
    joint_out: Joint;
    visibility: boolean;
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

export enum PortsRepresentation {
    Blocks = "Blocks",
    Labels = "Labels",
}

export interface PortsOptions {
    representation: PortsRepresentation;
    grouping: boolean;
    group_unbound: boolean;
}

export interface ConnectionsOptions {
    hide: boolean;
}

export interface GridOptions {
    cell_size_px: number;
    cells_space_vertical: number;
    cells_space_horizontal: number;
    visible: boolean;
    padding: number;
}

export interface LabelsOptions {
    ports_short_name: boolean;
    components_short_name: boolean;
    composition_short_name: boolean;
    port_label_max_len: number;
    comp_label_max_len: number;
}
export interface Options {
    ports: PortsOptions;
    connections: ConnectionsOptions;
    grid: GridOptions;
    labels: LabelsOptions;
}

export function getDefaultsOptions(): Options {
    return {
        ports: {
            representation: PortsRepresentation.Blocks,
            grouping: true,
            group_unbound: true,
        },
        connections: {
            hide: false,
        },
        grid: {
            padding: 3,
            cell_size_px: 25,
            cells_space_vertical: 3,
            cells_space_horizontal: 3,
            visible: true,
        },
        labels: {
            ports_short_name: true,
            components_short_name: true,
            composition_short_name: true,
            port_label_max_len: 16,
            comp_label_max_len: 12,
        },
    };
}
