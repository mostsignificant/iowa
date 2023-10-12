![build](https://github.com/mostsignificant/iowa/actions/workflows/build.yml/badge.svg)
![docker](https://github.com/mostsignificant/iowa/actions/workflows/docker.yml/badge.svg)

# IOWA

A HTTP-based key-value-store written in Rust. I wrote a blog article about building it on
[medium](https://medium.com/@mostsignificant/how-to-build-a-simple-but-efficient-key-value-store-with-rust-and-actix-b65ffc2ab4a0).

## Documentation

### Starting The Server

Start the IOWA server by running the `iowa` executable. You can optionally pass the `--host` (default: 0.0.0.0) and `--port` (default: 1984) command line options.

```sh
iowa --host 0.0.0.0 --port 1984
```

### Adding Values

Add a new value to the IOWA server via HTTP POST. The key will be the base path of the URL. In the following example, the key will be `hello/world` and the value `i am alive`.

```sh
curl -d'i am alive' -X POST 'localhost:1984/hello/world'
```

### Getting Values

You can retrieve the same value again via a simple HTTP GET call.

```sh
curl 'localhost:1984/hello/world'
> i am alive
```

### Deleting Values

Deleting a key and its value can be done via HTTP DELETE.

```sh
curl -X DELETE 'localhost:1984/hello/world'
```

## Build

You can build the IOWA server by cloning the repository and building via cargo.

```sh
git clone https://github.com/mostsignificant/iowa.git
cd iowa
cargo build --release
```
