# GET AJR

_Make GET request to Axis flavored JSON RPC APIs_

## Building

In the dev-container, run `cargo-acap-build`.

## Installing

1. Allow unsigned apps
2. Install by either:
   - using the device's UI, or
   - entering the dev-container and running `cargo-acap-sdk install` with the appropriate arguments

## Using

Make a get request to the application with the appropriate path and query parameters.
See [test.sh](test.sh) for an example.
The request can be made with the browser instead of `curl`.
