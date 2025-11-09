
CREATE TABLE IF NOT EXISTS run_history (
	id SERIAL NOT NULL,
	profile_id INTEGER REFERENCES profiles(id) NOT NULL, -- the profile this run was from.
	run_date TIMESTAMP NOT NULL DEFAULT NOW(), -- when this was run.
	error VARCHAR(4099) DEFAULT NULL, -- if this is null success is inferred, otherwise the profile should be set to paused.
	local_video_path VARCHAR(255) DEFAULT NULL, -- if the generated video was copied the url is stored here.
	youtube_video_url VARCHAR(255) DEFAULT NULL -- if the video was uploaded the url is tored here.
)
