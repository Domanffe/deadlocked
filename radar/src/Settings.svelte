<script lang="ts">
    import { fade } from "svelte/transition";
    import Gear from "./Gear.svelte";
    import { Config } from "./interfaces";

    let open = $state(false);

    let { settings }: { settings: Config } = $props();
</script>

<div class="settings">
    <button onclick={() => (open = !open)}><Gear /></button>
    {#if open}
        <div class="settings-menu" transition:fade={{ duration: 200 }}>
            <label>Team Color<input type="color" bind:value={settings.color_team} /></label>
            <label>Enemy Color<input type="color" bind:value={settings.color_enemy} /></label>
        </div>
    {/if}
</div>

<style>
    .settings {
        position: fixed;
        left: 1rem;
        top: 1rem;
        display: flex;
        flex-direction: column;
        align-items: start;
        gap: 0.2rem;
    }

    .settings > button {
        background-color: var(--color-base);
        border-radius: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
        cursor: pointer;
        border: none;
        padding: 0.4rem;
    }

    :global(.settings > button:hover > svg) {
        stroke: var(--color-blue);
    }

    .settings-menu {
        background-color: var(--color-base);
        border-radius: 0.5rem;
        padding: 0.4rem 0.8rem;
    }

    .settings-menu > label {
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .settings-menu > label > input[type="color"] {
        padding: 0;
        border: none;
        margin-left: 0.2rem;
        height: 1.2rem;
        width: 2.4rem;
        border-radius: 0.5rem;
    }
</style>
