export interface Config {
    show_team: boolean;
    show_enemy_hp: boolean;
    color_team: string;
    color_enemy: string;
}

export interface Globals {
    connected: boolean;
    current_player: boolean;
    data: WSData;
}

export interface WSData {
    players: PlayerData[];
    friendlies: PlayerData[];
    bomb: BombData;
    map_name: string;
    in_game: boolean;
}

export interface Vec2 {
    x: number;
    y: number;
}

export interface Vec3 {
    x: number;
    y: number;
    z: number;
}

export interface PlayerData {
    steam_id: number;
    health: number;
    armor: number;
    position: Vec3;
    head: Vec3;
    name: string;
    weapon: string;
    has_defuser: boolean;
    has_helmet: boolean;
    has_bomb: boolean;
    visible: boolean;
    color: number;
    rotation: number;
}

export interface BombData {
    planted: boolean;
    timer: number;
    being_defused: boolean;
    position: Vec3;
}

export interface MapInfo {
    x: number;
    y: number;
    scale: number;
    rotate: boolean;
    zoom: number;
    lowerThreshold?: number;
}
