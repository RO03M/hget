import { ReactNode } from "react";
import styles from "./index.module.css";

interface MenuItemProps {
    onClick?: () => void;
    children: ReactNode[] | ReactNode;
}

export function MenuItem(props: MenuItemProps) {
    return (
        <div
            className={styles["menu-item"]}
            onClick={props.onClick}
        >
            {props.children}
        </div>
    );
}