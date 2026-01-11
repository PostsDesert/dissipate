import { createSignal } from 'solid-js';
import { api } from '../services/api';
import type { User } from '../types';

// Auth state
const [token, setTokenSignal] = createSignal<string | null>(
    typeof window !== 'undefined' ? localStorage.getItem('token') : null
);
const [user, setUserSignal] = createSignal<User | null>(null);

// Derived state
export function isAuthenticated(): boolean {
    return token() !== null;
}

// Store object for reactive access
export const authStore = {
    get token() { return token(); },
    get user() { return user(); },
};

// Actions
export function setToken(newToken: string | null): void {
    setTokenSignal(newToken);
    if (newToken) {
        localStorage.setItem('token', newToken);
    } else {
        localStorage.removeItem('token');
    }
}

export function setUser(newUser: User | null): void {
    setUserSignal(newUser);
}

export async function login(email: string, password: string): Promise<void> {
    const response = await api.login(email, password);
    setToken(response.token);
    setUser(response.user);
}

export function logout(): void {
    setToken(null);
    setUser(null);
}
