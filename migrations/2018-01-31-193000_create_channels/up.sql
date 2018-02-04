CREATE TABLE channels (
	id SERIAL PRIMARY KEY,
	title VARCHAR NOT NULL,
	link VARCHAR NOT NULL,
	description VARCHAR NOT NULL,
	source VARCHAR NOT NULL,
	language VARCHAR,
	copyright VARCHAR,
	pub_date VARCHAR,
	image VARCHAR,
	ttl INTEGER
)

