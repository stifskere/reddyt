# NOTE: This is likely to change when the cloud version is implemented
# as the cloud version will implement it's own OAUTH.
table "accounts" {
	schema = schema.reddyt
	comment = "User account information."

	primary_key {
		columns = [column.id]
	}

	index "u_account_email" {
		unique = true
		columns = [column.email]
		comment = "Unique account by email."
	}

	column "id" {
		type = serial
		null = false
	}

	column "email" {
		type = varchar(255)
		null = false
		comment = "The account associated email."
	}

	column "password" {
		type = bytea
		null = false
		comment = "The encrypted and salted account password."
	}

	column "password_salt" {
		type = VARCHAR(64)
		null = false
		comment = "SCRYPT password salt, 64 bytes in size per standard."
	}
}
