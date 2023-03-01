---
title: mz user
description: The `mz user` command manages users in your organization.
menu:
  main:
    parent: cli-reference
    weight: 1
---

The `mz user` command manages users in your organization.

## `create`

Invite a new user to the organization.

```shell
mz user create [options]
```

### Flags

Flag                                | Description
------------------------------------|-----------------------
`--email=<EMAIL>    `               | **Required.** The email address of the user.
`--name=<NAME>      `               | **Required.** The name of the user.
`--profile=<PROFILE>`               | Set the authentication profile to use.


### Examples

Invite Franz Kafka to your organization:

```
mz user create --name="Franz Kafka" --email=franz@kafka.org
```

## `list`

List all users in the organization.

```
mz user list
```

### Examples

TODO

## Global flags

{{% cli-global-flags %}}
