import { FSNode } from "../../types";
import { Item } from "./item";
import styles from "./file-tree.module.css";
import { useState } from "react";
import { FolderContextMenu } from "./folder-context-menu";

interface Props {
    root: string;
    nodes: FSNode[];
    depth?: number;
    onClick?: (path: string) => void;
}

export function FileTree(props: Props) {
    const [mouseCoords, setMouseCoords] = useState<[number, number] | null>(null);
    const { nodes, depth = 0 } = props;

    if (nodes.length == 0) {
        return null;
    }

    return (
        <>
            <FolderContextMenu
                open={mouseCoords !== null}
                path={props.root}
                onClose={() => setMouseCoords(null)}
                anchorPosition={{
                    top: mouseCoords?.[1] ?? 0,
                    left: mouseCoords?.[0] ?? 0,
                }}
            />
            <ul
                className={styles.folder}
                onContextMenu={(e) => {
                    e.preventDefault();
                    setMouseCoords([e.clientX, e.clientY]);
                }}
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
        </>
    );
}