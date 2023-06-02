import { get, rawhttp } from "@/fetch";
import { store } from "@/store";
import mapboxgl from "mapbox-gl";
import { createApp } from "vue";
import type Coordinate from "./models/coordinate";
import type FilterItem from "./models/filteritem";
import type Location from "./models/location";
//import Card from '@/components/Card'

export async function showResults(start: number, refresh: boolean): Promise<Location[]> {
    //attributes
    let attribute_checkboxes = document.querySelectorAll(".attribute:checked");
    let attributes = [] as string[];
    attribute_checkboxes.forEach((checkbox) => {
        attributes.push(checkbox.id);
    });
    //location type
    let location_types_checkboxes = document.querySelectorAll(".location_type:checked");
    let location_types = [] as string[];
    location_types_checkboxes.forEach((checkbox) => {
        location_types.push(checkbox.id);
    });
    //noise
    let noise_checkboxes = document.querySelectorAll(".noise:checked");
    let noise = [] as string[];
    noise_checkboxes.forEach((checkbox) => {
        noise.push(checkbox.id);
    });

    const default_location_types = await get<FilterItem[]>('http://localhost:8080/api/v1/filters/location_type');
    const default_noises = await get<FilterItem[]>('http://localhost:8080/api/v1/filters/noise');

    if (location_types.length == 0) {
        location_types = default_location_types.map(i => i.identifier);
    }

    if (noise.length == 0) {
        noise = default_noises.map(i => i.identifier);
    }

    //check the textbox
    let search = "";
    let possible_input = document.getElementById("search-box");
    if (possible_input != null) {
        search = (possible_input as HTMLInputElement).value;
    }
    const body =
    {
        search: search,
        limit: 10000,
        start: start,
        coordinates_only: false,
        location_types: location_types,
        attributes: attributes,
        noise: noise
    };
    const url = 'http://localhost:8080/api/v1/locations';

    //TODO: pins op map krijgen
    try {
        body.coordinates_only = false;
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
        let results = await rawhttp<Location[]>(request);
        //if (refresh) {
        //    body.coordinates_only = true;
        //    body.limit = 9999999;
        //    const body_string = JSON.stringify(body);
        //    const length = (new TextEncoder().encode(body_string)).length;
        //    const request = new Request(url, {
        //        method: 'POST',
        //        body: body_string,
        //        headers: {
        //            'Content-Type': 'application/json; charset=UTF-8',
        //            'Content-Length': length.toString(),
        //        },
        //    })
        //    let pins: Coordinate[] = await rawhttp<Coordinate[]>(request);
        //    store.pins = pins;
        //}
        if (refresh) {
            store.results = results;
        } else {
            store.results = store.results.concat(results);
        }
    } catch (error) {
        console.log(error);
    }

    return [] as Location[];
}
