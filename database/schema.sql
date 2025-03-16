-- Database: User and Tasks

CREATE TABLE users (
    id UUID PRIMARY KEY NOT NULL, 
    username VARCHAR(255) UNIQUE NOT NULL,
    password_hashed TEXT NOT NULL
);

CREATE TABLE tasks (
    id UUID PRIMARY KEY NOT NULL, 
    user_id UUID REFERENCES users(id) ON DELETE CASCADE, -- for owner of the task
    task_title VARCHAR(255) NOT NULL,
    task_description TEXT,
    creation_time TIMESTAMP DEFAULT NOW(),
    task_status VARCHAR(50) CHECK (task_status IN ('in progress', 'completed')) NOT NULL DEFAULT 'in progress',
    completion_time TIMESTAMP
);