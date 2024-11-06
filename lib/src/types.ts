import { DEVICE_PIXEL_RATIO } from "./dom";
import { IPosition } from "./position";

export interface ILocation {
    id: number;
    sig: Signature;
}

export interface Match {
    id: number;
    holder: number | undefined;
    owner: number;
}

export interface Matches {
    ids: number[];
    extended: Match[];
    filter: string | undefined;
    currentIndex: number;
    currentId: number | undefined;
}
export interface State {
    composition: number | undefined;
    grouped: [number, number[]][];
    root: Composition | undefined;
    history: ILocation[];
}

export interface Snapshot {
    wasm: Uint8Array;
    state: State;
    position: IPosition;
    history: Map<number, IPosition>;
    matches: Matches;
}
//                         [ID    , Type, [x     , y     , x1    , y1    ]]
export type ElementCoors = [string, string, [number, number, number, number]];

export interface Signature {
    id: number;
    class_name: string;
    short_name: string;
}

export enum PortType {
    Left = "Left",
    Right = "Right",
}

export interface Port {
    sig: Signature;
    port_type: PortType;
    provided_interface: Signature | null;
    provided_required_interface: Signature | null;
    required_interface: Signature | null;
    visibility: boolean;
    contains: number[];
    connected: Map<number, number>;
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

export interface EntityProps {
    class_name: string[];
    short_name: string[];
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
    vpadding: number;
    hpadding: number;
    vmargin: number;
    hmargin: number;
}

export interface LabelsOptions {
    ports_short_name: boolean;
    components_short_name: boolean;
    composition_short_name: boolean;
    port_label_max_len: number;
    comp_label_max_len: number;
}

export interface RectColor {
    stroke: string;
    fill: string;
}

export interface ColorScheme {
    composition_rect: RectColor;
    composition_label: RectColor;
    composition_as_component_rect: RectColor;
    component_rect: RectColor;
    selected_rect: RectColor;
    highlighted_rect: RectColor;
    matched_rect: RectColor;
    hovered_rect: RectColor;
    connection_line: RectColor;
    port_highlighted_rect: RectColor;
    port_rect: RectColor;
    port_unlinked_rect: RectColor;
    port_linked_rect: RectColor;
    port_grouped_rect: RectColor;
    port_pri_bagde: RectColor;
    port_pi_bagde: RectColor;
    port_ri_bagde: RectColor;
    port_index_label: RectColor;
    port_subbagde: RectColor;
    label_subtitle: RectColor;
    label: RectColor;
}

export function getDefaultsColorScheme(): ColorScheme {
    return {
        composition_rect: {
            stroke: "rgb(0,0,0)",
            fill: "rgb(200,200,230)",
        },
        composition_label: {
            stroke: "rgb(30,30,30)",
            fill: "rgb(0,0,0)",
        },
        composition_as_component_rect: {
            stroke: "rgb(0,0,0)",
            fill: "rgb(250,200,200)",
        },
        component_rect: {
            stroke: "rgb(0,0,0)",
            fill: "rgb(240,240,240)",
        },
        selected_rect: {
            stroke: "rgb(0,0,0)",
            fill: "rgb(100,150,100)",
        },
        highlighted_rect: {
            stroke: "rgb(50,50,50)",
            fill: "rgb(185,230,255)",
        },
        port_highlighted_rect: {
            stroke: "rgb(0,0,0)",
            fill: "rgb(220,250,220)",
        },
        matched_rect: {
            stroke: "rgb(50,50,50)",
            fill: "rgb(195,190,190)",
        },
        hovered_rect: {
            stroke: "rgb(0,0,0)",
            fill: "rgb(200,200,200)",
        },
        connection_line: {
            stroke: "rgb(30,30,30)",
            fill: "rgb(30,30,30)",
        },
        port_rect: {
            stroke: "rgb(50,50,50)",
            fill: "rgb(240,240,240)",
        },
        port_unlinked_rect: {
            stroke: "rgb(50,50,50)",
            fill: "rgb(200,200,240)",
        },
        port_linked_rect: {
            stroke: "rgb(150,150,150)",
            fill: "rgb(250,250,250)",
        },
        port_grouped_rect: {
            stroke: "rgb(50,50,50)",
            fill: "rgb(255,255,200)",
        },
        port_pri_bagde: {
            stroke: "rgb(40,140,40)",
            fill: "rgb(255,255,255)",
        },
        port_pi_bagde: {
            stroke: "rgb(200,200,200)",
            fill: "rgb(0,0,0)",
        },
        port_ri_bagde: {
            stroke: "rgb(100,100,100)",
            fill: "rgb(255,255,255)",
        },
        port_index_label: {
            stroke: "rgb(0,0,0)",
            fill: "rgb(0,0,0)",
        },
        port_subbagde: {
            stroke: "rgb(240,240,240)",
            fill: "rgb(25,25,25)",
        },
        label_subtitle: {
            stroke: "rgb(40,40,40)",
            fill: "rgb(40,40,40)",
        },
        label: {
            stroke: "rgb(0,0,0)",
            fill: "rgb(0,0,0)",
        },
    };
}

export interface Options {
    ports: PortsOptions;
    connections: ConnectionsOptions;
    grid: GridOptions;
    labels: LabelsOptions;
    ratio: number;
    font: string;
    scheme: ColorScheme;
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
            vpadding: 3,
            hpadding: 5,
            vmargin: 0,
            hmargin: 5,
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
        ratio: DEVICE_PIXEL_RATIO,
        font: "Roboto, sans-serif",
        scheme: getDefaultsColorScheme(),
    };
}
