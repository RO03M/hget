import { Controller, useFormContext } from "react-hook-form";
import { IconButton } from "../../../components/icon-button";
import { SendIcon } from "../../../icons/send-icon";
import { HttpRequest } from "../request-side/types";
import { MethodSelect } from "./method-select";
import styles from "./url-input.module.css";
import { SaveIcon } from "../../../icons/save-icon";

export function UrlInput() {
    const { register } = useFormContext<HttpRequest>();

    return (
        <div className={styles.container}>
            <Controller
                name="method"
                render={({ field }) => (
                    <MethodSelect value={field.value} onChange={field.onChange} />
                )}
            />
            <input
                {...register("url")}
                placeholder="Enter URL"
                className={styles["url-input"]}
                spellCheck={false}
                autoCorrect="off"
            />
            <IconButton
                type={"button"}
            >
                <SaveIcon />
            </IconButton>
            <IconButton
                type={"submit"}
            >
                <SendIcon />
            </IconButton>
        </div>
    );
}
