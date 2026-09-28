CREATE TYPE task_status AS ENUM ('todo', 'in_progress', 'done');

CREATE TABLE
    IF NOT EXISTS tasks (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        title VARCHAR(200) NOT NULL,
        description TEXT,
        status task_status NOT NULL DEFAULT 'todo',
        user_id UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
        created_at TIMESTAMP NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMP NOT NULL DEFAULT NOW ()
    );

-- Index pour retrouver rapidement les tâches d’un utilisateur
CREATE INDEX idx_tasks_user_id ON tasks (user_id);

CREATE INDEX idx_tasks_status ON tasks (status);