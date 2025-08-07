#!/bin/bash
set -eux

# This command corresponds to example 1b on https://developer.axis.com/vapix/network-video/api-discovery-service/#get-all-apis

# To run with a real device:
# - Replace `localhost:2001` with the address of your device.
#   No need to include a port, unless the device is configured to use a non-standard port.
# - Add authentication e.g. like `--anyauth -u "root:pass"
#   where "root" and "pass" are the username and password for an admin user on the device.
curl -X GET 'http://localhost:2001/local/get_ajr/vapix/axis-cgi/apidiscovery.cgi?$.apiVersion=1.0&$.context=Client%20defined%20request%20ID%201&$.method=getApiList&$.params.id=*&$.params.version=*' \
     -H "Content-Type: application/json" \
     -H "Accept: application/json" \
     -v
