import { store } from '@/store';
import { VueCookieNext } from 'vue-cookie-next'

// https://eckertalex.dev/blog/typescript-fetch-wrapper
export async function http<T>(path: string, config: RequestInit): Promise<T> {
    const request = new Request(path, config)
    const response = await fetch(request)

    if (response.status == 401) {
        VueCookieNext.removeCookie("session");
        store.loggedIn = false;
        let json = await response.json().then(json => { return json }).catch(error => { throw error });
        let object = json.result;
        if (object == null) {
            throw new Error(json.error);
        }
    }

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

export async function rawhttp<T>(request: Request): Promise<T> {
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

export async function loginpost<T>(url: string, body: Object): Promise<T> {
    const body_string = JSON.stringify(body);
    const length = (new TextEncoder().encode(body_string)).length;
    const request = new Request(url, {
        method: 'POST',
        body: body_string,
        headers: {
            'Content-Type': 'application/json; charset=UTF-8',
            'Content-Length': length.toString(),
        },
    })
    try {
        const response = await fetch(request);
        if (!response.ok) {
            let json = await response.json().then(json => { return json }).catch(error => { throw error });
            throw json["error"];
        }

        // may error if there is no body, return empty array
        let json = await response.json().then(json => { return json }).catch(error => { throw error });
        let object = json.result;
        return object
    } catch (error) {
        throw "Internal error, try again later.";
    }
}

export async function get<T>(path: string, config?: RequestInit): Promise<T> {
    const init = { method: 'get', ...config }
    return await http<T>(path, init)
}

export async function delete_<T>(path: string, config?: RequestInit): Promise<T> {
    const init = { method: 'delete', ...config }
    return await http<T>(path, init)
}

export async function post<T, U>(path: string, body: T, config?: RequestInit): Promise<U> {
    const init = { method: 'post', body: JSON.stringify(body), ...config }
    return await http<U>(path, init)
}
