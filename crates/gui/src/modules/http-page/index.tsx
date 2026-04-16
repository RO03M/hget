import { useForm, FormProvider } from "react-hook-form";
import { UrlInput } from "./url-input";
import { RequestSide } from "./request-side/request-side";
import { HttpRequest, defaultFormValues } from "./request-side/types";
import { invoke } from "@tauri-apps/api/core";
import { SplitPane } from "../../components/split-pane";
import { ResponseContainer } from "./response-side/response-container";
import { HttpResponse } from "../../types";
import { useState } from "react";

export function HttpPageContainer() {
    const [response, setResponse] = useState<HttpResponse | null>(null);

    const methods = useForm<HttpRequest>({ defaultValues: defaultFormValues() });

    async function onSubmit(data: HttpRequest) {
        console.log(data);
        const response = await invoke<HttpResponse>("send_request", {
            request: {
                name: "my request",
                method: data.method,
                url: data.url,
                headers: data.headers
                    .filter(h => h.enabled && h.name)
                    .map(h => [h.name, h.value] as [string, string]),
                body: data.body.content || null,
            }
        });

        setResponse(response);
        console.log(response);
    }

    return (
        <FormProvider {...methods}>
            <form onSubmit={methods.handleSubmit(onSubmit)}>
                <UrlInput />
                <SplitPane>
                    <RequestSide />
                    <ResponseContainer
                        response={response}
                    />
                </SplitPane>
            </form>
        </FormProvider>
    );
}
