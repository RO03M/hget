import { invoke } from "@tauri-apps/api/core";
import { useCallback, useState } from "react";
import { safe } from "../utils/safe";
import { HttpRequest } from "../modules/http-page/request-side/types";

export function useFile() {
    const [isLoading, setLoading] = useState(false);

    const getFile = useCallback(async (path: string) => {
        setLoading(true);
        const response = await safe(invoke<HttpRequest>("load_file", { path }));
        setLoading(false);

        return response;
    }, []);

    return {
        getFile,
        isLoading
    }
}