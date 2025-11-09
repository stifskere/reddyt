
CREATE TABLE IF NOT EXISTS profile_overrides (
	id SERIAL NOT NULL,
	profile_id INTEGER REFERENCES profiles(id) NOT NULL, -- the profile this override is for
	runs_at TIMESTAMP NOT NULL, -- when should the profile run at
	claimed BOOLEAN NOT NULL DEFAULT false -- whether the profile is alraedy ran/claimed or not.
);
