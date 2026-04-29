import { FileTree } from "../components/file-tree/file-tree";
import { useFileTree } from "../hooks/use-file-tree";
import { useStore } from "../store/app-store";
import { useTabs } from "../store/use-tabs";

export function SideMenu() {
    const { nodes } = useFileTree();
    const { setPath } = useStore();
    const { add } = useTabs();

    const handleClick = (path: string) => {
        setPath(path);
        add(path);
    }

    return (
        <div>
            <FileTree
                nodes={nodes}
                root={""}
                onClick={handleClick}
            />
        </div>
    )
}