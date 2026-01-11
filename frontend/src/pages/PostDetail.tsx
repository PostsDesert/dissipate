import { Component, Show, createMemo, onMount, createSignal } from 'solid-js';
import { useParams, useNavigate } from '@solidjs/router';
import { messagesStore, fetchMessages, deleteMessage, updateMessage } from '../stores/messagesStore';
import { formatDate, formatRelativeTime, isWithinMinutes } from '../utils/date';
import { LoadingSpinner } from '../components/LoadingSpinner';
import { EditModal } from '../components/EditModal';
import { showToast } from '../stores/uiStore';
import './PostDetail.css';

const PostDetail: Component = () => {
    const params = useParams();
    const navigate = useNavigate();
    const [isEditSaving, setIsEditSaving] = createSignal(false);
    const [isEditing, setIsEditing] = createSignal(false);

    // Try to find the message in the store
    const message = createMemo(() =>
        messagesStore.messages.find(m => m.id === params.id)
    );

    // Dynamic title (first 30 chars)
    const title = createMemo(() => {
        const msg = message();
        if (!msg) return 'Post';
        return msg.content.length > 30 ? msg.content.slice(0, 30) + '...' : msg.content;
    });

    // Timestamp logic
    const timestamp = createMemo(() => {
        const msg = message();
        if (!msg) return '';
        if (isWithinMinutes(msg.created_at, 24 * 60)) {
            return formatRelativeTime(msg.created_at);
        }
        return formatDate(msg.created_at);
    });

    // If message not found, try fetching (in case of direct link or refresh)
    onMount(async () => {
        if (!message() && messagesStore.messages.length === 0) {
            try {
                await fetchMessages();
            } catch (error) {
                console.error('Failed to load messages', error);
            }
        }
    });

    const handleBack = () => {
        navigate('/');
    };

    const handleDelete = async () => {
        const msg = message();
        if (!msg) return;
        if (!confirm('Delete this message?')) return;

        try {
            await deleteMessage(msg.id);
            showToast('Message deleted', 'info');
            navigate('/');
        } catch (err) {
            showToast('Failed to delete message', 'error');
        }
    };

    const handleEditSave = async (content: string) => {
        const msg = message();
        if (!msg) return;

        setIsEditSaving(true);
        try {
            await updateMessage(msg.id, content);
            showToast('Message updated!', 'success');
            setIsEditing(false);
        } catch (err) {
            showToast('Failed to update message', 'error');
        } finally {
            setIsEditSaving(false);
        }
    };

    return (
        <div class="post-detail-page">
            <header class="post-detail-header">
                <button
                    class="back-button"
                    onClick={handleBack}
                    aria-label="Back to Feed"
                >
                    ← Back
                </button>
                <h1 title={message()?.content}>{title()}</h1>
            </header>

            <main class="post-detail-main">
                <Show when={messagesStore.isSyncing && !message()}>
                    <div class="loading-container">
                        <LoadingSpinner size="lg" />
                    </div>
                </Show>

                <Show when={!messagesStore.isSyncing && !message()}>
                    <div class="error-container">
                        <p>Message not found.</p>
                        <button onClick={handleBack} class="secondary-button">Return to Feed</button>
                    </div>
                </Show>

                <Show when={message()}>
                    {(msg) => (
                        <>
                            <article class="full-post">
                                <div class="post-meta-header">
                                    <span class="post-date" title={formatDate(msg().created_at)}>{timestamp()}</span>
                                    <div class="post-actions">
                                        <button
                                            class="action-button edit-button"
                                            onClick={() => setIsEditing(true)}
                                            aria-label="Edit message"
                                            title="Edit"
                                        >
                                            ✏️ Edit
                                        </button>
                                        <button
                                            class="action-button delete-button"
                                            onClick={handleDelete}
                                            aria-label="Delete message"
                                            title="Delete"
                                        >
                                            🗑️ Delete
                                        </button>
                                    </div>
                                </div>
                                <div class="post-content">
                                    {msg().content}
                                </div>
                            </article>

                            <EditModal
                                isOpen={isEditing()}
                                initialContent={msg().content}
                                onSave={handleEditSave}
                                onClose={() => setIsEditing(false)}
                                isLoading={isEditSaving()}
                            />
                        </>
                    )}
                </Show>
            </main>
        </div>
    );
};

export default PostDetail;
