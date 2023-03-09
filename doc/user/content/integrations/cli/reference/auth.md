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

```shell
mz auth init [<PROFILE>] [flags...]
```

### Flags

Flag                                | Description
------------------------------------|------------
`--force`                           | Whether to force reauthentication if the profile already exists.
`--interactive`, `--no-interactive` | If true, open a browser to authenticate. If false, prompt for a username and password on the terminal.


### Examples

```
$ mz auth init --interactive

Email: remote@example.com
Password: ...

Successfully logged in.
```

## `get`

Get a configuration parameter in an authentication profile.

```shell
mz auth get <NAME> <VALUE>
```

### Flags

Flag                                | Description
------------------------------------|-----------------------
`--profile=<PROFILE>`               | Set the authentication profile to use.

### Examples

Get the default region for the `production` profile:

```
$ mz auth get --profile=production region
aws/us-east-1
```


## `list`

List available authentication profiles.

```shell
mz auth list
```

### Examples

```
$ mz auth list

Profile           |
------------------|
development       |
production        |
staging           |
```

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

```shell
mz auth set --profile=production region aws/eu-west-1
```

## `switch`

Switch the active authentication profile.

```shell
mz auth switch <PROFILE>
```

### Examples

```shell
mz auth switch development
```

## Global flags

{{% cli-global-flags %}}
