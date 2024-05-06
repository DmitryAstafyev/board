export function stop(event: Event) {
    event.preventDefault();
    event.stopImmediatePropagation();
    event.stopPropagation();
}
