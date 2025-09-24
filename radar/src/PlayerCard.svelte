<script lang="ts">
    import { PlayerData } from "./interfaces";

    let { player }: { player: PlayerData } = $props();
</script>

<div class="card" class:dead={player.health <= 0}>
    <p>{player.name}</p>
    <div class="bars">
        <div class="bar health-bar">
            <div class="bar-inner" style:width={`${player.health}%`}></div>
        </div>
        <div class="bar armor-bar">
            <div class="bar-inner" style:width={`${player.armor}%`}></div>
        </div>
    </div>
</div>

<style>
    .card {
        width: 100%;
        display: flex;
        flex-direction: column;
        align-items: start;
    }

    .dead {
        opacity: 0.5;
    }

    .bars {
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;
    }

    .bar {
        width: 40%;
        border-radius: 0.2rem;
        display: flex;
        flex-direction: row;
    }

    .bar-inner {
        height: 100%;
    }
</style>
