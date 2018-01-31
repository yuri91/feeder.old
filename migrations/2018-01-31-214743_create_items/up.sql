CREATE TABLE items (
	id SERIAL PRIMARY KEY,
	channel_id INTEGER NOT NULL REFERENCES channels (id),
	title VARCHAR,
	link VARCHAR,
	description VARCHAR,
	author VARCHAR,
	guid VARCHAR,
	pub_date VARCHAR
)
