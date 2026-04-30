import { useEffect } from "react";
import { Controller, useFormContext } from "react-hook-form";
import { IconButton } from "../../../components/icon-button";
import { SendIcon } from "../../../icons/send-icon";
import { HttpRequest, KeyValueRow } from "../request-side/types";
import { MethodSelect } from "./method-select";
import styles from "./url-input.module.css";
import { SaveIcon } from "../../../icons/save-icon";

interface Props {
    onSave: () => void;
}

export function UrlInput(props: Props) {
    const { register, watch, getValues, setValue } = useFormContext<HttpRequest>();
    const params = watch("params");

    useEffect(() => {
        const currentUrl = getValues("url");
        const baseUrl = currentUrl.split("?")[0];
        const activeParams = params.filter(p => p.enabled && p.name.trim() !== "");

        const queryString = activeParams
            .map(p => `${encodeURIComponent(p.name)}=${encodeURIComponent(p.value)}`)
            .join("&");

        const newUrl = activeParams.length > 0 ? `${baseUrl}?${queryString}` : baseUrl;
        setValue("url", newUrl);
    }, [JSON.stringify(params)]);

    const handleUrlChange = (event: React.ChangeEvent<HTMLInputElement, HTMLInputElement>) => {
        register("url").onChange(event);

        const [, queryString] = event.target.value.split("?");

        const pairs = queryString.split("&");
        
        const newParams: KeyValueRow[] = [];
        
        for (const pair of pairs) {
            const [arg, value] = pair.split("=");

            newParams.push({
                enabled: true,
                name: arg,
                value: value
            });
        }
        
        setValue("params", newParams);
    }

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
                onChange={handleUrlChange}
                placeholder="Enter URL"
                className={styles["url-input"]}
                spellCheck={false}
                autoCorrect="off"
            />
            <IconButton
                type={"button"}
                onClick={() => props.onSave()}
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
