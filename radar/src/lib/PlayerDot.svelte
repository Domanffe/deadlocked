<script lang="ts">
    import type { MapInfo, PlayerData, Vec2 } from "./ts/interfaces";

    let { player, map_info }: { player: PlayerData; map_info: MapInfo } = $props();

    let radar_position = $derived({
        x: (player.position.x - map_info.x) / map_info.scale,
        y: -(player.position.y - map_info.y) / map_info.scale,
    });
</script>

<div class="player-dot">

</div>

<style>
    
</style>
