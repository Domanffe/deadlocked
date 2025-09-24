<script lang="ts">
    import type { Config, Globals, Vec2 } from "./ts/interfaces";
    import { MapData } from "./ts/map_data";
    import PlayerDot from "./PlayerDot.svelte";

    let { globals, settings }: { globals: Globals; settings: Config } = $props();

    let map_info = $state(MapData[globals.data.map_name]);

    let scale = $state(1.0);
    let pan_position: Vec2 = $state({ x: 0.0, y: 0.0 });
    let is_panning = $state(false);
    let last_mouse_pos: Vec2 = { x: 0, y: 0 };

    function handleWheel(event: WheelEvent) {
        event.preventDefault();

        const rect = (event.currentTarget! as HTMLDivElement).getBoundingClientRect();
        const mouse_x = event.clientX - rect.left;
        const mouse_y = event.clientY - rect.top;

        const zoom_factor = event.deltaY > 0 ? 0.97 : 1.03;
        const new_scale = Math.max(1.0, Math.min(5, scale * zoom_factor));

        const rel_x = (mouse_x - pan_position.x) / scale;
        const rel_y = (mouse_y - pan_position.y) / scale;

        scale = new_scale;

        //pan_position.x = mouse_x - rel_x * scale;
        //pan_position.y = mouse_y - rel_y * scale;
    }

    function handleMouseDown(event: MouseEvent) {
        if (event.button !== 0) return;

        is_panning = true;
        last_mouse_pos.x = event.clientX;
        last_mouse_pos.y = event.clientY;

        event.preventDefault();
    }

    function handleMouseMove(event: MouseEvent) {
        if (!is_panning) return;

        const dx = event.clientX - last_mouse_pos.x;
        const dy = event.clientY - last_mouse_pos.y;

        pan_position.x += dx;
        pan_position.y += dy;

        last_mouse_pos.x = event.clientX;
        last_mouse_pos.y = event.clientY;

        event.preventDefault();
    }

    function handleMouseUp(event: MouseEvent) {
        if (event.button !== 0) return;

        is_panning = false;
        event.preventDefault();
    }

    function resetView() {
        scale = 1.0;
        pan_position = { x: 0, y: 0 };
    }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
    class="radar-container"
    class:panning={is_panning}
    onwheel={handleWheel}
    onmousedown={handleMouseDown}
    onmousemove={handleMouseMove}
    onmouseup={handleMouseUp}
>
    <div class="radar" style="transform: translate({pan_position.x}px, {pan_position.y}px) scale({scale});">
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
</div>

<style>
    .radar-container {
        width: 100%;
        height: 100%;
        position: relative;
        overflow: hidden;
        border-radius: 1rem;
        border: 2px solid var(--color-text);
        background-color: var(--color-base);
        cursor: grab;
    }

    .radar-container.panning {
        cursor: grabbing;
    }

    .radar {
        width: 100%;
        height: 100%;
        max-height: 100%;
    }

    .radar > img {
        width: 100%;
        height: 100%;
    }
</style>
