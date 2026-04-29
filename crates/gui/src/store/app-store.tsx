import { createContext, ReactNode, useContext, useState } from "react";

interface AppStore {
    path: string | null;
    setPath: (path: string) => void;
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

    return (
        <AppContext
            value={{
                path: path,
                setPath: setPath
            }}
        >
            {props.children}
        </AppContext>
    )
}