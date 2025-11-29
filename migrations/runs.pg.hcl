table "runs" {
	schema = schema.reddyt
	comment = "A run history for scheduling, feedback and preservation."

	primary_key {
		columns = [column.id]
	}

	foreign_key "fk_runs_profile" {
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
		comment = "The profile this run belongs to."
	}

	column "error" {
		type = varchar(1024)
		null = true
		comment = "Debug information in case of error, otherwise null."
	}

	column "processing" {
		type = sql("VARCHAR(128)[]")
		null = false
		comment = "The layers currently being processed named as \"stage.layer\""
	}

	# This is used by the scheduler to know if a profile
	# scheduled time is already started.
	column "started_at" {
		type = date
		null = false
		default = "NOW()"
		comment = "When did the processing for this run started."
	}

	# If this is not defined the status will be "running", otherwise
	# error or finished depending on the error column.
	column "finished_at" {
		type = date
		null = true
		comment = "When did the processing for this run end."
	}
}
