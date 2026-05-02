import { useForm, FormProvider } from "react-hook-form";
import { UrlInput } from "./url-input/url-input";
import { RequestSide } from "./request-side/request-side";
import { RequestFormHttpRequest, defaultFormValues } from "./request-side/types";
import { invoke } from "@tauri-apps/api/core";
import { SplitPane } from "../../components/split-pane";
import { ResponseContainer } from "./response-side/response-container";
import { HttpResponse } from "../../types";
import { useEffect, useState } from "react";
import { safe } from "../../utils/safe";
import { useFile } from "../../hooks/use-file";
import { useTabs } from "../../store/use-tabs";
import { paramsFromUrl } from "../../utils/request";

export function HttpPageContainer() {
    const [response, setResponse] = useState<HttpResponse | null>(null);
    const [error, setError] = useState("");
    const [requestName, setRequestName] = useState("Unnamed");

    const methods = useForm<RequestFormHttpRequest>({ defaultValues: defaultFormValues() });
    const { activeTab } = useTabs();
    const { getFile } = useFile();

    async function onSubmit(data: RequestFormHttpRequest) {
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
        if (!activeTab?.path) {
            console.warn("couldn't save because the path was invalid!");
            return;
        }
        const data = methods.getValues();

        await safe(invoke("save_request", {
            path: activeTab?.path,
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
        if (!activeTab) {
            return;
        }

        const path = activeTab?.path;

        (async () => {
            const response = await getFile(path);

            if (response.error != null) {
                return;
            }

            const httpRequest = response.data.http_request;

            const headers = httpRequest.headers.map(([key, value]) => ({
                enabled: true,
                name: key,
                value: value,
            }));

            const rawParams = paramsFromUrl(httpRequest.url);
            const params = rawParams.map(([key, value]) => ({
                enabled: true,
                name: key,
                value: value,
            }));

            // methods.setValue("auth", httpRequest.auth);
            methods.setValue("body", { type: "json", content: httpRequest.body ?? "" });
            methods.setValue("headers", headers);
            methods.setValue("method", httpRequest.method);
            methods.setValue("params", params);
            methods.setValue("url", httpRequest.url);
            methods.setValue("rawHttp", response.data.raw_http ?? "");
        })();
    }, [activeTab]);

    if (!activeTab) {
        return;
    }

    return (
        <FormProvider {...methods}>
            <form
                onSubmit={methods.handleSubmit(onSubmit)}
                style={{
                    display: "flex",
                    flexDirection: "column",
                    flex: 1,
                    overflow: "hidden"
                }}
            >
                <UrlInput
                    onSave={onSave}
                />
                <SplitPane
                    max={1000}
                >
                    <RequestSide />
                    <ResponseContainer
                        response={response}
                    />
                </SplitPane>
                <span>{error}</span>

            </form>
        </FormProvider>
    );
}
