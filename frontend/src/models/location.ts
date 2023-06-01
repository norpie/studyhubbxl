export default interface Location {
    identifier: string;
    name: string;
    location_type: string;
    attributes: string[];
    noise: string;
    address: string;
    lat: number,
    long: number
}
