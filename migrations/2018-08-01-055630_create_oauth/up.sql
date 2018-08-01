CREATE TABLE clients (
  id SERIAL PRIMARY KEY,
  name VARCHAR(256) NOT NULL,
  url VARCHAR(256),
  description TEXT,
  icon TEXT,
  user_id INTEGER NOT NULL,
  identifier VARCHAR(256) NOT NULL,
  secret VARCHAR(256) NOT NULL,
  response_type VARCHAR(64) NOT NULL,
  CONSTRAINT clients__unique_identifier
    UNIQUE (identifier),
  CONSTRAINT clients__user_id
    FOREIGN KEY (user_id)
    REFERENCES users (id)
);

CREATE TABLE grant_types (
  id SERIAL PRIMARY KEY,
  name VARCHAR(32) NOT NULL,
  CONSTRAINT grant_types__unique_name
    UNIQUE (name)
);

CREATE TABLE client_redirect_uris (
  id SERIAL PRIMARY KEY,
  client_id INTEGER NOT NULL,
  redirect_uri VARCHAR(128) NOT NULL,
  CONSTRAINT client_redirect_uris__client_id
    FOREIGN KEY (client_id)
    REFERENCES clients (id)
);

CREATE TABLE access_tokens (
  id SERIAL PRIMARY KEY,
  token VARCHAR(256) NOT NULL,
  client_id INTEGER NOT NULL,
  user_id INTEGER NOT NULL,
  grant_id INTEGER NOT NULL,
  scope VARCHAR(255) NOT NULL,
  issued_at TIMESTAMP WITH TIME ZONE NOT NULL,
  expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
  CONSTRAINT access_tokens__client_id
    FOREIGN KEY (client_id)
    REFERENCES clients (id),
  CONSTRAINT access_tokens__grant_id
    FOREIGN KEY (grant_id)
    REFERENCES grant_types (id),
  CONSTRAINT access_tokens__unique_token
    UNIQUE(token),
  CONSTRAINT access_tokens__user_id
    FOREIGN KEY (user_id)
    REFERENCES users (id)
);

CREATE TABLE refresh_tokens (
  id SERIAL PRIMARY KEY,
  token VARCHAR(256) NOT NULL,
  client_id INTEGER NOT NULL,
  access_token_id INTEGER NOT NULL,
  scope VARCHAR(255) NOT NULL,
  issued_at TIMESTAMP WITH TIME ZONE NOT NULL,
  CONSTRAINT refresh_tokens__client_id
    FOREIGN KEY (client_id)
    REFERENCES clients (id),
  CONSTRAINT refresh_tokens__access_token_id
    FOREIGN KEY (access_token_id)
    REFERENCES access_tokens (id),
  CONSTRAINT refresh_tokens__token
    UNIQUE(token)
);

CREATE TABLE auth_codes (
  id SERIAL PRIMARY KEY,
  client_id INTEGER NOT NULL,
  name VARCHAR(64) NOT NULL,
  scope VARCHAR(255) NOT NULL,
  expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
  redirect_uri VARCHAR(128) NOT NULL,
  user_id INTEGER,
  CONSTRAINT auth_codes__client_id
    FOREIGN KEY (client_id)
    REFERENCES clients (id),
  CONSTRAINT auth_codes__user_id
    FOREIGN KEY (user_id)
    REFERENCES users (id)
);

INSERT INTO grant_types (name) VALUES
  ('authorization_code'),
  ('token'),
  ('password'),
  ('client_credentials'),
  ('refresh_token');

CREATE INDEX access_tokens_token_index ON access_tokens (token);
CREATE INDEX refresh_tokens_token_index ON refresh_tokens (token);
