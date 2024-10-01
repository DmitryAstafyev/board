import {
    Board,
    Composition,
    Options,
    DEVICE_PIXEL_RATIO,
    PortsRepresentation,
    MatchesEvent,
} from "board";

import {
    IComposition,
    IElement,
    UNKNOWN,
    getSignature,
    find,
    Types,
} from "./types";
import { getDummyComposition } from "./dummy";
import { load } from "./loader";

function getLabeledPortsOptions(): Options {
    return {
        ports: {
            representation: PortsRepresentation.Labels,
            grouping: true,
            group_unbound: true,
        },
        connections: {
            hide: false,
        },
        grid: {
            hpadding: 5,
            vpadding: 3,
            hmargin: 5,
            vmargin: 0,
            cell_size_px: 25,
            cells_space_vertical: 3,
            cells_space_horizontal: 11,
            visible: false,
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
                    short_name:
                        rootElement.shortName === undefined
                            ? UNKNOWN
                            : rootElement.shortName,
                },
                components: [],
                connections: [],
                compositions: [],
                ports: {
                    Origin: {
                        ports: [],
                        hide_invisible: true,
                        sig: getSignature(),
                    },
                },
                parent: undefined,
            };
            const unique: string[] = [];
            elements.forEach((el) => {
                !unique.includes(el.className) && unique.push(el.className);
            });
            load(rootElement as IComposition, elements, root);
            const board = new Board(`div#container`, getLabeledPortsOptions());
            board.subjects.get().onPortHover.subscribe((event) => {});
            board.subjects.get().onPortClick.subscribe((event) => {
                console.log(`Click on: ${event}`);
                console.log(board.getPort(event));
                // console.log(board.getPortsProps());
                // console.log(board.getCompsProps());
            });
            board.subjects.get().onComponentHover.subscribe((event) => {});
            board.subjects.get().onSelectionChange.subscribe((event) => {
                console.log(`Selection: ${JSON.stringify(event)}`);
            });
            const filter = document.querySelector(
                'input[id="filter"]'
            ) as HTMLInputElement;
            filter.addEventListener("keyup", () => {
                board
                    .filter()
                    .set(
                        filter.value.trim() === ""
                            ? undefined
                            : filter.value.trim()
                    );
                board.refresh();
            });
            filter.addEventListener("change", () => {
                // board.refresh();
            });
            const back = document.querySelector(
                'span[id="back"]'
            ) as HTMLSpanElement;
            back.addEventListener("click", () => {
                board.toPrevComposition();
            });
            const location = document.querySelector(
                'span[id="location"]'
            ) as HTMLSpanElement;
            board.subjects.get().onLocationChange.subscribe((locations) => {
                location.innerHTML = locations
                    .map((l) => l.sig.short_name)
                    .join("/");
            });
            const matches = document.querySelector(
                'input[id="matches"]'
            ) as HTMLInputElement;
            matches.addEventListener("keyup", () => {
                board
                    .matches()
                    .set(
                        matches.value.trim() === ""
                            ? undefined
                            : matches.value.trim()
                    );
            });
            (
                document.querySelector('span[id="prev"]') as HTMLSpanElement
            ).addEventListener("click", () => {
                board.matches().prev();
            });
            (
                document.querySelector('span[id="next"]') as HTMLSpanElement
            ).addEventListener("click", () => {
                board.matches().next();
            });
            const matches_state = document.querySelector(
                'span[id="matches_state"]'
            ) as HTMLSpanElement;
            board.subjects
                .get()
                .onMatches.subscribe((event: MatchesEvent | undefined) => {
                    if (event === undefined) {
                        matches_state.innerHTML = "";
                    } else {
                        matches_state.innerHTML = `${event.current}/${event.total} (${event.id})`;
                    }
                    console.log(board.matches().extended());
                });
            board.subjects.get().onContextMenu.subscribe((event) => {
                console.log(event);
            });
            board.bind(root);
            board.render();
        });
    }, 200);
}

function dummy() {
    setTimeout(() => {
        const composition = getDummyComposition(10, 5, 2, undefined);
        const board = new Board(`div#container`, getLabeledPortsOptions());
        board.bind(composition);
        board.render();
        board.subjects.get().onPortHover.subscribe((event) => {});
        board.subjects.get().onComponentHover.subscribe((event) => {});
    }, 200);
}

real();
