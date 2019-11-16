export async function getPeople() {
    let res = await fetch("http://localhost:8088/people")
    let json = await res.json()

    return json
}
