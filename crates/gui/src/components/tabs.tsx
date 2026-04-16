import React, { ReactElement } from "react";
import styles from "./tab.module.css";
import { TabProps, TabValue } from "./tab";

interface TabsProps<T extends TabValue> {
    value: T;
    children: ReactElement<TabProps<T>> | ReactElement<TabProps<T>>[];
    onChange: (value: T) => void;
}

export function Tabs<T extends TabValue>(props: TabsProps<T>) {
    return (
        <div
            className={styles.tabBar}
        >
            {React.Children.map(props.children, (child) => {
                if (!React.isValidElement(child)) {
                    return child;
                }

                return React.cloneElement(child, {
                    ...child.props,
                    selected: child.props.value === props.value,
                    onChange: (value) => props.onChange(value)
                });
            })}
        </div>
    )
}