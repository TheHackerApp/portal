CREATE FUNCTION set_updated_at_timestamp()
    RETURNS TRIGGER AS
$$
BEGIN
    new.updated_at = now();
    RETURN new;
END;
$$ LANGUAGE 'plpgsql';
