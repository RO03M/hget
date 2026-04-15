export interface KeyValueRow {
    id: string;
    enabled: boolean;
    name: string;
    value: string;
}

export type BodyType = "json";

export interface HttpRequest {
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
}

export function emptyRow(): KeyValueRow {
    return { id: crypto.randomUUID(), enabled: true, name: "", value: "" };
}

export function defaultFormValues(): HttpRequest {
    return {
        method: "GET",
        url: "",
        params: [emptyRow()],
        headers: [emptyRow()],
        body: { type: "json", content: "" },
        auth: { type: "none", token: "", username: "", password: "" },
    };
}
