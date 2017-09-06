# s2-api

## Development

The repo requires working with git submodules

```
git submodule update --init --recursive
```

To generate protobuf source files

```
protoc -I s2client-proto --rust_out src/protos s2client-proto/s2clientprotocol/*.proto
```
