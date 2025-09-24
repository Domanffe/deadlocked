<script lang="ts">
    import { WSData } from "./data";

    export let state: { connected: boolean; data: WSData };

    const location = window.location;
    const uuid = new URLSearchParams(location.search).get("uuid");
    const url = `ws://${location.hostname}:${location.port}`;

    let websocket: WebSocket | null;

    export function startWebSocket() {
        stopWebSocket();

        console.log("Opening WebSocket connection...");

        websocket = new WebSocket(url);
        websocket.onmessage = wsMessage;
        websocket.onopen = () => {
            console.info("websocket connected successfully");
            state.connected = true;
        };
        websocket.onerror = (error) => {
            console.error("websocket error:", error);
            state.connected = false;
        };
        websocket.onclose = (event) => {
            console.info("websocket closed: ", event.code, event.reason);
            state.connected = false;
        };
    }

    export function stopWebSocket() {
        if (websocket) {
            websocket.close();
            websocket = null;
        }
    }

    function wsMessage(event: MessageEvent<string>) {
        try {
            const json: WSData = JSON.parse(event.data);
            state.data = json;
        } catch (error) {
            console.error("error parsing text: ", error);
        }
    }
</script>
