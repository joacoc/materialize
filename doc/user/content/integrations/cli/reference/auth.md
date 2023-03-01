---
title: mz auth
description: The `mz auth` command manages authentication profiles for `mz`.
menu:
  main:
    parent: cli-reference
    weight: 1
---

The `mz auth` command manages authentication profiles for `mz`.

## `init`

Initialize an authentication profile.

```
mz auth init [<PROFILE>] [flags...]
```

### Flags

Flag                                | Description
------------------------------------|------------
`--force`                           | Whether to force reauthentication if the profile already exists.
`--interactive`, `--no-interactive` | If true, open a browser to authenticate. If false, prompt for a username and password on the terminal.


### Examples

TODO

## `get`

Get a configuration parameter in an authentication profile.

```
mz auth get <NAME> <VALUE>
```

### Flags

Flag                                | Description
------------------------------------|-----------------------
`--profile=<PROFILE>`               | Set the authentication profile to use.

### Examples

Get the default region for the `production` profile:

```shell
$ mz auth get --profile=production region
aws/us-east-1
```


## `list`

List available authentication profiles.

```
mz auth list
```

### Examples

TODO

## `set`

Set a configuration parameter in an authentication profile.

```shell
mz auth set <NAME> <VALUE>
```

### Flags

Flag                                | Description
------------------------------------|-----------------------
`--profile=<PROFILE>`               | Set the authentication profile to use.

### Examples

Set the default region for the `production` profile:

```
mz auth set --profile=production region aws/eu-west-1
```

## `switch`

Switch the active authentication profile.

```
mz auth switch <PROFILE>
```

### Examples

TODO

## Global flags

{{% cli-global-flags %}}
