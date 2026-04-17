type SafeSuccess<T> = {
    data: T;
    error: null;
};

type SafeError = {
    error: Error;
    data: null;
};

type Safe<T> = SafeSuccess<T> | SafeError;

export function safe<T>(fn: Promise<T>): Promise<Safe<T>>;
export function safe<T>(fn: () => T): Safe<T>;
export function safe<T>(fn: Promise<T> | (() => T)) {
    if (fn instanceof Promise) {
        return safeAsync(fn);
    }

    return safeSync(fn);
}

async function safeAsync<T>(promise: Promise<T>): Promise<Safe<T>> {
    try {
        const data = await promise;
        return {
            error: null,
            data
        };
    } catch (error) {
        if (error instanceof Error) {
            return {
                data: null,
                error: error
            };
        }

        if (typeof error == "string") {
            return {
                data: null,
                error: new Error(error)
            }
        }

        return {
            data: null,
            error: new Error("Something went wrong")
        };
    }
}

function safeSync<T>(fn: () => T): Safe<T> {
    try {
        return {
            error: null,
            data: fn()
        };
    } catch (error) {
        if (error instanceof Error) {
            return {
                data: null,
                error: error
            };
        }

        return {
            data: null,
            error: new Error("Something went wrong")
        };
    }
}
