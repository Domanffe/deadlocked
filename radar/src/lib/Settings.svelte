<script lang="ts">
    import { fade } from "svelte/transition";
    import Gear from "./Gear.svelte";
    import type { Config } from "./ts/interfaces";

    let open = $state(false);

    let { settings = $bindable() }: { settings: Config } = $props();
</script>

<div class="settings">
    <button onclick={() => (open = !open)}><Gear /></button>
    {#if open}
        <div class="settings-menu" transition:fade={{ duration: 200 }}>
            <label><input type="checkbox" bind:checked={settings.show_team} />Show Team</label>
            <label><input type="checkbox" bind:checked={settings.show_enemy_hp} />Show Enemy HP</label>
            <label><input type="color" bind:value={settings.color_team} />Team Color</label>
            <label><input type="color" bind:value={settings.color_enemy} />Enemy Color</label>
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
        display: flex;
        flex-direction: column;
        align-items: start;
        background-color: var(--color-base);
        border-radius: 0.5rem;
        padding: 0.4rem 0.8rem;
    }

    .settings-menu > label {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.2rem;
        cursor: pointer;
    }

    .settings-menu > label > input {
        cursor: pointer;
    }

    .settings-menu > label > input[type="color"] {
        padding: 0;
        border: none;
        margin-left: 0.2rem;
        height: 1.2rem;
        width: 2.4rem;
        border-radius: 0.5rem;
    }

    .settings-menu > label > input[type="checkbox"] {
        width: 1rem;
        height: 1rem;
        margin: 0;
        background-color: var(--color-highlight);

        &:checked {
            background-color: var(--color-blue);
        }
    }
</style>
