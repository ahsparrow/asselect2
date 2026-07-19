# ASSelect

## Building

1. Copy the following files from aip_airspace/build to the data directory

    - `airspace.geojson`
    - `loa.geojson`
    - `rat.geojson`
    - `obstacle.geojson`
    - `overlay_105.geojson`
    - `overlay_195.geojson`
    - `overlay_atzdz.geojson`

1. Build the release

    `trunk build --release`

1. Upload the release to Cloudflare

    `npm run deploy`
