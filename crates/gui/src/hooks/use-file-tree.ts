import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { FSNode } from "../types";

export function useFileTree() {
    const [tree, setTree] = useState<FSNode | null>(null);

    useEffect(() => {
        (async () => {
            const res = await invoke<FSNode>("get_tree");
            
            setTree(res);
        })()
    }, []);

    return {
        tree
    };
}