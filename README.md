# Yooper
[![builds.sr.ht status](https://builds.sr.ht/~liz/yooper/stable.yml.svg)](https://builds.sr.ht/~liz/yooper/stable.yml?)
_yooper: A person from Michigan's Upper Peninsula_

Yooper is a crate for interacting with UPnP. There's both a CLI and a programatic interface.

## CLI

```
crate install yooper
yooper --help
```

There's two commands: `discover` and `describe`. 
`describe` takes URLs that `discover` produces.

What do you do with the information you get from describe? Up to you! I haven't implemented it yet.

## Library

Every attempt has been made to be as modular as possible. 
There's two optional features: `description` and `cli`. 
If you only need discovery, you can exclude those features.
When `description` is disabled but `cli` isn't, `yooper describe` will not work.
