import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { FSNode } from "../types";

export function useFileTree() {
    const [nodes, setNodes] = useState<FSNode[]>([]);

    useEffect(() => {
        (async () => {
            const res = await invoke<FSNode[]>("get_tree");
            
            setNodes(res);
        })()
    }, []);

    return {
        nodes
    };
}