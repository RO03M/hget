import { useForm, FormProvider } from "react-hook-form";
import { UrlInput } from "./url-input/url-input";
import { RequestSide } from "./request-side/request-side";
import { HttpRequest, defaultFormValues } from "./request-side/types";
import { invoke } from "@tauri-apps/api/core";
import { SplitPane } from "../../components/split-pane";
import { ResponseContainer } from "./response-side/response-container";
import { HttpResponse } from "../../types";
import { useState } from "react";
import { safe } from "../../utils/safe";

export function HttpPageContainer() {
    const [response, setResponse] = useState<HttpResponse | null>(null);
    const [error, setError] = useState("");
    const [requestName, setRequestName] = useState("Unnamed");

    const methods = useForm<HttpRequest>({ defaultValues: defaultFormValues() });

    async function onSubmit(data: HttpRequest) {
        const { data: response, error: err } = await safe(invoke<HttpResponse>("send_request", {
            request: {
                name: requestName,
                method: data.method,
                url: data.url,
                headers: data.headers
                    .filter(h => h.enabled && h.name)
                    .map(h => [h.name, h.value] as [string, string]),
                body: data.body.content || null,
            }
        }));

        if (err !== null) {
            setError(err.message);
            return;
        }

        setResponse(response);
    }

    async function onSave() {
        const data = methods.getValues();
        await safe(invoke("save_request", {
            request: {
                name: requestName,
                method: data.method,
                url: data.url,
                headers: data.headers
                    .filter(h => h.enabled && h.name)
                    .map(h => [h.name, h.value] as [string, string]),
                body: data.body.content || null,
            }
        }));
    }

    return (
        <FormProvider {...methods}>
            <form
                onSubmit={methods.handleSubmit(onSubmit)}
                style={{
                    height: "100%"
                }}
            >
                <UrlInput
                    onSave={onSave}
                />
                <span>{error}</span>
                <SplitPane
                    max={1000}
                >
                    <RequestSide />
                    <ResponseContainer
                        response={response}
                    />
                </SplitPane>
            </form>
        </FormProvider>
    );
}
