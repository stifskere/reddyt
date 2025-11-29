enum "upload_platform_type" {
	schema = schema.reddyt
	comment = "The upload platforms implemented in the application."

	values = [
		"LOCAL",
		"YOUTUBE_SHORTS",
		"YOUTUBE_VIDEO" # May be extended with other providers in the future
	]
}

table "upload_platforms" {
	schema = schema.reddyt
	comment = "Upload platforms OAuth information for each profile."

	primary_key {
		columns = [column.id]
	}

	index "u_upload_platform_profile" {
		unique = true
		columns = [column.profile_id, column.platform]
		comment = "Unique platform per profile_id"
	}

	foreign_key "fk_upload_platforms_profile" {
		columns = [column.profile_id]
		ref_columns = [table.profiles.column.id]
		on_delete = CASCADE
	}

	column "id" {
		type = int
		null = false
	}

	column "profile_id" {
		type = int
		null = false
		comment = "The profile owning this OAuth information set."
	}

	column "platform" {
		type = enum.upload_platform_type
		null = false
		comment = "The platform this information belongs to."
	}

	column "oauth_refresh" {
		type = bytea
		null = true
		comment = "The refresh token for this OAuth set."
	}

	column "oauth_token" {
		type = bytea
		null = true
		comment = "The refresh token for this OAuth set."
	}
}

