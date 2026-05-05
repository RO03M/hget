import { ReactNode } from "react";
import styles from "./index.module.css";

interface MenuListProps {
    children: ReactNode[] | ReactNode;
}

export function MenuList(props: MenuListProps) {
    return (
        <div className={styles["menu-list"]}>
            {props.children}
        </div>
    );
}