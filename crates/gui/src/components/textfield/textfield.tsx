import { Activity, InputHTMLAttributes, useMemo } from "react";
import styles from "./textfield.module.css";

interface TextFieldProps extends InputHTMLAttributes<HTMLInputElement> {
    error?: string | { message: string };
}

export function TextField(props: TextFieldProps) {
    const error = useMemo(() => {
        if (!props.error) {
            return "";
        }

        if (typeof props.error == "string") {
            return props.error;
        }

        return props.error.message;
    }, [props.error]);

    return (
        <div>
            <input
                {...props}
                className={[props.className, styles.default, Boolean(error) ? styles.error : ""].join(" ")}
            />
            <Activity mode={Boolean(error) ? "visible" : "hidden"}>
                <span className={styles.error}>{error}</span>
            </Activity>
        </div>
    );
}