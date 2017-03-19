-- Your SQL goes here
CREATE TABLE users (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  email VARCHAR NOT NULL,
  password TEXT NOT NULL,
  fence_key UUID NOT NULL DEFAULT gen_random_uuid(),
  finished_setup BOOLEAN NOT NULL DEFAULT 'f',
  awesome BOOLEAN NOT NULL DEFAULT 'f'
)