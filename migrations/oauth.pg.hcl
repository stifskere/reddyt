enum "oauth_type" {
  schema = schema.reddyt
  comment = "OAuth provider type for profile connections."

  values = [
    "YOUTUBE" # May be extended with other providers in the future
  ]
}

table "profile_oauth" {
  schema = schema.reddyt
  comment = "An OAuth connection for a profile."

  primary_key {
    columns = [column.id]
  }

  foreign_key "fk_profile_oauth_profile" {
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
    comment = "The profile this OAuth entry belongs to."
  }

  column "oauth_type" {
    type = enum.oauth_type
    null = false
    comment = "Type of OAuth provider."
  }

  column "refresh_token" {
    type = varchar(512)
    null = true
    comment = "Refresh token for the OAuth provider; nullable depending on platform requirements."
  }

  column "auth_token" {
    type = varchar(512)
    null = true
    comment = "Access token for the OAuth provider; nullable depending on platform requirements."
  }
}
