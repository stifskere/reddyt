CREATE TYPE RUN_STATE AS ENUM (
	'PAUSED', -- in this state polling won't have effect on the profile.
	'IDLING', -- the profile is waiting to be picked up.
	'GENERATING_QUESTION', -- the google gemini API is being called to generate a question.
	'GENERATING_ANSWER', -- the google gemini API is being called to generate an answer.
	'RENDERING_VOICE', --  the TTS engine is generating a voice.
	'RENDERING_SUBTITLES', -- subtitles for the voice are being generated as well as synced.
	'DOWNLOADING_BACKGROUND', -- the background is being downloade from the storage provider.
	'COMPOSING_VIDEO', -- the video is being composed with FFMPEG.
	'UPLOADING' -- the video is being uploaded to youtube.
);

CREATE TABLE IF NOT EXISTS profiles (
	id SERIAL PRIMARY KEY,
	name VARCHAR(35) NOT NULL UNIQUE, -- a simple name identifier, no effect.
	youtube_oauth VARCHAR(255), -- the youtube oauth token.
	upload_schedule VARCHAR(30) NOT NULL DEFAULT '0 12 * * *', -- used to set the next_run filed.
	question_prompt VARCHAR(4099) NOT NULL DEFAULT '', -- a prompt to generate a reddyt style question.
	answer_prompt VARCHAR(4099) NOT NULL DEFAULT '', -- a prompt to generate a reddyt style answer.
	background_glob VARCHAR(255) NOT NULL DEFAULT '*', -- a filter to choose what background videos will be rendered.
	voice_name VARCHAR(255) NOT NULL, -- the name of the voice for the TTS engine.
	font_name VARCHAR(255) NOT NULL, -- a google fonts identifier for the captions.
	current_state RUN_STATE NOT NULL DEFAULT 'IDLING', -- the current running state.
	next_run TIMESTAMP -- if this is NULL run on next poll.
);
