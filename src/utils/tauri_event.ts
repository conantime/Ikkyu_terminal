import {event as tauriEvent} from "@tauri-apps/api";

// @ts-ignore
tauriEvent.listen('ssh-connected', function (info: string) {

})