<script lang="ts">
    import { Config, Globals } from "./interfaces";
    import { MapData } from "./map_data";
    import PlayerDot from "./PlayerDot.svelte";

    let { globals, settings }: { globals: Globals; settings: Config } = $props();

    let map_info = $state(MapData[globals.data.map_name]);
</script>

<div class="radar">
    <img src={`/radars/${globals.data.map_name}.png`} alt={globals.data.map_name} />

    {#each globals.data.players as player}
        <PlayerDot {player} {map_info} />
    {/each}

    {#if settings.show_team}
        {#each globals.data.friendlies as player}
            <PlayerDot {player} {map_info} />
        {/each}
    {/if}
</div>
