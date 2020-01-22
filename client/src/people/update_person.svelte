<script lang="typescript">
    import { constants } from "../constants"
    import { updatePersonById } from "../utils/app_utils"

    const { UPDATE_RECORD } = constants
    const nil: null = null

    let id: string = ""
    let name: string = ""

    const handleClick = async () => {
        const ts = Date.now()
        const response = await updatePersonById({ id, name, ts })

        console.log(response)
    }
</script>

<h2>Update A Person By ID</h2>

<input bind:value={id} placeholder="enter existing ID" />
<input bind:value={name} placeholder="enter new name" />
<button on:click={handleClick}>{UPDATE_RECORD(name)}</button>
