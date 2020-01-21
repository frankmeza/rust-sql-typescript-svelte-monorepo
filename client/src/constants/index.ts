const ADD_RECORD = (name: string) => {
	return name.length > 0 ? `add record for ${name}` : "add record"
}

export const constants = {
	ADD_RECORD,
	PEOPLE_DATABASE: "People Database",
	BUTTON_TEXT: "GET /people",
	RESET: "reset client",
}
