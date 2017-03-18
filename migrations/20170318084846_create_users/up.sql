-- Your SQL goes here
CREATE TABLE users (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  email VARCHAR NOT NULL,
  password TEXT NOT NULL,
  awesome BOOLEAN NOT NULL DEFAULT 'f'
)