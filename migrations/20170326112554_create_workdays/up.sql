-- Your SQL goes here
CREATE TABLE workdays (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  fence UUID NOT NULL,
  date DATE NOT NULL DEFAULT CURRENT_DATE,
  stamps UUID[] NOT NULL
)