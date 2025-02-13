# Install

There's lots of ways to install oranda!

## The Quickest Way

On the [oranda website][website], there's a one-liner command you can execute for your
OS that'll download and install oranda for you, without any further hassle!

## Install Prebuilt Binaries With [cargo-binstall]

```sh
cargo binstall oranda
```

## Build From Source With Cargo

```sh
cargo install oranda --locked --profile=dist
```

> `--profile=dist` is the profile we build our shippable binaries with, it's optional.
>
> `--locked` asks Cargo to respect the lockfile, improving build reproducibility at the
> the cost of not getting any bugfixes from newer releases of its dependencies.


## Download Prebuilt Binaries From Github Releases

[See The Latest Release](https://github.com/axodotdev/oranda/releases/latest)!

## Install With NPM

```sh
npm install oranda
```

[cargo-binstall]:https://github.com/cargo-bins/cargo-binstall
[website]: https://opensource.axo.dev/oranda
