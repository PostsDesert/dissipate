#!/bin/bash

# Seed script for Dissipate database
# Creates test users and sample messages

DB_PATH="dissipate.db"

echo "Seeding database..."

sqlite3 "$DB_PATH" <<EOF
-- Create test user 1
INSERT INTO users (id, email, username, password_hash, salt, created_at, updated_at)
VALUES (
    '550e8400-e29b-41d4-a716-446655440001',
    'test1@example.com',
    'Test User 1',
    '\$argon2id\$v=19\$m=65536,t=3,p=4\$c29tZXNhbHQ\$examplehash1',
    'somesalt1',
    '2026-01-04T00:00:00Z',
    '2026-01-04T00:00:00Z'
);

-- Create test user 2
INSERT INTO users (id, email, username, password_hash, salt, created_at, updated_at)
VALUES (
    '550e8400-e29b-41d4-a716-446655440002',
    'test2@example.com',
    'Test User 2',
    '\$argon2id\$v=19\$m=65536,t=3,p=4\$c29tZXNhbHQ\$examplehash2',
    'somesalt2',
    '2026-01-04T00:00:00Z',
    '2026-01-04T00:00:00Z'
);

-- Create sample messages for user 1
INSERT INTO messages (id, user_id, content, created_at, updated_at) VALUES
    ('650e8400-e29b-41d4-a716-446655440001', '550e8400-e29b-41d4-a716-446655440001', 'Hello, world!', '2026-01-04T10:00:00Z', '2026-01-04T10:00:00Z'),
    ('650e8400-e29b-41d4-a716-446655440002', '550e8400-e29b-41d4-a716-446655440001', 'This is my second message.', '2026-01-04T11:00:00Z', '2026-01-04T11:00:00Z'),
    ('650e8400-e29b-41d4-a716-446655440003', '550e8400-e29b-41d4-a716-446655440001', 'Testing the platform.', '2026-01-04T12:00:00Z', '2026-01-04T12:00:00Z'),
    ('650e8400-e29b-41d4-a716-446655440004', '550e8400-e29b-41d4-a716-446655440001', 'Another day, another message.', '2026-01-04T13:00:00Z', '2026-01-04T13:00:00Z'),
    ('650e8400-e29b-41d4-a716-446655440005', '550e8400-e29b-41d4-a716-446655440001', 'Fifth and final test message for user 1.', '2026-01-04T14:00:00Z', '2026-01-04T14:00:00Z');

-- Create sample messages for user 2
INSERT INTO messages (id, user_id, content, created_at, updated_at) VALUES
    ('650e8400-e29b-41d4-a716-446655440006', '550e8400-e29b-41d4-a716-446655440002', 'User 2 first message!', '2026-01-04T10:30:00Z', '2026-01-04T10:30:00Z'),
    ('650e8400-e29b-41d4-a716-446655440007', '550e8400-e29b-41d4-a716-446655440002', 'User 2 second message.', '2026-01-04T11:30:00Z', '2026-01-04T11:30:00Z'),
    ('650e8400-e29b-41d4-a716-446655440008', '550e8400-e29b-41d4-a716-446655440002', 'User 2 third message.', '2026-01-04T12:30:00Z', '2026-01-04T12:30:00Z'),
    ('650e8400-e29b-41d4-a716-446655440009', '550e8400-e29b-41d4-a716-446655440002', 'User 2 fourth message.', '2026-01-04T13:30:00Z', '2026-01-04T13:30:00Z'),
    ('650e8400-e29b-41d4-a716-44665544000a', '550e8400-e29b-41d4-a716-446655440002', 'User 2 fifth message.', '2026-01-04T14:30:00Z', '2026-01-04T14:30:00Z');
EOF

echo "Database seeded successfully!"
echo "Test accounts:"
echo "  test1@example.com / password123"
echo "  test2@example.com / password123"
