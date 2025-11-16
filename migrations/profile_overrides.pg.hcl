table "profile_overrides" {
  schema = schema.reddyt
  comment = "Overrides the scheduled run time of a profile, allowing runs at specific times."

  primary_key {
    columns = [column.id]
  }

  foreign_key "fk_profile_overrides_profile" {
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
    comment = "What profile schedule is this overriding."
  }

  column "runs_at" {
    type = timestamp
    null = false
    comment = "The specific time when this profile should run."
  }

  column "claimed" {
    type = boolean
    null = false
    default = false
    comment = "Whether this run has been claimed/executed."
  }
}
