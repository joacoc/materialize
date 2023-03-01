---
title: "Materialize CLI Installation"
description: "The Materialize CLI can be installed through several different methods."
menu:
  main:
    parent: cli
    name: Installation
    weight: 1
---

We offer several installation methods for `mz`.

## macOS

On macOS, the preferred installation method is Homebrew.

### Homebrew

You'll need [Homebrew] installed on your system. Then install `mz` from
[our tap][homebrew-tap]:

```
brew install materialize/materialize/mz
```

### Binary download

```
curl -L https://binaries.materialize.com/mz-latest-$(uname -m)-apple-darwin.tar.gz \
    | sudo tar -xzC /usr/local --strip-components=1
```

## Linux

On Linux, the preferred installation method is APT.

### apt (Ubuntu, Debian, or variants)

```
# Add the signing key for the Materialize apt repository
sudo apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 79DEC5E1B7AE7694
# Add and update the repository
sudo sh -c 'echo "deb http://apt.materialize.com/ generic main" > /etc/apt/sources.list.d/materialize.list'
sudo apt update
# Install mz
sudo apt install mz
```

### Binary download

```
curl -L https://binaries.materialize.com/mz-latest-$(uname -m)-unknown-linux-gnu.tar.gz \
    | sudo tar -xzC /usr/local --strip-components=1
```

## Docker

You can use the `materialize/mz` Docker image to run `mz`. You'll need to
mount your local `~/.mz` directory in the container to ensure that configuration
settings and authentiation tokens outlive the container.

```
docker run -v $HOME/.mz:/root/.mz materialize/mz [args...]
```

[Homebrew]: https://brew.sh
[homebrew-tap]: https://github.com/MaterializeInc/homebrew-materialize
