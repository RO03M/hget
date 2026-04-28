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