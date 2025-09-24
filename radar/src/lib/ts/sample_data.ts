import type { WSData } from "./interfaces";

export const sampleData: WSData = {
    players: [
        {
            steam_id: 0,
            health: 30,
            armor: 60,
            position: { x: 0, y: 0, z: 0 },
            head: { x: 0, y: 0, z: 0 },
            name: "sample",
            weapon: "",
            has_defuser: false,
            has_helmet: false,
            has_bomb: false,
            visible: false,
            color: 0,
            rotation: 0,
        },
    ],
    friendlies: [],
    bomb: {
        planted: false,
        timer: 0,
        being_defused: false,
        position: {
            x: 0,
            y: 0,
            z: 0,
        },
    },
    map_name: "de_dust2",
    in_game: true,
};
