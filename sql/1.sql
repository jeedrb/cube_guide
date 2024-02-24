DROP TABLE cubes;
CREATE TABLE cubes (
	make VARCHAR(16),
    model VARCHAR(32),
    price DEC(4,2),
    magnets TINYINT,
    magnetic_core TINYINT,
    description VARCHAR(512)
);