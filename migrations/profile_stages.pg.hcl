table "profile_stages" {
	schema = schema.reddyt
	comment = "The stages or \"dynamic tracks\" a video is composed of."
	
	primary_key {
		columns = [column.id]
	}

	foreign_key "fk_stages_profile" {
		columns = [column.profile_id]
		ref_columns = [table.profiles.column.id]
		on_delete = CASCADE
	}

	index "u_profile_stage_name_profile_id" {
		unique = true
		columns = [column.profile_id, column.name]
		comment = "Unique index defining unique profile stage names for each profile."
	}

	column "id" {
		type = int
		null = false
	}

	column "profile_id" {
		type = int
		null = false
		comment = "Which profile owns this stage."
	}

	column "name" {
		# This should be validated against unsafe characters.
		type = varchar(255)
		null = false
		comment = "An identifier for cross stage communication."
	}

	column "last_stage" {
		type = int
		null = true
		comment = "The last profile stage, if -1 its the first stage, if null its disconnected."
	}
}
