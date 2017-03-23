-- Your SQL goes here
CREATE TABLE stamps (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  fence UUID NOT NULL,
  event VARCHAR NOT NULL,
  time TIMESTAMP NOT NULL DEFAULT current_timestamp
)