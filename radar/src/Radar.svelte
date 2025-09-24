<script lang="ts">
    import { Config, State } from "./interfaces";
    import PlayerDot from "./PlayerDot.svelte";

    export let state: State;
    export let settings: Config;
</script>

<div class="radar">
    <img src={`/radars/${state.data.map_name}.png`} alt={state.data.map_name} />

    {#each state.data.players as player}
        <PlayerDot {player} />
    {/each}

    {#if settings.show_team}
        {#each state.data.friendlies as player}
            <PlayerDot {player} />
        {/each}
    {/if}
</div>
