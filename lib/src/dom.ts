const MIN_DEVICE_PIXEL_RATIO = 2;
export const DEVICE_PIXEL_RATIO: number =
    window === undefined
        ? MIN_DEVICE_PIXEL_RATIO
        : typeof window.devicePixelRatio === "number"
        ? Math.ceil(window.devicePixelRatio) >= MIN_DEVICE_PIXEL_RATIO
            ? Math.ceil(window.devicePixelRatio)
            : MIN_DEVICE_PIXEL_RATIO
        : MIN_DEVICE_PIXEL_RATIO;

export function stop(event: Event) {
    event.preventDefault();
    event.stopImmediatePropagation();
    event.stopPropagation();
}
