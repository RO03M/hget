import { useEffect, useMemo, useRef } from "react";
import { HttpResponse } from "../../../types";
import hljs from "highlight.js";
import "highlight.js/styles/atom-one-dark.css";

interface Props {
    response: HttpResponse;
}

export function ResponseTab(props: Props) {
    const ref = useRef<HTMLElement>(null);

    const language = useMemo(() => {
        for (const [key, value] of props.response.headers) {
            if (key != "content-type") {
                continue;
            }

            const parts = value.split(";");

            for (const part of parts) {
                if (!part.includes("application/")) {
                    continue;
                }

                const [_, type] = part.split("/");

                return type;
            }
        }

        return "plaintext";
    }, [props.response.headers])

    const formattedContent = useMemo(() => {
        if (language == "json") {
            return JSON.stringify(JSON.parse(props.response.body), null, 4);
        }

        return props.response.body;
    }, [language, props.response.body]);

    useEffect(() => {
        if (!ref.current) {
            return;
        }

        hljs.highlightElement(ref.current);
    }, [formattedContent]);

    return (
        <pre>
            <code ref={ref} className={`language-${language}`}>
                {formattedContent}
            </code>
        </pre>
    );
}