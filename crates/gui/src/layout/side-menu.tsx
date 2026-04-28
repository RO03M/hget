import { FileTree } from "../components/file-tree/file-tree";
import { useFileTree } from "../hooks/use-file-tree";

export function SideMenu() {
    const { nodes } = useFileTree();

    return (
        <div>
            <FileTree
                nodes={nodes}
                root={""}
            />
        </div>
    )
}