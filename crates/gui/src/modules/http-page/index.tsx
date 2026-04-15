import { useForm, FormProvider } from "react-hook-form";
import { UrlInput } from "./url-input";
import { RequestSide } from "./request-side/request-side";
import { HttpRequest, defaultFormValues } from "./request-side/types";
import { invoke } from "@tauri-apps/api/core";

export function HttpPageContainer() {
    const methods = useForm<HttpRequest>({ defaultValues: defaultFormValues() });

    async function onSubmit(data: HttpRequest) {
        const response = await invoke("send_request", {
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

        console.log(response);
    }

    return (
        <FormProvider {...methods}>
            <form onSubmit={methods.handleSubmit(onSubmit)}>
                <UrlInput />
                <RequestSide />
            </form>
        </FormProvider>
    );
}
