CREATE TABLE items_categories (
	id SERIAL PRIMARY KEY,
	item_id INTEGER NOT NULL REFERENCES items (id),
	category_id INTEGER NOT NULL REFERENCES categories (id)
)
