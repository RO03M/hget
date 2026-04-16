import { useMemo } from "react";
import { HttpResponse } from "../../../types";

interface Props {
    response: HttpResponse;
}

export function ResponseTab(props: Props) {
    const isJson = useMemo(() => {
        for (const [key, value] of props.response.headers) {
            if (key != "content-type") {
                continue;
            }

            return value.includes("application/json");
        }
    }, [props.response.headers]);

    return (
        <pre>
            {isJson ? JSON.stringify(JSON.parse(props.response.body), null, 1) : props.response.body}
        </pre>
    );
}