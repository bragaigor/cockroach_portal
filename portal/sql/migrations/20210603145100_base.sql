CREATE TABLE transformer (
    id UUID NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    title text NOT NULL,
    power integer NOT NULL
);
CREATE INDEX ON transformer (title);
