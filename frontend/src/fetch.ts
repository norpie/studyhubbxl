async function get<T>(url: string): Promise<T | string> {
    await fetch(url)
        .then(response => {
            response.json().then(json => {
                if (json["result"] != null && json["result"] != "") {
                    let object = json["result"] as T;
                    return object;
                }
                return json["error"];
            }).catch(error => {
                return "Failed to get json: " + error.message;
            });

        })
        .catch(error => {
            return "Failed to fetch: " + error.message;
        });
    return ""
}

export default get;
