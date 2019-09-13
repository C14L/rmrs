CREATE TABLE IF NOT EXISTS sr (
  id VARCHAR PRIMARY KEY, -- without the "t5_" part!
  name VARCHAR,
  created DATE NOT NULL, -- created_utc
  url VARCHAR, -- max_length=50; e.g. "/r/de"
  over18 BOOLEAN,
  lang VARCHAR, -- max_length=10; language
  title VARCHAR, -- max_length=100
  header_title VARCHAR, -- max_length=100
  display_name VARCHAR, -- primary name to index
  subreddit_type VARCHAR, -- "public", "restricted", or "private"
  subscribers INT, -- subreddit subscribers count
  subscribers_here INT -- subreddit subscribers with an account on reddmeet
)