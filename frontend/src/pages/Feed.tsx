import { Component, For, Show, onMount, createSignal, createMemo } from 'solid-js';
import { useNavigate } from '@solidjs/router';
import { messagesStore, fetchMessages, addMessage, updateMessage, deleteMessage } from '../stores/messagesStore';
import { authStore, logout } from '../stores/authStore';
import { showToast } from '../stores/uiStore';
import { MessageCard } from '../components/MessageCard';
import { MessageInput } from '../components/MessageInput';
import { ThemeToggle } from '../components/ThemeToggle';
import { LoadingSpinner } from '../components/LoadingSpinner';
import { EditModal } from '../components/EditModal';
import './Feed.css';

export const Feed: Component = () => {
    const navigate = useNavigate();
    const [isLoading, setIsLoading] = createSignal(true);
    const [selectedMessage, setSelectedMessage] = createSignal<string | null>(null);
    const [editingMessageId, setEditingMessageId] = createSignal<string | null>(null);
    const [isEditSaving, setIsEditSaving] = createSignal(false);

    // Get the content of the message being edited
    const editingMessageContent = createMemo(() => {
        const id = editingMessageId();
        if (!id) return '';
        const message = messagesStore.messages.find(m => m.id === id);
        return message?.content || '';
    });

    onMount(async () => {
        try {
            await fetchMessages();
        } catch (err) {
            showToast('Failed to load messages', 'error');
        } finally {
            setIsLoading(false);
        }
    });

    const handleSubmit = async (content: string) => {
        try {
            await addMessage(content);
            showToast('Message posted!', 'success');
        } catch (err) {
            showToast('Failed to post message', 'error');
        }
    };

    const handleDelete = async (id: string) => {
        if (!confirm('Delete this message?')) return;

        try {
            await deleteMessage(id);
            showToast('Message deleted', 'info');
        } catch (err) {
            showToast('Failed to delete message', 'error');
        }
    };

    const handleLogout = () => {
        logout();
        navigate('/login', { replace: true });
    };

    const handleEditSave = async (content: string) => {
        const id = editingMessageId();
        if (!id) return;

        setIsEditSaving(true);
        try {
            await updateMessage(id, content);
            showToast('Message updated!', 'success');
            setEditingMessageId(null);
        } catch (err) {
            showToast('Failed to update message', 'error');
        } finally {
            setIsEditSaving(false);
        }
    };

    return (
        <div class="feed-page">
            <header class="feed-header">
                <h1 class="feed-title">Dissipate</h1>
                <div class="feed-actions">
                    <button
                        class="header-button"
                        onClick={() => navigate('/settings')}
                        aria-label="Settings"
                        title="Settings"
                    >
                        ⚙️
                    </button>
                    <ThemeToggle />
                    <button
                        class="header-button"
                        onClick={handleLogout}
                        aria-label="Logout"
                        title="Logout"
                    >
                        🚪
                    </button>
                </div>
            </header>

            <main class="feed-main">
                <Show when={isLoading()}>
                    <div class="feed-loading">
                        <LoadingSpinner size="lg" />
                    </div>
                </Show>

                <Show when={!isLoading() && messagesStore.messages.length === 0}>
                    <div class="feed-empty">
                        <p>No messages yet.</p>
                        <p>Start typing below!</p>
                    </div>
                </Show>

                <Show when={!isLoading() && messagesStore.messages.length > 0}>
                    <div class="message-list">
                        <For each={messagesStore.messages}>
                            {(message) => (
                                <MessageCard
                                    message={message}
                                    onClick={() => navigate(`/post/${message.id}`)}
                                    onEdit={() => setEditingMessageId(message.id)}
                                    onDelete={() => handleDelete(message.id)}
                                />
                            )}
                        </For>
                    </div>
                </Show>
            </main>

            <MessageInput onSubmit={handleSubmit} disabled={messagesStore.isSyncing} />

            <EditModal
                isOpen={editingMessageId() !== null}
                initialContent={editingMessageContent()}
                onSave={handleEditSave}
                onClose={() => setEditingMessageId(null)}
                isLoading={isEditSaving()}
            />
        </div>
    );
};

export default Feed;
