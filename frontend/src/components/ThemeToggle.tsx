import { Component } from 'solid-js';
import { uiStore, cycleTheme } from '../stores/uiStore';
import './ThemeToggle.css';

export const ThemeToggle: Component = () => {
    const getIcon = () => {
        switch (uiStore.theme) {
            case 'light': return '☀️';
            case 'dark': return '🌙';
            default: return '🌓';
        }
    };

    const getLabel = () => {
        switch (uiStore.theme) {
            case 'light': return 'Light mode';
            case 'dark': return 'Dark mode';
            default: return 'Auto mode';
        }
    };

    return (
        <button
            class="theme-toggle"
            onClick={cycleTheme}
            aria-label={`Current theme: ${getLabel()}. Click to change.`}
            title={getLabel()}
        >
            <span class="theme-icon">{getIcon()}</span>
        </button>
    );
};
