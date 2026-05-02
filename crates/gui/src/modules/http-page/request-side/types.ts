export interface KeyValueRow {
    enabled: boolean;
    name: string;
    value: string;
}

export type BodyType = "json";

export interface RequestFormHttpRequest {
    method: string;
    url: string;
    params: KeyValueRow[];
    headers: KeyValueRow[];
    body: {
        type: BodyType;
        content: string;
    };
    auth: {
        type: "none" | "bearer" | "basic";
        token: string;
        username: string;
        password: string;
    };
    rawHttp?: string;
}

export function emptyRow(): KeyValueRow {
    return { enabled: true, name: "", value: "" };
}

export function defaultFormValues(): RequestFormHttpRequest {
    return {
        method: "GET",
        url: "",
        params: [emptyRow()],
        headers: [emptyRow()],
        body: { type: "json", content: "" },
        auth: { type: "none", token: "", username: "", password: "" },
    };
}
