# url-short-rs

This is a playful repo for playing with code in rust.

It is an idea for a url shortener that presents a simple API with a Redis backend as a datastore. This would require Redis to have a persistent layer, but as short urls are read more than written, caching this in Redis make for quicker lookups.

## Details

The backend of Redis is run through docker. There is a docker-compose [file](./docker/docker-compose.yml) that will spin up a Redis instance.

Naturally, using docker-compose for anything outside testing, should be avoided. Please run Redis in a proper cluster/permissions etc.

The url handler doesn't dedup any data, so you can generate short urls for the same url multiple times, each time will give a different short code. They are all stored separately as well. The url can be set to have a TTL (time-to-live) where it will no longer be available.

## Running

The project can be run by using the command, in the root:
```bash
 REDIS_SERVER_URL="redis://:changeit@localhost" cargo run
```

This will expect that there is a REDIS server running at the URL. For a test environment, the docker-compose will wil run Redis in a way all the default configuration setup will expect.

There are some environment variables that can be used:

| Variable         | Description                                       | Default            |
| ---------------- | --------------------------------------------------| ------------------ |
| REDIS_SERVER_URL | URL to the REDIS server to use                    | None, Required     |
| HOST_IP          | Host IP to run the webserver on                   | localhost          |
| HOST_PORT        | Host Port to run the webserver on                 | 8000               |
| ALPHABET         | Alphabet to use for short-code generation         | All safe URL codes |
| SHORT_ID_LENGTH  | Length on short code                              | 7                  |
| SHORT_ID_REPEAT_CLASH_LENGTH | Number of Repeats to perform if there is a key clash | 5  |

The Redis server url can contain all the Redis permission/user login details as required. See Redis documentation for details.


## Testing

There is a http [file](./http/test_command.http) that will run test requests against the server.

The endpoints exposed are:
```
GET /api/health
```
Check if the service is up and running

```
POST /api/short-code
```
Generate a shortcode for a given url

```
GET {server}:80/{short-code}
```
Redirect to the short code, or 404

## Performance using bombardier

Write performance:
```
$ bombardier -c 125 -n 100000 -m POST http://127.0.0.1:8000/api/short-code -H "Content-Type: application/json" -b '{"ShortUrl": "https://www.google.com", "Sseconds": 100}'
Bombarding http://127.0.0.1:8000/api/short-code with 100000 request(s) using 125 connection(s)
 100000 / 100000 [=========================================================================================================================================================================] 100.00% 1093/s 1m31s
Done!
Statistics        Avg      Stdev        Max
  Reqs/sec      1094.81    1216.03    6755.92
  Latency      114.02ms    80.34ms   740.46ms
  HTTP codes:
    1xx - 0, 2xx - 56462, 3xx - 0, 4xx - 43538, 5xx - 0
    others - 0
  Throughput:   359.59KB/s
```

Write performance does not account for key-clashes

Redirect performance:
```
$ bombardier -c 125 -n 100000 -m GET http://localhost:8000/EL_nCSq
Bombarding http://localhost:8000/EL_nCSq with 100000 request(s) using 125 connection(s)
 100000 / 100000 [=========================================================================================================================================================================] 100.00% 1182/s 1m24s
Done!
Statistics        Avg      Stdev        Max
  Reqs/sec      1185.16    1478.58    9040.65
  Latency      105.40ms    68.43ms   435.15ms
  HTTP codes:
    1xx - 0, 2xx - 0, 3xx - 56461, 4xx - 43539, 5xx - 0
    others - 0
  Throughput:   202.68KB/s
```

This redirects to Google.com
