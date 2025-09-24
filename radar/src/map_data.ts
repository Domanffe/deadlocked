import { MapInfo } from "./interfaces";

export const MapData: Record<string, MapInfo> = {
    de_ancient_night: { x: -2953, y: 2164, scale: 5.1, rotate: false, zoom: 1 },
    de_dust2: { x: -2476, y: 3239, scale: 4.4, rotate: true, zoom: 1.1 },
    de_inferno: { x: -2087, y: 3870, scale: 4.9, rotate: false, zoom: 1 },
    de_mirage: { x: -3230, y: 1713, scale: 5, rotate: false, zoom: 1 },
    de_nuke: { x: -3453, y: 2887, scale: 7, rotate: false, zoom: 1, lowerThreshold: -495 },
    de_overpass: { x: -4831, y: 1781, scale: 5.2, rotate: false, zoom: 1 },
    de_vertigo: { x: -3168, y: 1762, scale: 4, rotate: false, zoom: 1, lowerThreshold: 11700 },
};
