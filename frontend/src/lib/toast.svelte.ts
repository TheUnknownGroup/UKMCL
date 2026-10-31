export type ToastKind = 'success' | 'error' | 'info';
export type ToastItem = { id: number; kind: ToastKind; message: string; paused: boolean };

export const toasts = $state<ToastItem[]>([]);
let nextId = 0;

type Timer = { handle: ReturnType<typeof setTimeout> | null; start: number; remaining: number };
const timers = new Map<number, Timer>();

function remove(id: number) {
     const i = toasts.findIndex((t) => t.id === id);
     if (i >= 0) toasts.splice(i, 1);
     timers.delete(id);
}

export function pushToast(kind: ToastKind, message: string, ms = 4000) {
     const id = nextId++;
     toasts.push({ id, kind, message, paused: false });
     timers.set(id, { handle: setTimeout(() => remove(id), ms), start: performance.now(), remaining: ms });
}

export function pauseToast(id: number) {
     const timer = timers.get(id);
     const toast = toasts.find((t) => t.id === id);
     if (!timer || !toast || timer.handle === null) return;
     clearTimeout(timer.handle);
     timer.handle = null;
     timer.remaining -= performance.now() - timer.start;
     toast.paused = true;
}

export function resumeToast(id: number) {
     const timer = timers.get(id);
     const toast = toasts.find((t) => t.id === id);
     if (!timer || !toast || timer.handle !== null) return;
     timer.start = performance.now();
     timer.handle = setTimeout(() => remove(id), timer.remaining);
     toast.paused = false;
}