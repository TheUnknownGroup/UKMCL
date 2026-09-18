let resolver = null;

export function confirmDialog(message) {
     return new Promise((resolve) => {
          resolver = { resolve, message };
          window.dispatchEvent(new CustomEvent('open-confirm', { detail: { message } }));
     });
}

export function resolve(result) {
     if (!resolver) return;
     resolver.resolve(result);
     resolver = null;
}