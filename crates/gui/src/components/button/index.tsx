import { ButtonHTMLAttributes } from "react";
import styles from "./button.module.css";

type ButtonVariant = "contained" | "text" | "outlined";

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
    variant?: ButtonVariant;
}

export function Button(props: ButtonProps) {
    const { variant = "contained" } = props;

    return (
        <button
            {...props}
            className={[styles["button"], styles[variant]].join(" ")}
        >
            {props.children}
        </button>
    )
}