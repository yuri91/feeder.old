CREATE TABLE channels (
	id SERIAL PRIMARY KEY,
	title VARCHAR NOT NULL,
	link VARCHAR NOT NULL,
	description VARCHAR,
	language VARCHAR,
	copyright VARCHAR,
	pub_date VARCHAR,
	image VARCHAR,
	ttl INTEGER
)

