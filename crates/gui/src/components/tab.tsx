import styles from "./tab.module.css";

export type TabValue = number | string | boolean;

export interface TabProps<T extends TabValue> {
    value: T;
    label: string;
    selected?: boolean;
    badge?: number;
    onChange?: (value: T) => void;
}

export function Tab<T extends TabValue>(props: TabProps<T>) {
    return (
        <button
            type="button"
            className={`${styles.tabBtn} ${props.selected ? styles.tabBtnActive : ""}`}
            onClick={() => props?.onChange?.(props.value)}
        >
            {props.label}
            {!!props.badge && (
                <span className={styles.badge}>{props.badge}</span>
            )}
        </button>
    );
}
