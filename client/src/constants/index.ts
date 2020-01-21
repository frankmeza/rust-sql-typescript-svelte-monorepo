const ADD_RECORD = (name: string) => {
	return name.length > 0 ? `add record for ${name}` : "add record"
}

const UPDATE_RECORD = (id: string) => {
	return id.length > 0 ? `update record for ${id}` : "update record"
}

const DELETE_RECORD = (id: string) => {
	return id.length > 0 ? `delete record for ${id}` : "delete record"
}

export const constants = {
    ADD_RECORD,
    UPDATE_RECORD,
    DELETE_RECORD,
	PEOPLE_DATABASE: "People Database",
	BUTTON_TEXT: "GET /people",
	RESET: "reset client",
}
