import { Component, For, Show } from 'solid-js';
import { uiStore, dismissToast } from '../stores/uiStore';
import type { Toast as ToastType } from '../types';
import './Toast.css';

export const ToastContainer: Component = () => {
    return (
        <div class="toast-container">
            <For each={uiStore.toasts}>
                {(toast) => <Toast toast={toast} />}
            </For>
        </div>
    );
};

interface ToastProps {
    toast: ToastType;
}

const Toast: Component<ToastProps> = (props) => {
    const handleDismiss = () => {
        dismissToast(props.toast.id);
    };

    return (
        <div
            class={`toast toast-${props.toast.type}`}
            role="alert"
            aria-live="polite"
        >
            <span class="toast-message">{props.toast.message}</span>
            <button
                class="toast-dismiss"
                onClick={handleDismiss}
                aria-label="Dismiss notification"
            >
                ×
            </button>
        </div>
    );
};

export { Toast };
