import { createSignal, createEffect } from 'solid-js';
import type { Theme, Toast } from '../types';

// Theme state
const getInitialTheme = (): Theme => {
    if (typeof window === 'undefined') return 'auto';
    const stored = localStorage.getItem('theme');
    if (stored === 'light' || stored === 'dark' || stored === 'auto') {
        return stored;
    }
    return 'auto';
};

const [theme, setThemeSignal] = createSignal<Theme>(getInitialTheme());

// Online state
const [isOnline, setIsOnline] = createSignal<boolean>(
    typeof navigator !== 'undefined' ? navigator.onLine : true
);

// Toast notifications
const [toasts, setToasts] = createSignal<Toast[]>([]);

// Apply theme to document
function applyTheme(newTheme: Theme): void {
    if (typeof document === 'undefined') return;

    const root = document.documentElement;

    if (newTheme === 'auto') {
        root.removeAttribute('data-theme');
    } else {
        root.setAttribute('data-theme', newTheme);
    }
}

// Initialize theme on load
if (typeof window !== 'undefined') {
    applyTheme(theme());

    // Listen for online/offline events
    window.addEventListener('online', () => setIsOnline(true));
    window.addEventListener('offline', () => setIsOnline(false));
}

// Store object for reactive access
export const uiStore = {
    get theme() { return theme(); },
    get isOnline() { return isOnline(); },
    get toasts() { return toasts(); },
};

// Actions
export function setTheme(newTheme: Theme): void {
    setThemeSignal(newTheme);
    localStorage.setItem('theme', newTheme);
    applyTheme(newTheme);
}

export function cycleTheme(): void {
    const current = theme();
    const next: Theme = current === 'auto' ? 'light' : current === 'light' ? 'dark' : 'auto';
    setTheme(next);
}

let toastId = 0;

export function showToast(message: string, type: 'success' | 'error' | 'info' = 'info', duration = 3000): void {
    const id = `toast-${++toastId}`;
    const toast: Toast = { id, message, type, duration };

    setToasts((prev) => [...prev, toast]);

    if (duration > 0) {
        setTimeout(() => {
            dismissToast(id);
        }, duration);
    }
}

export function dismissToast(id: string): void {
    setToasts((prev) => prev.filter((t) => t.id !== id));
}
