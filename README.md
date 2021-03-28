## Hubitat Elevation Device Attribute Prometheus Exporter

[![rust](https://github.com/syepes/hubitat_exporter/workflows/rust/badge.svg)](https://github.com/syepes/hubitat_exporter/actions?query=branch%3Amain+event%3Apush+workflow%3Arust)
[![release](https://github.com/syepes/hubitat_exporter/workflows/release/badge.svg)](https://github.com/syepes/hubitat_exporter/actions?query=branch%3Amain+event%3Apush+workflow%3Arelease)

## Functionality
 Uses the Hubitat built-in Maker API application to expose all the device attributes are Prometheus metrics.

## Usage (Docker)
    # List available options
    docker run -it --rm syepes/hubitat_exporter:latest --help

    # Parameters can be either set by arguments or environment variables
    docker run -d --name he -p 8000:8000 syepes/hubitat_exporter:latest -h IP -i MakerAPI-ID -t MakerAPI-TOKEN
    docker run -d --name he -p 8000:8000 -e HE_IP=IP -e HE_API_ID=MakerAPI-ID -e HE_API_TOKEN=MakerAPI-TOKEN syepes/hubitat_exporter:latest
