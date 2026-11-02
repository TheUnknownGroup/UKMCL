let resolver: ((result: boolean) => void) | null = null;

export function confirmDialog(message: string) {
     resolver?.(false);
     return new Promise<boolean>((re) => {
          resolver = re;
          window.dispatchEvent(new CustomEvent('open-confirm', { detail: { message }}))
     })
}

export function resolve(result: any) {
     if (!resolver) return;
     resolver(result);
     resolver = null;
}