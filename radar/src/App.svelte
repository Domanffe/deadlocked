<script lang="ts">
    import { fade } from "svelte/transition";
    import Gear from "./Gear.svelte";

    const location = window.location;

    let connected = false;

    let state = {
        settingsOpen: false,
    };
    let settings = {
        showTeam: true,
        showEnemyHP: false,
        colorTeam: "#6496f0",
        colorEnemy: "#f06464",
    };
</script>

<main>
    <div class="connection-indicator">
        <span class={connected ? "green" : "red"}></span>
        <p>
            {#if connected}connected{:else}connecting...{/if}
        </p>
    </div>
    <div class="settings">
        <button on:click={() => (state.settingsOpen = !state.settingsOpen)}><Gear /></button>
        {#if state.settingsOpen}
            <div class="settings-menu" transition:fade={{ duration: 200 }}>
                <label>Team Color<input type="color" bind:value={settings.colorTeam} /></label>
                <label>Enemy Color<input type="color" bind:value={settings.colorEnemy} /></label>
            </div>
        {/if}
    </div>
    <div class="main">
        <div class="player-list team-opponents">
            <h1>Opponents</h1>
        </div>

        <div class="radar"></div>

        <div class="player-list team-friendlies">
            <h1>Teammates</h1>
        </div>
    </div>
</main>

<style>
    @font-face {
        font-family: "SequelRounded";
        src: url("/fonts/SequelRounded.woff2");
        font-display: swap;
    }

    @font-face {
        font-family: "Inter";
        src: url("/fonts/Inter.woff2");
        font-display: swap;
    }

    :root {
        --color-backdrop: #0c0c10;
        --color-base: #1e1e28;
        --color-highlight: #323246;
        --color-subtext: #b4b4b4;
        --color-text: #ffffff;
        --color-red: #f06464;
        --color-orange: #f08c5a;
        --color-yellow: #f0c878;
        --color-green: #a0f082;
        --color-teal: #50c8c8;
        --color-blue: #6496f0;
        --color-purple: #b478f0;
    }

    :global(body) {
        background-color: var(--color-backdrop);
        color: var(--color-text);
        margin: 0;
        font-family: "SequelRounded", sans-serif;
    }

    main {
        text-align: center;
        padding: 1em;
        margin: 0 auto;
    }

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

    .main {
        display: grid;
        grid-template-columns: 1fr 3fr 1fr;
    }

    .player-list {
        border-radius: 1rem;
    }

    .team-opponents > h1 {
        color: var(--color-red);
    }

    .team-friendlies > h1 {
        color: var(--color-blue);
    }
</style>
