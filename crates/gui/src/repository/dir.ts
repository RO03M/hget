import { invoke } from "@tauri-apps/api/core";
import { safe } from "../utils/safe";

export async function createDir(name: string, path: string) {
    return await safe(invoke("create_dir_command", {
        path,
        name
    }));
}