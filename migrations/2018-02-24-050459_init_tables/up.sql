-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY NOT NULL,
  email VARCHAR(255) NOT NULL,
  password TEXT NOT NULL,
  first_name VARCHAR(80) NOT NULL,
  date_joined TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  is_staff BOOLEAN NOT NULL DEFAULT FALSE,
  is_active BOOLEAN NOT NULL DEFAULT TRUE,
  avatar TEXT
);

CREATE TABLE tasks (
  id SERIAL PRIMARY KEY NOT NULL,
  parent_id INTEGER REFERENCES tasks (id),
  user_id INTEGER NOT NULL REFERENCES users (id),
  created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  modified TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  name TEXT NOT NULL,
  notes TEXT,
  completable BOOLEAN NOT NULL DEFAULT TRUE,
  completed BOOLEAN NOT NULL DEFAULT FALSE,
  start_date DATE,
  due_date DATE
);


CREATE INDEX task_parent_index ON tasks (parent_id);
CREATE INDEX task_user_index ON tasks (user_id);
CREATE INDEX task_completable_index ON tasks (completable);
CREATE INDEX task_completed_index ON tasks (completed);
CREATE INDEX task_start_date_index ON tasks (start_date);
CREATE INDEX task_due_date_index ON tasks (due_date);

CREATE UNIQUE INDEX user_email_index ON users (email);
CREATE INDEX user_staff_index ON users (is_staff);
CREATE INDEX user_active_index ON users (is_active);

CREATE OR REPLACE FUNCTION update_modified_column()   
RETURNS TRIGGER AS $$
BEGIN
    NEW.modified = now();
    RETURN NEW;   
END;
$$ language 'plpgsql';

CREATE TRIGGER update_tasks_modified BEFORE UPDATE ON tasks FOR EACH ROW EXECUTE PROCEDURE  update_modified_column();
