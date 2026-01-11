import type { Component } from 'solid-js';
import './LoadingSpinner.css';

interface LoadingSpinnerProps {
    size?: 'sm' | 'md' | 'lg';
}

export const LoadingSpinner: Component<LoadingSpinnerProps> = (props) => {
    const size = () => props.size || 'md';

    return (
        <div class={`spinner spinner-${size()}`} role="status" aria-label="Loading">
            <span class="spinner-circle" />
        </div>
    );
};
