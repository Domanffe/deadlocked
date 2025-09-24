<script lang="ts">
    import type { Globals } from "./ts/interfaces";

    let { globals }: { globals: Globals } = $props();
</script>

<div class="connection-indicator">
    <span class={globals.connected ? "green" : "red"}></span>
    <p>
        {#if globals.connected}connected{:else}connecting...{/if}
    </p>
</div>

<style>
    p {
        margin: 0;
    }

    .connection-indicator {
        position: fixed;
        left: 50%;
        top: 1rem;
        transform: translateX(-50%);
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;
        gap: 0.3rem;
        padding: 0.3rem 0.6rem;
        border-radius: 0.5rem;
        background-color: var(--color-base);
    }

    .connection-indicator > span {
        display: block;
        width: 0.5rem;
        height: 0.5rem;
        border-radius: 100%;
    }

    .green {
        background-color: var(--color-green);
    }

    .red {
        background-color: var(--color-red);
    }
</style>
