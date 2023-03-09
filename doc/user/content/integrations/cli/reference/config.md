---
title: mz config
description: The `mz config` command manages configuration for `mz`.
menu:
  main:
    parent: cli-reference
    weight: 1
---

The `mz config` command manages configuration for the CLI.

## `get`

Get a configuration parameter.

```shell
mz config get <NAME> <VALUE>
```

### Examples

Get the default vault:

```shell
$ mz auth get vault
keychain
```

## `list`

List all configuration parameters.

```shell
mz config list
```

### Examples

```
$ mz config list

Parameter                   | Value
----------------------------|-----------------------
profile                     | default
vault                       | keychain
```

## `set`

Set a configuration parameter.

```shell
mz config set <NAME> <VALUE>
```

### Examples

Set the default vault:

```shell
mz config set vault inline
```

## Global flags

{{% cli-global-flags %}}
