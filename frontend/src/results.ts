import { get } from "@/fetch";
import FilterPanelVue from "./components/FilterPanel.vue";
export async function showResults(refresh: boolean) {
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
        return;
    }
    console.log((possible_input as HTMLInputElement).value);
}