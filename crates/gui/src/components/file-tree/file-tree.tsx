import { FSNode } from "../../types";
import { Item } from "./item";
import styles from "./file-tree.module.css";

interface Props {
    root: string;
    nodes: FSNode[];
    depth?: number;
    onClick?: (path: string) => void;
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
                    key={`${props.root}/${node.name}`}
                    node={node}
                    depth={depth}
                    root={props.root}
                    onClick={(v) => props.onClick?.(v)}
                />
            ))}
        </ul>
    );
}