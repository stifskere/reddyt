
CREATE TYPE OAUTH_TYPE AS ENUM ( -- this may be extended in a future.
	'YOUTUBE'
);

CREATE TABLE IF NOT EXISTS profile_oauth (
	id SERIAL NOT NULL,
	profile_id INTEGER REFERENCES profiles(id) NOT NULL,
	oauth_type OAUTH_TYPE NOT NULL,
	refresh_token VARCHAR(512), -- this is nullable depending on platform needs.
	auth_token VARCHAR(512) -- this is nullable depending on platform needs.
);
