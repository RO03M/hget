import { ButtonHTMLAttributes, ForwardedRef, forwardRef } from "react";

import styles from "./icon-button.module.css";

interface Props extends ButtonHTMLAttributes<HTMLButtonElement> {
    size?: number;
}

export const IconButton = forwardRef((props: Props, ref: ForwardedRef<HTMLButtonElement>) => {
    const { size = 24 } = props;
    
    return (
        <button
            type={"button"}
            ref={ref}
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
});