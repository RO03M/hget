export interface HttpRequest {
    name: string;
    method: string;
    url: string;
    headers: [string, string][];
    body: string | null;
}

export interface HttpResponse {
    status: number;
    headers: [string, string][];
    body: string;
}

export interface FSNode {
    name: string;
    is_dir: boolean;
    children: FSNode[];
}