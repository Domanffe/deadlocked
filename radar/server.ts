import express from "express";
import { WebSocket, WebSocketServer } from "ws";
import path from "path";
import http from "http";

const app = express();
const PORT = 6346;

app.use(express.static(path.join(import.meta.dirname, "dist")));

const server = http.createServer(app);

const ws = new WebSocketServer({ server, path: "/" });

const games: Record<string, { data: any; last_update: number }> = {};
const gameServers: Record<string, WebSocket> = {};

ws.on("connection", (ws, req) => {
    console.info(`connection from ${req.socket.remoteAddress}`);

    (ws as any)._connectionTime = Date.now();
    (ws as any)._remoteAddress = req.socket.remoteAddress;

    ws.on("message", (message) => {
        try {
            const data = JSON.parse(message.toString());
            const kind: string = data["kind"];

            if (kind == "connect_server") {
                connect_server(ws, data);
            } else if (kind == "update_data") {
                update_data(data);
            } else if (kind == "get_data") {
                get_data(ws, data);
            }
        } catch (error) {
            console.error("error parsing message:", error);
        }
    });

    ws.on("close", () => {
        cleanup_connection(ws);
    });

    ws.on("error", (error) => {
        console.error(`websocket error from ${(ws as any)._remoteAddress}:`, error);
        cleanup_connection(ws);
    });
});

function cleanup_connection(ws: WebSocket) {
    for (const [gameUuid, gameWs] of Object.entries(gameServers)) {
        if (gameWs === ws) {
            delete gameServers[gameUuid];
            delete games[gameUuid];
            break;
        }
    }
}

function connect_server(ws: WebSocket, data: any) {
    const uuid = data["uuid"];
    games[uuid] = { data: {}, last_update: Date.now() };
    gameServers[uuid] = ws;

    const message = { kind: "accept" };
    ws.send(JSON.stringify(message));

    console.info(`game server connected with uuid: ${uuid}`);
}

function update_data(data: any) {
    const uuid = data["uuid"];
    if (uuid in games) {
        const gameData = { ...data };
        delete gameData.kind;
        delete gameData.uuid;

        games[uuid].data = gameData;
        games[uuid].last_update = Date.now();
    }
}

function get_data(ws: WebSocket, data: any) {
    const uuid = data["uuid"];
    if (uuid in games) {
        const message = JSON.stringify(games[uuid].data);
        ws.send(message);
    } else {
        console.log(`uuid ${uuid} not found`);
        ws.send(JSON.stringify({}));
    }
}

setInterval(() => {
    const now = Date.now();
    let cleanedGames = 0;

    for (const [uuid, game] of Object.entries(games)) {
        if (now - game.last_update > 1_200_000) {
            delete games[uuid];
            delete gameServers[uuid];
            cleanedGames++;
        }
    }

    if (cleanedGames > 0) {
        console.log(`cleaned ${cleanedGames} games viewers`);
    }
}, 10000);

server.listen(PORT, () => {
    console.info(`server listening on port ${PORT}`);
});
