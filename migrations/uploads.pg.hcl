table "uploads" {
	schema = schema.reddyt
	comment = "Stores what was uploaded by a specific run."

	primary_key {
		columns = [column.id]
	}

	foreign_key "fk_uploads_platform" {
		columns = [column.upload_platform_id]
		ref_columns = [table.upload_platforms.column.id]
		on_delete = NO_ACTION
	}

	foreign_key "fk_uploads_run" {
		columns = [column.run_id]
		ref_columns = [table.runs.column.id]
		on_delete = CASCADE
	}

	column "id" {
		type = int
		null = false
	}

	column "upload_platform_id" {
		type = int
		null = false
		comment = "Which upload platform in the profile was this uploaded to."
	}

	column "run_id" {
		type = int
		null = false
		comment = "Which run was responsible for this upload."
	}

	column "generated_url" {
		type = varchar(1024)
		null = false
		comment = "The URL returned by the service."
	}

	column "uploaded_at" {
		type = date
		null = false
		default = "NOW()"
		comment = "When was this uploaded at."
	}
}
