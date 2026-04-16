export interface HttpResponse {
    status: number;
    headers: [string, string][];
    body: string;
}