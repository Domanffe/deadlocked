<script lang="ts">
    import type { PlayerData } from "./ts/interfaces";

    let { player }: { player: PlayerData } = $props();

    const green = "var(--color-green)";
    const red = "var(--color-red)";
    const health = $derived(Math.max(0, Math.min(100, player.health)));
    let hp_bg = $derived(`color-mix(in hsl, ${green} ${health}%, ${red} ${100 - health}%)`);
</script>

<div class="card" class:dead={player.health <= 0}>
    <p>{player.name}</p>
    <div class="bars">
        <div class="bar health-bar">
            <div class="bar-inner" style:width={`${health}%`} style:background-color={hp_bg}></div>
            <div class="bar-number">{health}</div>
        </div>
        <div class="bar armor-bar">
            <div class="bar-inner" style:width={`${player.armor}%`} style:background-color="var(--color-blue)"></div>
            <div class="bar-number">{player.armor}</div>
        </div>
    </div>
</div>

<style>
    .card {
        width: 100%;
        display: flex;
        flex-direction: column;
        align-items: start;
        padding: 0.2rem 0.5rem;
        gap: 0.5rem;
        padding-bottom: 0.5rem;
        border-top: 2px solid var(--color-highlight);
    }

    .card > p {
        margin: 0;
        font-size: 1rem;
    }

    .dead {
        opacity: 0.5;
    }

    .bars {
        display: flex;
        flex-direction: row;
        justify-content: space-evenly;
        align-items: center;
        width: 100%;
        height: 1rem;
    }

    .bar {
        width: 40%;
        height: 100%;
        border-radius: 0.2rem;
        display: flex;
        flex-direction: row;
        background-color: var(--color-highlight);
        position: relative;
    }

    .bar-inner {
        height: 100%;
        border-radius: 0.2rem;
    }

    .bar-number {
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
    }

    .health-bar > .bar-number {
        mix-blend-mode: difference;
    }
</style>
