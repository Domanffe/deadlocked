<script lang="ts">
    import PlayerCard from "./PlayerCard.svelte";
    import type { Globals } from "./ts/interfaces";
    import { PlayerListType } from "./ts/player_list";

    let { globals, type }: { globals: Globals; type: PlayerListType } = $props();
</script>

<div class={["player-list", type === PlayerListType.Friendly ? "team-friendly" : "team-enemy"]}>
    <p>
        {#if type === PlayerListType.Friendly}Teammates{:else}Opponents{/if}
    </p>
    {#each globals.data.players as player}
        <PlayerCard {player} />
    {/each}
</div>

<style>
    .player-list {
        height: fit-content;
        border-radius: 1rem;
        background-color: var(--color-base);
    }

    .player-list > p {
        font-size: 1.2rem;
        padding: 0.4rem 0.8rem;
        margin: 0;
        border-bottom: 1px solid var(--color-highlight);
    }

    .team-enemy > p {
        color: var(--color-red);
    }

    .team-friendly > p {
        color: var(--color-blue);
    }

    @media (max-width: 64rem) {
        .player-list {
            display: none;
        }
    }
</style>
