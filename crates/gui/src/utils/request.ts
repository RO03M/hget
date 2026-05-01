export function paramsFromUrl(url: string): [string, string][] {
    const [, queryString] = url.split("?");

    if (!queryString) {
        return [];
    }

    const pairs = queryString.split("&");

    const params: [string, string][] = [];

    for (const pair of pairs) {
        const [arg, value] = pair.split("=");

        params.push([arg, value]);
    }

    return params;
}