table "profiles" {
  schema = schema.reddyt
  comment = "The profile settings table."

  primary_key {
    columns = [column.id]
  }

  column "id" {
    type = integer
    null = false
  }

  column "name" {
    type = varchar(35)
    null = false
    comment = "A simple name identifier, no effect on the profile."
  }

  column "upload_schedule" {
    type = varchar(30)
    null = false
    default = "0 12 * * *"
    comment = "Used to populate the next_run field."
  }

  column "paused" {
    type = bool
    null = false
    default = false
    comment = "If true no run will be triggered regardlress of the upload schedule."
  }

  column "question_prompt" {
    type = varchar(4096)
    null = false
    default = ""
    comment = "Will be passed to the AI provider to generate a \"reddyt style question\"."
  }

  column "answer_prompt" {
    type = varchar(4096)
    null = false
    default = ""
    comment = "Will be passed to the AI provider to ideally answer the previously generated question."
  }

  column "background_glob" {
    type = varchar(255)
    null = false
    default = "*"
    comment = "A GLOB filtering the background paths in the storage provider."
  }

  column "voice_name" {
    type = varchar(255)
    null = false
    comment = "A voice identifier for the TTS provider to use."
  }

  column "font_name" {
    type = varchar(255)
    null = false
    comment = "A Google Fonts identifier to display captions."
  }
}
