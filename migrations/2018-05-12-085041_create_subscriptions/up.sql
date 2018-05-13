CREATE TABLE subscriptions (
	id SERIAL PRIMARY KEY,
	user_id INTEGER NOT NULL REFERENCES users (id),
	channel_id INTEGER NOT NULL REFERENCES channels (id)
)

