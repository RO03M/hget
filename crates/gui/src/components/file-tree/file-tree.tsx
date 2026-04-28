import { FSNode } from "../../types";
import { Item } from "./item";
import styles from "./file-tree.module.css";

interface Props {
    root: string;
    nodes: FSNode[];
    depth?: number;
}

export function FileTree(props: Props) {
    const { nodes, depth = 0 } = props;

    if (nodes.length == 0) {
        return null;
    }

    return (
        <ul
            className={styles.folder}
        >
            {nodes.map((node) => (
                <Item
                    node={node}
                    depth={depth}
                    root={`${props.root}/${node.name}`}
                />
            ))}
        </ul>
    );
}