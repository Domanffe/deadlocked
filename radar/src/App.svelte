<script lang="ts">
    import ConnectionIndicator from "./ConnectionIndicator.svelte";
    import Radar from "./Radar.svelte";
    import Settings from "./Settings.svelte";
    import WebSocketComponent from "./WebSocketComponent.svelte";
    import { Config, Globals } from "./interfaces";

    let globals: Globals = {
        connected: false,
        current_player: null,
        data: {
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
        },
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
        padding: 1em;
        margin: 0 auto;
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

    @media (max-width: 32rem) {
        .main {
            grid-template-columns: 1fr;
        }

        .player-list {
            display: none;
        }
    }
</style>
