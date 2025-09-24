<script lang="ts">
    import ConnectionIndicator from "./lib/ConnectionIndicator.svelte";
    import Radar from "./lib/Radar.svelte";
    import Settings from "./lib/Settings.svelte";
    import WebSocketComponent from "./lib/WebSocketComponent.svelte";
    import type { Config, Globals } from "./lib/ts/interfaces";
    import { sampleData } from "./lib/ts/sample_data";

    let globals: Globals = {
        connected: false,
        current_player: null,
        data: sampleData /*{
            players: [],
            friendlies: [],
            bomb: {
                planted: false,
                timer: 0.0,
                being_defused: false,
                position: undefined,
            },
            map_name: "",
            in_game: false,
        }*/,
    };
    let settings: Config = {
        show_team: true,
        show_enemy_hp: false,
        color_team: "#6496f0",
        color_enemy: "#f06464",
    };
</script>

<main>
    <WebSocketComponent {globals} />
    <ConnectionIndicator state={globals} />
    <Settings {settings} />
    <div class="main">
        <div class="player-list team-opponents">
            <h1>Opponents</h1>
        </div>

        <Radar {globals} {settings} />

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
        padding: 1rem;
        margin: 0 auto;
        display: flex;
        justify-content: center;
        align-items: center;
        height: calc(100dvh - 2rem);
    }

    .main {
        width: 100%;
        max-height: 100%;
        display: grid;
        grid-template-columns: 1fr 3fr 1fr;
    }

    .player-list {
        border-radius: 1rem;
        background-color: var(--color-base);
    }

    .player-list > h1 {
        padding: 0.4rem 0.8rem;
        margin: 0;
        border-bottom: 1px solid var(--color-highlight);
    }

    .team-opponents > h1 {
        color: var(--color-red);
    }

    .team-friendlies > h1 {
        color: var(--color-blue);
    }

    @media (max-width: 64rem) {
        .main {
            grid-template-columns: 1fr;
        }

        .player-list {
            display: none;
        }
    }
</style>
