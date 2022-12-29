CREATE EXTENSION "pgcrypto";

CREATE TABLE orders
(
    id        UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    create_by VARCHAR NOT NULL,
    signature TEXT    NOT NULL
);