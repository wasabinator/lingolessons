ALTER TABLE setting
    RENAME COLUMN value TO text;
ALTER TABLE setting
    ADD COLUMN number NUMERIC;
