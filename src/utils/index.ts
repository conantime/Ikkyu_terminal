import {invoke} from "@tauri-apps/api/core";

export async function getPwd(): Promise<String> {
    return invoke('get_pwd')
}


export function removeQuotes(str: string) {
    if (typeof str !== 'string') {
        str = String(str)
    }
    return str.replace(/^["']|["']$/g, '');
}