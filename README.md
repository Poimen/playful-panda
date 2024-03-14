# playful-panda

*** WIP * WIP ***

This is a work-in-progress

*** WIP * WIP ***

This is a playful repo for playing with code in rust.

It is an idea for a url shortener that presents a simple API with a REDIS backend as a datastore. This would require REDIS to have a persistent layer, but as short urls are read more than written, caching this in REDIS make for quicker lookups.

## Requirements

The backend of REDIS is run through docker. There is a docker-compose [file](./docker/docker-compose.yml) that will spin up a REDIS instance.

Naturally, using docker-compose for anything outside testing, should be avoided. Please run REDIS in a proper cluster/permissions etc.

## Running

The project can be run by using the command, in the root:
```bash
REDIS_SERVER_URL="localhost" cargo run
```

This will expect that there is a REDIS server running at the URL. For a test environment, the docker-compose will wil run REDIS in a way all the default configuration setup will expect.

There are some environment variables that can be used:

| Variable         | Description                                       | Default            |
| ---------------- | --------------------------------------------------| ------------------ |
| REDIS_SERVER_URL | URL to the REDIS server to use                    | None, Required     |
| HOST_IP          | Host IP to run the webserver on                   | http://localhost   |
| HOST_PORT        | Host Port to run the webserver on                 | 8000               |
| ALPHABET         | Alphabet to use for short-code generation         | All safe URL codes |
| SHORT_ID_LENGTH  | Length on short code                              | 7                  |

The REDIS server url can contain all the REDIS permission/user login details as required. See REDIS documentation for details.
