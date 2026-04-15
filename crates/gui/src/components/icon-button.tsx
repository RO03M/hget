import { ButtonHTMLAttributes } from "react";

import styles from "./icon-button.module.css";

interface Props extends ButtonHTMLAttributes<HTMLButtonElement> {
    size?: number;
}

export function IconButton(props: Props) {
    const { size = 28 } = props;
    
    return (
        <button
            {...props}
            className={[props.className, styles["icon-button"]].join(" ")}
            style={{
                width: size,
                ...props.style
            }}
        >
            {props.children}
        </button>
    )
}