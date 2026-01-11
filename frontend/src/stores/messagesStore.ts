import { createSignal } from 'solid-js';
import { api } from '../services/api';
import type { Message } from '../types';

// Messages state
const [messages, setMessages] = createSignal<Message[]>([]);
const [isSyncing, setIsSyncing] = createSignal<boolean>(false);
const [lastSync, setLastSync] = createSignal<string | null>(null);

// Store object for reactive access
export const messagesStore = {
    get messages() { return messages(); },
    get isSyncing() { return isSyncing(); },
    get lastSync() { return lastSync(); },
};

// Sort messages by created_at descending (newest first)
function sortMessages(msgs: Message[]): Message[] {
    return [...msgs].sort((a, b) =>
        new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
    );
}

// Actions
export async function fetchMessages(since?: string): Promise<void> {
    setIsSyncing(true);
    try {
        const response = await api.getMessages(since);
        if (since) {
            // Merge new messages with existing ones
            setMessages((prev) => {
                const merged = [...prev];
                for (const msg of response.messages) {
                    const idx = merged.findIndex((m) => m.id === msg.id);
                    if (idx >= 0) {
                        merged[idx] = msg; // Update existing
                    } else {
                        merged.push(msg); // Add new
                    }
                }
                return sortMessages(merged);
            });
        } else {
            // Full fetch
            setMessages(sortMessages(response.messages));
        }
        setLastSync(new Date().toISOString());
    } finally {
        setIsSyncing(false);
    }
}

export async function addMessage(content: string, clientId?: string): Promise<Message> {
    const message = await api.createMessage(content, clientId);
    setMessages((prev) => sortMessages([message, ...prev]));
    return message;
}

export async function updateMessage(id: string, content: string): Promise<Message> {
    const message = await api.updateMessage(id, content);
    setMessages((prev) =>
        prev.map((m) => m.id === id ? message : m)
    );
    return message;
}

export async function deleteMessage(id: string): Promise<void> {
    await api.deleteMessage(id);
    setMessages((prev) => prev.filter((m) => m.id !== id));
}

// Optimistic update helpers (for offline support)
export function optimisticAdd(message: Message): void {
    setMessages((prev) => sortMessages([message, ...prev]));
}

export function optimisticUpdate(id: string, content: string): void {
    setMessages((prev) =>
        prev.map((m) => m.id === id ? { ...m, content, updated_at: new Date().toISOString() } : m)
    );
}

export function optimisticDelete(id: string): void {
    setMessages((prev) => prev.filter((m) => m.id !== id));
}

export function clearMessages(): void {
    setMessages([]);
    setLastSync(null);
}
