import { Person } from "core"

const BASE_URL = "http://localhost:8088"
const headers = { "Content-Type": "application/json" }

type RequestMethod = "POST" | "PUT" | "DELETE"

interface RequestBody {
	readonly id: string
	readonly name: string
	readonly ts: number
}

interface AppRequest {
	readonly body: string
	readonly headers: {}
	readonly method: RequestMethod
}

const buildRequest = (method: RequestMethod, body: Partial<RequestBody>): AppRequest => ({
	body: JSON.stringify(body),
	headers,
	method,
})

// TODO add error handling
export async function getPeople(): Promise<Person[]> {
	const response: Response = await fetch(`${BASE_URL}/people`)
	const people: Person[] = await response.json()

	return people
}

// TODO add error handling
export async function getPersonById(id: string): Promise<Person> {
	const reqBody: Partial<RequestBody> = { id }

	const request = buildRequest("POST", reqBody)
	const response = await fetch(`${BASE_URL}/people/${id}`, request)
	const person: Person = await response.json()

	return person
}

const statusReturnHelper = (rs: Response): string => {
	if (rs.ok) {
		console.log("operation successful 👍")
		return "ok"
	} else {
		console.log(`very virus: ${rs}`)
		return rs.statusText
	}
}

export async function createPerson(name: string): Promise<string> {
	const reqBody: Partial<RequestBody> = { name }

	const request = buildRequest("POST", reqBody)
	const response = await fetch(`${BASE_URL}/people`, request)

	return statusReturnHelper(response)
}

export async function updatePersonById(person: Person): Promise<string> {
	const { id, name } = person
	const reqBody: Partial<RequestBody> = { id, name, ts: Date.now() }

	const request = buildRequest("PUT", reqBody)
	const response = await fetch(`${BASE_URL}/people`, request)

	return statusReturnHelper(response)
}

export async function deletePersonById(id: string): Promise<string> {
	const reqBody: Partial<RequestBody> = { id }

	const request = buildRequest("DELETE", reqBody)
	const response = await fetch(`${BASE_URL}/people`, request)

	return statusReturnHelper(response)
}
