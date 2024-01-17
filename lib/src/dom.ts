export function stop(event: Event) {
    event.stopImmediatePropagation();
    event.stopPropagation();
    event.preventDefault();
}
