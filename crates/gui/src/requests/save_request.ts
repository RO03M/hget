import { invoke } from "@tauri-apps/api/core";
import { HttpRequest } from "../types";
import { safe } from "../utils/safe";

// path is the path relative to the root of the repository
export async function saveRequest(request: HttpRequest, path: string) {
    await safe(invoke("save_request", {
        path: path,
        request: request
    }))
}