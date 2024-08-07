"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const board_1 = require("board");
const types_1 = require("./types");
const dummy_1 = require("./dummy");
const loader_1 = require("./loader");
function getLabeledPortsOptions() {
    return {
        ports: {
            representation: board_1.PortsRepresentation.Labels,
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
            cells_space_horizontal: 10,
            visible: false,
        },
        labels: {
            ports_short_name: true,
            components_short_name: true,
            composition_short_name: true,
            port_label_max_len: 16,
            comp_label_max_len: 12,
        },
        ratio: board_1.DEVICE_PIXEL_RATIO,
    };
}
function real() {
    setTimeout(() => {
        Promise.resolve().then(() => require("../resources/example.json")).then((data) => {
            const compositionId = data[0];
            const elements = data[1];
            const rootElement = (0, types_1.find)(compositionId, elements);
            const root = {
                sig: {
                    id: rootElement.id,
                    class_name: rootElement.className,
                    short_name: rootElement.shortName === undefined
                        ? types_1.UNKNOWN
                        : rootElement.shortName,
                },
                components: [],
                connections: [],
                compositions: [],
                ports: {
                    Origin: {
                        ports: [],
                        hide_invisible: true,
                        sig: (0, types_1.getSignature)(),
                    },
                },
                parent: undefined,
            };
            const unique = [];
            elements.forEach((el) => {
                !unique.includes(el.className) && unique.push(el.className);
            });
            (0, loader_1.load)(rootElement, elements, root);
            const board = new board_1.Board(`div#container`, getLabeledPortsOptions());
            board.subjects.get().onPortHover.subscribe((event) => { });
            board.subjects.get().onPortClick.subscribe((event) => {
                console.log(`Click on: ${event}`);
                console.log(board.getPort(event));
                // console.log(board.getPortsProps());
                // console.log(board.getCompsProps());
            });
            board.subjects.get().onComponentHover.subscribe((event) => { });
            board.subjects.get().onSelectionChange.subscribe((event) => {
                console.log(`Selection: ${JSON.stringify(event)}`);
            });
            const filter = document.querySelector('input[id="filter"]');
            filter.addEventListener("keyup", () => {
                board.setFilter(filter.value.trim() === "" ? undefined : filter.value.trim());
                board.refresh();
            });
            filter.addEventListener("change", () => {
                // board.refresh();
            });
            const back = document.querySelector('span[id="back"]');
            back.addEventListener("click", () => {
                board.toPrevComposition();
            });
            const location = document.querySelector('span[id="location"]');
            board.subjects.get().onLocationChange.subscribe((locations) => {
                location.innerHTML = locations
                    .map((l) => l.sig.short_name)
                    .join("/");
            });
            const matches = document.querySelector('input[id="matches"]');
            matches.addEventListener("keyup", () => {
                board
                    .matches()
                    .set(matches.value.trim() === ""
                    ? undefined
                    : matches.value.trim());
            });
            document.querySelector('span[id="prev"]').addEventListener("click", () => {
                board.matches().prev();
            });
            document.querySelector('span[id="next"]').addEventListener("click", () => {
                board.matches().next();
            });
            const matches_state = document.querySelector('span[id="matches_state"]');
            board.subjects
                .get()
                .onMatches.subscribe((event) => {
                if (event === undefined) {
                    matches_state.innerHTML = "";
                }
                else {
                    matches_state.innerHTML = `${event.current}/${event.total} (${event.id})`;
                }
            });
            board.bind(root);
            board.render();
        });
    }, 200);
}
function dummy() {
    setTimeout(() => {
        const composition = (0, dummy_1.getDummyComposition)(10, 5, 2, undefined);
        const board = new board_1.Board(`div#container`, getLabeledPortsOptions());
        board.bind(composition);
        board.render();
        board.subjects.get().onPortHover.subscribe((event) => { });
        board.subjects.get().onComponentHover.subscribe((event) => { });
    }, 200);
}
real();
//# sourceMappingURL=main.js.map