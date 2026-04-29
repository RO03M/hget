import { useCallback, useMemo } from "react";
import { useStore } from "./app-store";

export function useTabs() {
    const { tabs, setTabs } = useStore();

    const focus = useCallback((path: string) => {
        setTabs((prev) => {
            return prev.map(tab => ({
                ...tab,
                active: tab.path === path
            }));
        });
    }, []);

    const close = useCallback((path: string) => {
        setTabs((prev) => {
            const index = prev.findIndex((tab) => tab.path === path);
            if (index === -1) return prev;

            const next = prev.map(tab => ({ ...tab }));

            if (next[index].active && next.length > 1) {
                const newIndex = index > 0 ? index - 1 : 1;
                next[newIndex].active = true;
            }

            next.splice(index, 1);

            return [...next];
        });
    }, []);

    const add = useCallback((path: string) => {
        setTabs((prev) => {
            const exists = prev.some(tab => tab.path === path);

            if (exists) {
                return prev.map(tab => ({
                    ...tab,
                    active: tab.path === path
                }));
            }

            return [
                ...prev.map(tab => ({ ...tab, active: false })),
                { path, active: true }
            ];
        });
    }, [focus]);

    const reorder = useCallback((path: string, newIndex: number) => {

    }, []);

    const activeTab = useMemo(() => {
        return tabs.find((tab) => tab.active);
    }, [tabs]);

    return {
        tabs,
        activeTab,
        focus,
        close,
        add,
        reorder,
    };
}