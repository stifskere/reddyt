table "profiles" {
	schema = schema.reddyt
	comment = "The video profiles owned by an account."

	primary_key {
		columns = [column.id]
	}

	foreign_key "fk_profiles_account" {
		columns = [column.account_id]
		ref_columns = [table.accounts.column.id]
		on_delete = CASCADE
	}

	index "u_project_name_account_id" {
		unique = true
		columns = [column.account_id, column.name]
		comment = "Unique index defining unique profile names for each account."
	}

	column "id" {
		type = serial
		null = false
	}

	column "account_id" {
		type = int
		null = false
		comment = "The account that owns this profile."
	}

	column "name" {
		type = varchar(255)
		null = false
		comment = "The profile human readable idenitifer."
	}

	column "description" {
		type = varchar(1024)
		null = true
		comment = "A human readable description for the profile."
	}

	column "schedule" {
		type = varchar(64)
		null = false
		comment = "A cron schedule to define when a video should be generated and uploaded."
	}

	column "paused" {
		type = bool
		null = false
		default = false
		comment = "Whether the schedule is paused and no videos should be generated."
	}

	column "ar_height" {
		type = int
		null = false
		comment = "The aspect ratio height for the video."
	}

	column "ar_width" {
		type = int
		null = false
		comment = "The aspect ratio width for the video."
	}
}
