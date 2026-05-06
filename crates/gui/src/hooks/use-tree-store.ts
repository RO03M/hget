import { invoke } from "@tauri-apps/api/core";
import { create } from "zustand";
import { FSNode } from "../types";

interface Store {
    tree: FSNode | null;
    refresh: () => Promise<void>;
}

export const useTreeStore = create<Store>((set) => ({
    tree: null,
    refresh: async () => {
        const tree = await invoke<FSNode>("get_tree");

        set({ tree });
    }
}));