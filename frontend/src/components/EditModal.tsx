import { Component, createSignal, createEffect, onMount, onCleanup, Show } from 'solid-js';
import './EditModal.css';

interface EditModalProps {
    isOpen: boolean;
    initialContent: string;
    onSave: (content: string) => void;
    onClose: () => void;
    isLoading?: boolean;
}

export const EditModal: Component<EditModalProps> = (props) => {
    let textareaRef: HTMLTextAreaElement | undefined;
    const [content, setContent] = createSignal('');

    // Sync content when modal opens or initialContent changes
    createEffect(() => {
        if (props.isOpen) {
            setContent(props.initialContent);
        }
    });

    // Focus textarea when modal opens
    createEffect(() => {
        if (props.isOpen && textareaRef) {
            textareaRef.focus();
            // Move cursor to end
            textareaRef.selectionStart = textareaRef.value.length;
            textareaRef.selectionEnd = textareaRef.value.length;
        }
    });

    // Handle escape key to close
    const handleKeyDown = (e: KeyboardEvent) => {
        if (e.key === 'Escape' && props.isOpen) {
            props.onClose();
        }
    };

    onMount(() => {
        document.addEventListener('keydown', handleKeyDown);
    });

    onCleanup(() => {
        document.removeEventListener('keydown', handleKeyDown);
    });

    const handleSave = () => {
        const trimmed = content().trim();
        if (trimmed && trimmed !== props.initialContent) {
            props.onSave(trimmed);
        } else {
            props.onClose();
        }
    };

    const handleBackdropClick = (e: MouseEvent) => {
        if (e.target === e.currentTarget) {
            props.onClose();
        }
    };

    const adjustHeight = () => {
        if (textareaRef) {
            textareaRef.style.height = 'auto';
            textareaRef.style.height = `${Math.min(textareaRef.scrollHeight, 300)}px`;
        }
    };

    createEffect(() => {
        content();
        adjustHeight();
    });

    return (
        <Show when={props.isOpen}>
            <div class="edit-modal-overlay" onClick={handleBackdropClick}>
                <div class="edit-modal" role="dialog" aria-modal="true" aria-labelledby="edit-modal-title">
                    <header class="edit-modal-header">
                        <h2 id="edit-modal-title" class="edit-modal-title">Edit Message</h2>
                        <button
                            class="edit-modal-close"
                            onClick={props.onClose}
                            aria-label="Close"
                            disabled={props.isLoading}
                        >
                            ✕
                        </button>
                    </header>

                    <div class="edit-modal-body">
                        <textarea
                            ref={textareaRef}
                            class="edit-modal-textarea"
                            value={content()}
                            onInput={(e) => setContent(e.currentTarget.value)}
                            disabled={props.isLoading}
                            rows={3}
                            aria-label="Message content"
                        />
                        <span class="edit-modal-char-count">{content().length} characters</span>
                    </div>

                    <footer class="edit-modal-footer">
                        <button
                            class="edit-modal-button edit-modal-button-cancel"
                            onClick={props.onClose}
                            disabled={props.isLoading}
                        >
                            Cancel
                        </button>
                        <button
                            class="edit-modal-button edit-modal-button-save"
                            onClick={handleSave}
                            disabled={!content().trim() || props.isLoading}
                        >
                            {props.isLoading ? 'Saving...' : 'Save'}
                        </button>
                    </footer>
                </div>
            </div>
        </Show>
    );
};
