export const theme = $state({ light: typeof document !== 'undefined' && document.documentElement.classList.contains('light') });

export function initTheme() {
     theme.light = document.documentElement.classList.contains('light');
}

export function toggle() {
     theme.light = !theme.light;
     document.documentElement.classList.toggle('light', theme.light);
     try {
          localStorage.setItem('theme', theme.light ? 'light' : 'dark');
     } catch {}
}