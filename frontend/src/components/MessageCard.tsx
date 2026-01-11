import { Component, Show, createMemo } from 'solid-js';
import type { Message } from '../types';
import { formatDate, formatRelativeTime, isWithinMinutes } from '../utils/date';
import './MessageCard.css';

interface MessageCardProps {
    message: Message;
    onClick?: () => void;
    onEdit?: () => void;
    onDelete?: () => void;
}

const PREVIEW_LENGTH = 200;

export const MessageCard: Component<MessageCardProps> = (props) => {
    const preview = createMemo(() => {
        const content = props.message.content;
        if (content.length <= PREVIEW_LENGTH) {
            return { text: content, truncated: false, remaining: 0 };
        }
        return {
            text: content.slice(0, PREVIEW_LENGTH),
            truncated: true,
            remaining: content.length - PREVIEW_LENGTH,
        };
    });

    const timestamp = createMemo(() => {
        // Show relative time if within last 24 hours, otherwise full date
        if (isWithinMinutes(props.message.created_at, 24 * 60)) {
            return formatRelativeTime(props.message.created_at);
        }
        return formatDate(props.message.created_at);
    });

    const handleEdit = (e: Event) => {
        e.stopPropagation();
        props.onEdit?.();
    };

    const handleDelete = (e: Event) => {
        e.stopPropagation();
        props.onDelete?.();
    };

    return (
        <article
            class="message-card"
            onClick={props.onClick}
            role="button"
            tabIndex={0}
            onKeyDown={(e) => e.key === 'Enter' && props.onClick?.()}
        >
            <div class="message-content">
                <p class="message-text">
                    {preview().text}
                    <Show when={preview().truncated}>
                        <span class="message-truncated">
                            ... <span class="message-remaining">+{preview().remaining} chars</span>
                        </span>
                    </Show>
                </p>
            </div>

            <div class="message-footer">
                <time class="message-timestamp" datetime={props.message.created_at}>
                    {timestamp()}
                </time>

                <div class="message-actions">
                    <button
                        class="message-action"
                        onClick={handleEdit}
                        aria-label="Edit message"
                        title="Edit"
                    >
                        ✏️
                    </button>
                    <button
                        class="message-action message-action-delete"
                        onClick={handleDelete}
                        aria-label="Delete message"
                        title="Delete"
                    >
                        🗑️
                    </button>
                </div>
            </div>
        </article>
    );
};
