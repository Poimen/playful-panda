# playful-panda

*** WIP * WIP ***

This is a work-in-progress

*** WIP * WIP ***

This is a playful repo for playing with code in rust.

It is an idea for a url shortener that presents a simple API with a REDIS backend as a datastore. This would require REDIS to have a persistent layer, but as short urls are read more than written, caching this in REDIS make for quicker lookups.

## Requirements

The backend of REDIS is run through docker. There is a docker-compose [file](./docker/docker-compose.yml) that will spin up a REDIS instance.

Naturally, using docker-compose for anything outside testing, should be avoided. Please run REDIS in a proper cluster/permissions etc.
