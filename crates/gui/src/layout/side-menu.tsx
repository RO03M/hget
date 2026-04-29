import { FileTree } from "../components/file-tree/file-tree";
import { useFileTree } from "../hooks/use-file-tree";
import { useStore } from "../store/app-store";

export function SideMenu() {
    const { nodes } = useFileTree();
    const { setPath } = useStore();

    return (
        <div>
            <FileTree
                nodes={nodes}
                root={""}
                onClick={setPath}
            />
        </div>
    )
}