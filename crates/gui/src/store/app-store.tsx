import { createContext, ReactNode, useContext, useState } from "react";
import { Tab } from "./types";

interface AppStore {
    path: string | null;
    setPath: (path: string) => void;
    tabs: Tab[];
    setTabs: React.Dispatch<React.SetStateAction<Tab[]>>;
}

const AppContext = createContext<AppStore>(null!);

interface AppStateProps {
    children: ReactNode;
}

export function useStore() {
    return useContext(AppContext);
}

export function AppState(props: AppStateProps) {
    const [path, setPath] = useState<null | string>(null);
    const [tabs, setTabs] = useState<Tab[]>([]);

    return (
        <AppContext
            value={{
                path: path,
                setPath: setPath,
                tabs,
                setTabs
            }}
        >
            {props.children}
        </AppContext>
    )
}