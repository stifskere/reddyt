enum "run_state" {
  schema = schema.reddyt
  comment = "Represents the current processing state of a run."

  values = [
    "ERROR",
    "IDLING",
    "GENERATING_QUESTION",
    "GENERATING_ANSWER",
    "RENDERING_VOICE",
    "RENDERING_SUBTITLES",
    "DOWNLOADING_BACKGROUND",
    "COMPOSING_VIDEO",
    "UPLOADING",
    "DONE"
  ]
}

table "runs" {
  schema = schema.reddyt
  comment = "The runs associated with profiles."

  primary_key {
    columns = [column.id]
  }

  foreign_key "fk_runs_profile" {
    columns = [column.profile_id]
    ref_columns = [table.profiles.column.id]
    on_delete = CASCADE
  }

  column "id" {
    type = serial
    null = false
  }

  column "profile_id" {
    type = int
    null = false
    comment = "What profile is this run processing."
  }

  column "run_date" {
    type = timestamp
    null = false
    default = "NOW()"
    comment = "At what time this run started."
  }

  column "current_state" {
    type = enum.run_state
    null = false
    default = "IDLING"
    comment = "The current state of this run."
  }

  column "error" {
    type = varchar(4096)
    null = true
    comment = "Contains additional information if current_state is ERROR."
  }
}
