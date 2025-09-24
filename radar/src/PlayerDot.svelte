<script lang="ts">
    import { MapInfo, PlayerData, Vec2 } from "./interfaces";

    let { player, map_info }: { player: PlayerData; map_info: MapInfo } = $props();

    function radarPosition(): Vec2 {
        return {
            x: (player.position.x - map_info.x) / map_info.scale,
            y: -(player.position.y - map_info.y) / map_info.scale,
        };
    }
</script>

<div></div>
