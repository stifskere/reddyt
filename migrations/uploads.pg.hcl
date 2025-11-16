enum "upload_platform" {
  schema = schema.reddyt
  comment = "Upload platforms."

  values = [
    "LOCAL_PROVIDER",
    "YOUTUBE"
  ]
}

table "uploads" {
  schema = schema.reddyt
  comment = "The generated content URLs for runs."

  primary_key {
    columns = [column.id]
  }

  foreign_key "fk_uploads_run" {
    columns = [column.run_id]
    ref_columns = [table.runs.column.id]
    on_delete = CASCADE
    on_update = NO_ACTION
  }

  column "id" {
    type = serial
    null = false
  }

  column "run_id" {
    type = int
    null = false
    comment = "The run this upload belongs to."
  }

  column "platform" {
    type = enum.upload_platform
    null = false
    comment = "The platform to which the video was uploaded."
  }

  column "url" {
    type = varchar(512)
    null = false
    comment = "URL of the uploaded video, regardless of platform."
  }

  column "uploaded_at" {
    type = timestamp
    null = false
    default = "NOW()"
    comment = "Time when the upload occurred."
  }
}
