import { post } from "@/fetch";
import type Location from "./models/location";
import FilterPanelVue from "./components/FilterPanel.vue";
import SearchPanelVue from "./components/SearchPanel.vue";
export async function showResults(start: number, refresh: boolean): Promise<Location[]> {
    //attributes
    let attribute_checkboxes = document.querySelectorAll(".attribute:checked");
    let attributes = [] as string[];
    attribute_checkboxes.forEach((checkbox) => {
        attributes.push(checkbox.id);
    });
    console.log(attributes);
    //location type
    let location_types_checkboxes = document.querySelectorAll(".location_type:checked");
    let location_types = [] as string[];
    location_types_checkboxes.forEach((checkbox) => {
        location_types.push(checkbox.id);
    });
    console.log(location_types);
    //noise
    let noise_checkboxes = document.querySelectorAll(".noise:checked");
    let noise = [] as string[];
    noise_checkboxes.forEach((checkbox) => {
        noise.push(checkbox.id);
    });
    console.log(noise);

    //check the textbox
    let possible_input = document.getElementById("search-box");
    if (possible_input == null) {
        return [];
    }
    let search = (possible_input as HTMLInputElement).value;

    if (refresh) {

        let pins = await post<Object, Location[]>('http://127.0.0.1:8080/api/v1/locations?limit=9999999&coordinates_only=true&search=' + search + '&start=' + start, {
            'location_types': location_types,
            'attributes': attributes,
            'noise': noise
        });
        //TODO: pins op map krijgen
    }
    try {
        let results = await post<Object, Location[]>('http://127.0.0.1:8080/api/v1/locations?search=' + search + '&start=' + start, {
            'location_types': location_types,
            'attributes': attributes,
            'noise': noise
        });
        return results
    }
    catch (error) {
        console.log(error);
    }

    return [] as Location[];
}