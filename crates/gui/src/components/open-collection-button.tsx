import { open } from "@tauri-apps/plugin-dialog";
import { useCallback } from "react"

export function OpenCollectionButton() {
    const openFolderPicker = useCallback(async () => {
        const folder = await open({
            directory: true
        });

        console.log(folder);
    }, []);

    return (
        <button onClick={openFolderPicker}>open collection</button>
    );
}