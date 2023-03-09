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

```shell
mz user create --name="Franz Kafka" --email=franz@kafka.org
```

## `list`

List all users in the organization.

```shell
mz user list
```

### Examples

```shell
mz user list
```
```
Email                       | Name
----------------------------|------------------------
production@example.com      | 2023-04-09T12:49:11.000Z
development@example.com     | 2023-04-09T12:39:26.000Z
```

## Global flags

{{% cli-global-flags %}}
