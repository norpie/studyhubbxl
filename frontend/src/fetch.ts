// https://eckertalex.dev/blog/typescript-fetch-wrapper
export async function http<T>(path: string, config: RequestInit): Promise<T> {
    const request = new Request(path, config)
    const response = await fetch(request)

    if (!response.ok) {
        throw new Error(/*{ name: response.status, message: response.statusText }*/"Error fetching.")
    }

    // may error if there is no body, return empty array
    let json = await response.json().then(json => { return json }).catch(error => { throw error });
    let object = json.result;
    if (object == null) {
        throw new Error(json.error);
    }
    return object
}

export async function get<T>(path: string, config?: RequestInit): Promise<T> {
    const init = { method: 'get', ...config }
    return await http<T>(path, init)
}

export async function post<T, U>(path: string, body: T, config?: RequestInit): Promise<U> {
    const init = { method: 'post', body: JSON.stringify(body), ...config }
    return await http<U>(path, init)
}
