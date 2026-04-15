import { Controller, useFormContext } from "react-hook-form";
import { HttpRequest } from "../types";
import styles from "./body-tab.module.css";

export function BodyTab() {
    const { control } = useFormContext<HttpRequest>();

    return (
        <div className={styles.container}>
            <Controller
                control={control}
                name="body.content"
                render={({ field }) => (
                    <textarea
                        {...field}
                        className={styles.editor}
                        placeholder={"{\n  \n}"}
                        spellCheck={false}
                        autoCorrect="off"
                        autoCapitalize="off"
                    />
                )}
            />
        </div>
    );
}
