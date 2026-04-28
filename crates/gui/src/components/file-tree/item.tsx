import { useCallback, useState } from "react";
import { FSNode } from "../../types";
import { FileTree } from "./file-tree";
import styles from "./file-tree.module.css";
import { ArrowIcon } from "../../icons/arrow";

interface Props {
    root: string;
    node: FSNode;
    depth: number;
}

export function Item(props: Props) {
    const { node, depth, root } = props;

    const [collapsed, setCollapsed] = useState(true);

    const toggle = useCallback((event: React.MouseEvent) => {
        event.stopPropagation();

        if (node.is_dir) {
            setCollapsed((prev) => !prev);
            return;
        }

        console.log(root);
    }, [node, root]);

    return (
        <li className={styles.item}>
            <div
                onClick={toggle}
                className={styles.label}
                style={{
                    paddingLeft: 14 * depth
                }}
            >
                {node.is_dir && <ArrowIcon rotation={collapsed ? 0 : 90} size={10} />}{node.name}
            </div>
            {!collapsed && node.is_dir && (
                <FileTree
                    nodes={node.children}
                    depth={depth + 1}
                    root={props.root}
                />
            )}
        </li>
    )
}