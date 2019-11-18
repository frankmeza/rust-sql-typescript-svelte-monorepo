import { Person } from "core"

const BASE_URL = "http://localhost:8088"
const headers = { "Content-Type": "application/json" }

type RequestMethod = "POST" | "PUT" | "DELETE"
type RequestBody = {}

const buildRequest = (method: RequestMethod, body: RequestBody) => ({
	body: JSON.stringify(body),
	headers,
	method,
})

export async function getPeople(): Promise<Person[]> {
	const response = await fetch(`${BASE_URL}/people`)
	const people: Person[] = await response.json()

	return people
}

export async function getPersonById(id: string): Promise<Person> {
	const reqBody = { id }
	const request = buildRequest("POST", reqBody)

	const response = await fetch(`${BASE_URL}/people/${id}`, request)
	const person: Person = await response.json()

	return person
}

export async function updatePersonById(person: Person): Promise<string | null> {
	const { id, name } = person
	const reqBody = JSON.stringify({ id, name })

	const request = buildRequest("PUT", reqBody)
	const response = await fetch(`${BASE_URL}/people`, request)

	return response.ok ? null : response.statusText
}

export async function deletePersonById(id: string): Promise<string | null> {
	const reqBody = { id }
	const request = buildRequest("DELETE", reqBody)
	const response = await fetch(`${BASE_URL}/people`, request)

	return response.ok ? null : response.statusText
}
