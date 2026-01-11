import { format, formatDistanceToNow, parseISO } from 'date-fns';

/**
 * Format a date string to local time display
 * Example: "Jan 4, 2026 5:30 PM"
 */
export function formatDate(dateString: string): string {
    const date = parseISO(dateString);
    return format(date, 'MMM d, yyyy h:mm a');
}

/**
 * Format a date as relative time
 * Example: "5 minutes ago", "2 hours ago"
 */
export function formatRelativeTime(dateString: string): string {
    const date = parseISO(dateString);
    return formatDistanceToNow(date, { addSuffix: true });
}

/**
 * Get current time as ISO string
 */
export function nowISO(): string {
    return new Date().toISOString();
}

/**
 * Check if a date is within the last N minutes
 */
export function isWithinMinutes(dateString: string, minutes: number): boolean {
    const date = parseISO(dateString);
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    return diffMs <= minutes * 60 * 1000;
}
