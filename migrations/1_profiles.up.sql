CREATE TABLE IF NOT EXISTS profiles (
	id SERIAL PRIMARY KEY,
	name VARCHAR(35) NOT NULL UNIQUE, -- a simple name identifier, no effect.
	upload_schedule VARCHAR(30) NOT NULL DEFAULT '0 12 * * *', -- used to set the next_run filed.
	question_prompt VARCHAR(4099) NOT NULL DEFAULT '', -- a prompt to generate a reddyt style question.
	answer_prompt VARCHAR(4099) NOT NULL DEFAULT '', -- a prompt to generate a reddyt style answer.
	background_glob VARCHAR(255) NOT NULL DEFAULT '*', -- a filter to choose what background videos will be rendered.
	voice_name VARCHAR(255) NOT NULL, -- the name of the voice for the TTS engine.
	font_name VARCHAR(255) NOT NULL, -- a google fonts identifier for the captions.
);
