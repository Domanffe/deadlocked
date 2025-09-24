<script lang="ts">
    import type { Config, Globals, Vec2 } from "./ts/interfaces";
    import { MapData } from "./ts/map_data";
    import PlayerDot from "./PlayerDot.svelte";

    let { globals, settings }: { globals: Globals; settings: Config } = $props();

    let map_info = $state(MapData[globals.data.map_name]);

    let scale = $state(1.0);
    let pan_position: Vec2 = $state({ x: 0.0, y: 0.0 });
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

<style>
    .radar {
        width: 100%;
        height: 100%;
    }

    .radar > img {
        width: 100%;
        height: 100%;
    }
</style>
