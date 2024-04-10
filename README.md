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
cargo run
```

There is a `.env.development` that sets up the `REDIS_SERVER` url. Please update that to give the proper URL for the redis server.

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

## Web client

The initial version was written using actix-web, and then ported to axum. Axum performed better, from with random benchmarks.

The axum version has since been iterated upon and improved.

## Performance using bombardier

Write performance (actix-web):
```
$ bombardier -c 125 -n 100000 -m POST http://127.0.0.1:8000/api/short-code -H "Content-Type: application/json" -b '{"ShortUrl": "http://localhost", "Seconds": 100}'
Bombarding http://127.0.0.1:8000/api/short-code with 100000 request(s) using 125 connection(s)
 100000 / 100000 [==================================================================================] 100.00% 11327/s 8s
Done!
Statistics        Avg      Stdev        Max
  Reqs/sec     11590.25    2265.67   23626.36
  Latency       10.78ms     1.21ms    32.66ms
  HTTP codes:
    1xx - 0, 2xx - 100000, 3xx - 0, 4xx - 0, 5xx - 0
    others - 0
  Throughput:     3.40MB/s
```

Write performance (axum):
```
$ bombardier -c 125 -n 100000 -m POST http://127.0.0.1:8000/api/short-code -H "Content-Type: application/json" -b '{"ShortUrl": "http://localhost", "Seconds": 100}'
Bombarding http://127.0.0.1:8000/api/short-code with 100000 request(s) using 125 connection(s)
 100000 / 100000 [==================================================================================] 100.00% 41529/s 2s
Done!
Statistics        Avg      Stdev        Max
  Reqs/sec     44817.37    8597.59   62631.09
  Latency        2.79ms   505.56us    15.79ms
  HTTP codes:
    1xx - 0, 2xx - 100000, 3xx - 0, 4xx - 0, 5xx - 0
    others - 0
  Throughput:    13.12MB/s
```

Redirect performance (actix-web):
```
$ bombardier -c 125 -n 100000 -m GET http://localhost:8000/FzZeTeK
Bombarding http://localhost:8000/FzZeTeK with 100000 request(s) using 125 connection(s)
 100000 / 100000 [==================================================================================] 100.00% 83040/s 1s
Done!
Statistics        Avg      Stdev        Max
  Reqs/sec     96462.98   18407.90  126084.82
  Latency        1.29ms   467.18us    19.38ms
  HTTP codes:
    1xx - 0, 2xx - 0, 3xx - 100000, 4xx - 0, 5xx - 0
    others - 0
  Throughput:    17.27MB/s
```

Redirect performance (axum):
```
$ bombardier -c 125 -n 100000 -m GET http://localhost:8000/FzZeTeK
Bombarding http://localhost:8000/FzZeTeK with 100000 request(s) using 125 connection(s)
 100000 / 100000 [==================================================================================] 100.00% 83014/s 1s
Done!
Statistics        Avg      Stdev        Max
  Reqs/sec     91774.95   20325.00  124183.63
  Latency        1.36ms   620.62us    25.01ms
  HTTP codes:
    1xx - 0, 2xx - 0, 3xx - 100000, 4xx - 0, 5xx - 0
    others - 0
  Throughput:    24.96MB/s
```
