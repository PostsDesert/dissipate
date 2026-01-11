import { describe, expect, test, vi, beforeEach, afterEach } from 'vitest';
import { api, ApiError } from './api';
import type { LoginResponse, Message, MessagesResponse, SuccessResponse } from '../types';

// Mock fetch globally
const mockFetch = vi.fn();
global.fetch = mockFetch;

// Mock localStorage for jsdom
const localStorageMock = (() => {
    let store: Record<string, string> = {};
    return {
        getItem: (key: string) => store[key] ?? null,
        setItem: (key: string, value: string) => { store[key] = value; },
        removeItem: (key: string) => { delete store[key]; },
        clear: () => { store = {}; },
        get length() { return Object.keys(store).length; },
        key: (index: number) => Object.keys(store)[index] ?? null,
    };
})();

// Use Object.defineProperty to set localStorage on window
Object.defineProperty(window, 'localStorage', {
    value: localStorageMock,
    writable: true
});

describe('API Client', () => {
    beforeEach(() => {
        mockFetch.mockReset();
        localStorageMock.clear();
    });

    afterEach(() => {
        vi.restoreAllMocks();
    });

    describe('login', () => {
        test('should successfully login and return token and user', async () => {
            const mockResponse: LoginResponse = {
                token: 'test-jwt-token',
                user: {
                    id: '123',
                    email: 'test@example.com',
                    username: 'testuser',
                    created_at: '2026-01-01T00:00:00Z',
                    updated_at: '2026-01-01T00:00:00Z',
                },
            };

            mockFetch.mockResolvedValueOnce({
                ok: true,
                json: () => Promise.resolve(mockResponse),
            });

            const result = await api.login('test@example.com', 'password123');

            expect(mockFetch).toHaveBeenCalledWith(
                expect.stringContaining('/api/login'),
                expect.objectContaining({
                    method: 'POST',
                    headers: expect.objectContaining({
                        'Content-Type': 'application/json',
                    }),
                    body: JSON.stringify({ email: 'test@example.com', password: 'password123' }),
                })
            );
            expect(result).toEqual(mockResponse);
        });

        test('should throw ApiError on login failure', async () => {
            mockFetch.mockResolvedValueOnce({
                ok: false,
                status: 401,
                statusText: 'Unauthorized',
            });

            await expect(api.login('test@example.com', 'wrongpassword')).rejects.toThrow(ApiError);
        });
    });

    describe('getMessages', () => {
        test('should fetch messages with auth header', async () => {
            localStorageMock.setItem('token', 'test-token');

            const mockResponse: MessagesResponse = {
                messages: [
                    {
                        id: 'msg-1',
                        user_id: 'user-1',
                        content: 'Hello world',
                        created_at: '2026-01-01T00:00:00Z',
                        updated_at: '2026-01-01T00:00:00Z',
                    },
                ],
            };

            mockFetch.mockResolvedValueOnce({
                ok: true,
                json: () => Promise.resolve(mockResponse),
            });

            const result = await api.getMessages();

            expect(mockFetch).toHaveBeenCalledWith(
                expect.stringContaining('/api/messages'),
                expect.objectContaining({
                    headers: expect.objectContaining({
                        Authorization: 'Bearer test-token',
                    }),
                })
            );
            expect(result.messages).toHaveLength(1);
        });

        test('should include since parameter when provided', async () => {
            localStorageMock.setItem('token', 'test-token');

            mockFetch.mockResolvedValueOnce({
                ok: true,
                json: () => Promise.resolve({ messages: [] }),
            });

            await api.getMessages('2026-01-01T00:00:00Z');

            expect(mockFetch).toHaveBeenCalledWith(
                expect.stringContaining('since=2026-01-01T00%3A00%3A00Z'),
                expect.any(Object)
            );
        });
    });

    describe('createMessage', () => {
        test('should create a new message', async () => {
            localStorageMock.setItem('token', 'test-token');

            const mockMessage: Message = {
                id: 'new-msg-id',
                user_id: 'user-1',
                content: 'New message',
                created_at: '2026-01-01T00:00:00Z',
                updated_at: '2026-01-01T00:00:00Z',
            };

            mockFetch.mockResolvedValueOnce({
                ok: true,
                json: () => Promise.resolve(mockMessage),
            });

            const result = await api.createMessage('New message');

            expect(mockFetch).toHaveBeenCalledWith(
                expect.stringContaining('/api/messages'),
                expect.objectContaining({
                    method: 'POST',
                    body: JSON.stringify({ content: 'New message' }),
                })
            );
            expect(result).toEqual(mockMessage);
        });

        test('should include client-generated id when provided', async () => {
            localStorageMock.setItem('token', 'test-token');

            mockFetch.mockResolvedValueOnce({
                ok: true,
                json: () => Promise.resolve({ id: 'custom-id' }),
            });

            await api.createMessage('Test', 'custom-id');

            expect(mockFetch).toHaveBeenCalledWith(
                expect.any(String),
                expect.objectContaining({
                    body: JSON.stringify({ content: 'Test', id: 'custom-id' }),
                })
            );
        });
    });

    describe('updateMessage', () => {
        test('should update an existing message', async () => {
            localStorageMock.setItem('token', 'test-token');

            const mockMessage: Message = {
                id: 'msg-1',
                user_id: 'user-1',
                content: 'Updated content',
                created_at: '2026-01-01T00:00:00Z',
                updated_at: '2026-01-01T00:00:00Z',
            };

            mockFetch.mockResolvedValueOnce({
                ok: true,
                json: () => Promise.resolve(mockMessage),
            });

            const result = await api.updateMessage('msg-1', 'Updated content');

            expect(mockFetch).toHaveBeenCalledWith(
                expect.stringContaining('/api/messages/msg-1'),
                expect.objectContaining({
                    method: 'PUT',
                    body: JSON.stringify({ content: 'Updated content' }),
                })
            );
            expect(result.content).toBe('Updated content');
        });
    });

    describe('deleteMessage', () => {
        test('should delete a message', async () => {
            localStorageMock.setItem('token', 'test-token');

            const mockResponse: SuccessResponse = { success: true };

            mockFetch.mockResolvedValueOnce({
                ok: true,
                json: () => Promise.resolve(mockResponse),
            });

            const result = await api.deleteMessage('msg-1');

            expect(mockFetch).toHaveBeenCalledWith(
                expect.stringContaining('/api/messages/msg-1'),
                expect.objectContaining({
                    method: 'DELETE',
                })
            );
            expect(result.success).toBe(true);
        });
    });

    describe('user updates', () => {
        test('should update email', async () => {
            localStorageMock.setItem('token', 'test-token');

            mockFetch.mockResolvedValueOnce({
                ok: true,
                json: () => Promise.resolve({ success: true }),
            });

            const result = await api.updateEmail('new@example.com');

            expect(mockFetch).toHaveBeenCalledWith(
                expect.stringContaining('/api/user/email'),
                expect.objectContaining({
                    method: 'PUT',
                    body: JSON.stringify({ email: 'new@example.com' }),
                })
            );
            expect(result.success).toBe(true);
        });

        test('should update username', async () => {
            localStorageMock.setItem('token', 'test-token');

            mockFetch.mockResolvedValueOnce({
                ok: true,
                json: () => Promise.resolve({ success: true }),
            });

            const result = await api.updateUsername('newusername');

            expect(mockFetch).toHaveBeenCalledWith(
                expect.stringContaining('/api/user/username'),
                expect.objectContaining({
                    method: 'PUT',
                    body: JSON.stringify({ username: 'newusername' }),
                })
            );
            expect(result.success).toBe(true);
        });

        test('should update password', async () => {
            localStorageMock.setItem('token', 'test-token');

            mockFetch.mockResolvedValueOnce({
                ok: true,
                json: () => Promise.resolve({ success: true }),
            });

            const result = await api.updatePassword('oldpass', 'newpass');

            expect(mockFetch).toHaveBeenCalledWith(
                expect.stringContaining('/api/user/password'),
                expect.objectContaining({
                    method: 'PUT',
                    body: JSON.stringify({ current_password: 'oldpass', new_password: 'newpass' }),
                })
            );
            expect(result.success).toBe(true);
        });
    });
});
