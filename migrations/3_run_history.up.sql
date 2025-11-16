CREATE TYPE RUN_STATE AS ENUM (
	'ERROR', -- an error occurred while processing any of the following steps'.
	'GENERATING_QUESTION', -- the google gemini API is being called to generate a question.
	'GENERATING_ANSWER', -- the google gemini API is being called to generate an answer.
	'RENDERING_VOICE', --  the TTS engine is generating a voice.
	'RENDERING_SUBTITLES', -- subtitles for the voice are being generated as well as synced.
	'DOWNLOADING_BACKGROUND', -- the background is being downloaded from the storage provider.
	'COMPOSING_VIDEO', -- the video is being composed with FFMPEG.
	'UPLOADING' -- the video is being uploaded to youtube.
);

CREATE TABLE IF NOT EXISTS runs (
	id SERIAL NOT NULL,
	profile_id INTEGER REFERENCES profiles(id) NOT NULL, -- the profile this run is from.
	run_date TIMESTAMP NOT NULL DEFAULT NOW(), -- when this was run.
	current_state RUN_STATE NOT NULL, -- the run state of this current run.
	error VARCHAR(4099) DEFAULT NULL, -- if there is an error this will be populated and current_state set to ERROR
	local_video_path VARCHAR(255) DEFAULT NULL, -- if the generated video was copied the url is stored here.
	youtube_video_url VARCHAR(255) DEFAULT NULL -- if the video was uploaded the url is tored here.
);
