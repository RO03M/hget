import { useCallback, useMemo, useState } from "react";
import { FSNode } from "../../types";
import { FileTree } from "./file-tree";
import styles from "./file-tree.module.css";
import { ArrowIcon } from "../../icons/arrow";
import { DirSettingsButton } from "./dir-settings-button";

interface Props {
    root: string;
    node: FSNode;
    depth: number;
    onClick?: (path: string) => void;
}

export function Item(props: Props) {
    const { node, depth } = props;

    const [collapsed, setCollapsed] = useState(true);

    const path = useMemo(() => {
        return `${props.root}/${node.name}`
    }, [props.root, props.node])

    const toggle = useCallback((event: React.MouseEvent) => {
        event.stopPropagation();

        props.onClick?.(path);

        if (node.is_dir) {
            setCollapsed((prev) => !prev);
            return;
        }
    }, [node, path, props.onClick]);

    return (
        <li className={styles.item}>
            <div
                onClick={toggle}
                className={styles.label}
                style={{
                    paddingLeft: 14 * depth
                }}
            >
                <div className={"row-centered gap-4"}>{node.is_dir && <ArrowIcon rotation={collapsed ? 0 : 90} size={10} />}{node.name}</div>
                <DirSettingsButton
                    node={props.node}
                    path={path}
                />
            </div>
            {!collapsed && node.is_dir && (
                <FileTree
                    nodes={node.children}
                    depth={depth + 1}
                    root={path}
                    onClick={props.onClick}
                />
            )}
        </li>
    )
}