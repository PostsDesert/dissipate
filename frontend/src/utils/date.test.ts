import { describe, expect, test } from 'vitest';
import { formatDate, formatRelativeTime, nowISO, isWithinMinutes } from './date';

describe('Date Utilities', () => {
    describe('formatDate', () => {
        test('should format ISO date to readable local format', () => {
            // Note: actual output depends on local timezone
            const result = formatDate('2026-01-04T17:30:00Z');
            expect(result).toMatch(/Jan 4, 2026/);
            expect(result).toMatch(/\d{1,2}:\d{2} (AM|PM)/);
        });
    });

    describe('formatRelativeTime', () => {
        test('should format date as relative time', () => {
            const now = new Date();
            const fiveMinutesAgo = new Date(now.getTime() - 5 * 60 * 1000);
            const result = formatRelativeTime(fiveMinutesAgo.toISOString());
            expect(result).toMatch(/minutes? ago/);
        });
    });

    describe('nowISO', () => {
        test('should return current time as ISO string', () => {
            const result = nowISO();
            expect(result).toMatch(/^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}/);
        });
    });

    describe('isWithinMinutes', () => {
        test('should return true for dates within the threshold', () => {
            const now = new Date();
            const twoMinutesAgo = new Date(now.getTime() - 2 * 60 * 1000);
            expect(isWithinMinutes(twoMinutesAgo.toISOString(), 5)).toBe(true);
        });

        test('should return false for dates outside the threshold', () => {
            const now = new Date();
            const tenMinutesAgo = new Date(now.getTime() - 10 * 60 * 1000);
            expect(isWithinMinutes(tenMinutesAgo.toISOString(), 5)).toBe(false);
        });
    });
});
