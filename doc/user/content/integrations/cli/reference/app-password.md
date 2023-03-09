---
title: mz app-password
description: The `mz app-password` command manages app passwords for your user account.
menu:
  main:
    parent: cli-reference
    weight: 1
---

The `mz app-password` command manages app passwords for your user account.

## `create`

Create an app password.

```shell
mz app-password create <NAME>
```

### Flags

Flag                                | Description
------------------------------------|-----------------------
`--profile=<PROFILE>`               | Set the authentication profile to use.

### Examples

Create an app password for your production deployment:

```
$ mz app-password create "Production Deployment"
mzp_f283gag2t3...
```

## `list`

List all app passwords.

```shell
mz app-password list
```

### Flags

Flag                                | Description
------------------------------------|-----------------------
`--profile=<PROFILE>`               | Set the authentication profile to use.

### Examples

List all app passwords:

```
$ mz app-password list

Name        | Created at
------------|-----------------
pass1       | January 21, 2022
pass2       | January 23, 2022
...
```

## Global flags

{{% cli-global-flags %}}

