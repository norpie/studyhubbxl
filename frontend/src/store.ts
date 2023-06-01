import { reactive } from 'vue'

export const store = reactive({
    results: [],
    pins: [],
    noises: [],
    attributes: [],
    types: [],
    favourites: [],
    loggedIn: false,
})
