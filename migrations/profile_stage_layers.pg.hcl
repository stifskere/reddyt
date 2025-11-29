table "video_stage_layers" {
	schema = schema.reddyt
	comment = "A video stage dynamic composing layer."

	primary_key {
		columns = [column.id]
	}

	foreign_key "fk_stages_profile" {
		columns = [column.profile_id]
		ref_columns = [table.profile_stages.column.id]
		on_delete = CASCADE
	}

	index "u_order_video_stage" {
		unique = true
		columns = [column.video_stage_id, column.order]
		comment = "Unique layer order per video stage."
	}

	column "id" {
		type = int
		null = false
	}

	column "video_stage_id" {
		type = int
		null = false
		comment = "The video stage owning this layer."
	}

	column "order" {
		type = int
		null = false
		comment = "The layer physical display order."
	}

	# This is abstracted directly to layer processing,
	# the layer type is to be implemented in a resolver
	# wrapper.
	column "layer_data" {
		type = bytea
		null = false
		comment = "The data interpreted by the layer processing module."
	}
}

