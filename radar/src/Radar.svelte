<script lang="ts">
    import { WSData } from "./data";

    export let state: { current_player: number; data: WSData };
</script>

<div class="radar">
    <img src={`/radars/${state.data.map_name}.png`} alt={state.data.map_name} />
</div>
