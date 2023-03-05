CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    complete BOOLEAN NOT NULL DEFAULT FALSE,
    -- created_at will never change during the lifecycle of a task
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    -- updated_at will change every time the task is, such as toggle complete
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);