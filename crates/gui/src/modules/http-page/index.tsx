import { useForm, FormProvider } from "react-hook-form";
import { UrlInput } from "./url-input/url-input";
import { RequestSide } from "./request-side/request-side";
import { HttpRequest, defaultFormValues } from "./request-side/types";
import { invoke } from "@tauri-apps/api/core";
import { SplitPane } from "../../components/split-pane";
import { ResponseContainer } from "./response-side/response-container";
import { HttpResponse } from "../../types";
import { useEffect, useState } from "react";
import { safe } from "../../utils/safe";
import { useStore } from "../../store/app-store";
import { useFile } from "../../hooks/use-file";

export function HttpPageContainer() {
    const [response, setResponse] = useState<HttpResponse | null>(null);
    const [error, setError] = useState("");
    const [requestName, setRequestName] = useState("Unnamed");

    const methods = useForm<HttpRequest>({ defaultValues: defaultFormValues() });
    const { path } = useStore();
    const { getFile } = useFile();

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

    useEffect(() => {
        if (path === null) {
            return;
        }

        (async () => {
            const response = await getFile(path);

            if (response.error != null) {
                return;
            }
            console.log(response.data);
            methods.setValue("auth", response.data.auth);
            methods.setValue("body", response.data.body ?? "");
            methods.setValue("headers", []);
            methods.setValue("method", response.data.method);
            methods.setValue("params", []);
            methods.setValue("url", response.data.url);
        })();
    }, [path]);

    if (!path) {
        return;
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
