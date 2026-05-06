import { useEffect } from "react";
import { useTreeStore } from "./use-tree-store";

export function useFileTree() {
    const { tree, refresh } = useTreeStore();

    useEffect(() => {
        refresh();
    }, []);

    return {
        tree
    };
}