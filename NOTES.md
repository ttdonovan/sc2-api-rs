# Notes

General notes and tips that may be useful in development.

## Protocol Buffers

To generate protobuf source files:

```
protoc -I s2client-proto --rust_out src/protos s2client-proto/s2clientprotocol/*.proto
```

## Python Tips

### Development Setup

```
$ brew install pyenv
$ brew install pyenv-virtualenv
$ eval "$(pyenv init -)"
$ eval "$(pyenv virtualenv-init -)"
$ pyenv virtualenv 2.7.13 pysc2-2.7.13
$ pyenv versions
$ pyenv virtualenvs
$ pyenv activate pysc2-2.7.13
$ pip install .
$ pyenv deactivate
$ pyenv uninstall pysc2-2.7.13
```

### Run an agent with PySC2

A simple way to start an running instance of StarCraft II to receive WebSocket messages.

```
$ git clone https://github.com/deepmind/pysc2
# edit https://github.com/deepmind/pysc2/blob/master/pysc2/lib/sc_process.py#L62 an set the port 5000
$ python -m pysc2.bin.agent --map Simple64
```

### StarCraft Tips

```
$ /Applications/StarCraft\ II/Versions/Base56787/SC2.app/Contents/MacOS/SC2 -listen 127.0.0.1 -port 5000 -displayMode 0
```

## Ideas and References

### SC2API, PySC2, and CommandCenter

* https://github.com/Blizzard/s2client-proto
* https://github.com/Blizzard/s2client-api
* https://github.com/deepmind/pysc2
* https://github.com/davechurchill/CommandCenter

### Rust projects

* https://github.com/SpaceManiac/discord-rs
