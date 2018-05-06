CREATE TABLE items (
	id SERIAL PRIMARY KEY,
	channel_id INTEGER NOT NULL REFERENCES channels (id),
	title VARCHAR NOT NULL,
	link VARCHAR NOT NULL,
	description VARCHAR NOT NULL,
	pub_date TIMESTAMP NOT NULL,
	guid VARCHAR
)
