import { describe, expect, test, vi, beforeEach } from 'vitest';
import { createRoot } from 'solid-js';
import type { User } from '../types';
import { localStorageMock } from '../test/setup';

// Mock the API module
vi.mock('../services/api', () => ({
    api: {
        login: vi.fn(),
    },
}));

// Now import the store and API after mocks are set up
import { authStore, setToken, setUser, login, logout, isAuthenticated } from './authStore';
import { api } from '../services/api';

describe('Auth Store', () => {
    beforeEach(() => {
        localStorageMock.clear();
        vi.clearAllMocks();
        // Reset store state
        setToken(null);
        setUser(null);
    });

    test('initial state after reset should be unauthenticated', () => {
        expect(isAuthenticated()).toBe(false);
    });

    test('setToken should update token and persist to localStorage', () => {
        setToken('test-token');
        expect(authStore.token).toBe('test-token');
        expect(localStorageMock.getItem('token')).toBe('test-token');
    });

    test('setUser should update user state', () => {
        const user: User = {
            id: '123',
            email: 'test@example.com',
            username: 'testuser',
            created_at: '2026-01-01T00:00:00Z',
            updated_at: '2026-01-01T00:00:00Z',
        };
        setUser(user);
        expect(authStore.user).toEqual(user);
    });

    test('login should call API and update state', async () => {
        const mockResponse = {
            token: 'jwt-token',
            user: {
                id: '123',
                email: 'test@example.com',
                username: 'testuser',
                created_at: '2026-01-01T00:00:00Z',
                updated_at: '2026-01-01T00:00:00Z',
            },
        };

        vi.mocked(api.login).mockResolvedValueOnce(mockResponse);

        await login('test@example.com', 'password123');

        expect(api.login).toHaveBeenCalledWith('test@example.com', 'password123');
        expect(authStore.token).toBe('jwt-token');
        expect(authStore.user?.email).toBe('test@example.com');
        expect(isAuthenticated()).toBe(true);
    });

    test('logout should clear token and user', () => {
        setToken('test-token');
        setUser({
            id: '123',
            email: 'test@example.com',
            username: 'testuser',
            created_at: '2026-01-01T00:00:00Z',
            updated_at: '2026-01-01T00:00:00Z',
        });

        logout();

        expect(authStore.token).toBeNull();
        expect(authStore.user).toBeNull();
        expect(isAuthenticated()).toBe(false);
        expect(localStorageMock.getItem('token')).toBeNull();
    });
});
