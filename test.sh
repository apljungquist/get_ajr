#!/bin/bash
set -eux

# This command corresponds to example 1b on https://developer.axis.com/vapix/network-video/api-discovery-service/#get-all-apis

HOST=${AXIS_DEVICE_IP:-"localhost:2001"}
USER=${AXIS_DEVICE_USER:-"root"}
PASS=${AXIS_DEVICE_PASS:-"pass"}

curl -X GET "http://${HOST}/local/get_ajr/vapix/axis-cgi/apidiscovery.cgi?apiVersion=1.0&context=Client%20defined%20request%20ID%201&method=getApiList&params.id=%2A&params.version=%2A" \
     -H "Content-Type: application/json" \
     -H "Accept: application/json" \
     --anyauth \
     -u "${USER}:${PASS}" \
     -v
