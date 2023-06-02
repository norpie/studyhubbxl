import { reactive } from 'vue'
import type FilterItem from './models/filteritem'
import type Location from './models/location'

export const store = reactive({
    results: [] as Location[],
    noises: [] as FilterItem[],
    attributes: [] as FilterItem[],
    types: [] as FilterItem[],
    favourites: [] as Location[],
    loggedIn: false,
})
