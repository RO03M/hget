import { FileTree } from "../components/file-tree/file-tree";
import { useFileTree } from "../hooks/use-file-tree";
import { useStore } from "../store/app-store";
import { useTabs } from "../store/use-tabs";

export function SideMenu() {
    const { tree } = useFileTree();
    const { setPath } = useStore();
    const { add } = useTabs();

    const handleClick = (path: string) => {
        setPath(path);
        add(path);
    }

    if (!tree) {
        return null;
    }

    return (
        <div>
            <FileTree
                nodes={[tree]}
                root={""}
                onClick={handleClick}
            />
        </div>
    )
}