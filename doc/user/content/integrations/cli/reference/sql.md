---
title: mz sql
description: The `mz sql` command executes SQL statements in a region.
menu:
  main:
    parent: cli-reference
    weight: 1
---

The `mz sql` command executes SQL statements in a region.

```shell
mz sql [<REGION>] [-- psql options...]
```

If you do not specify a region, `mz sql` will use the default region for your
authentication profile.

## Flags

Flag                                | Description
------------------------------------|-----------------------
`--profile=<PROFILE>`               | Set the authentication profile to use.

## Examples

Launch a SQL shell against the `aws/us-east-1` region:

```
mz sql aws/us-east-1
```

Execute a single SQL query against the `aws/us-east-1` region:

```
mz sql aws/us-east-1 -- -c "SELECT * FROM mz_sources"
```

## Global flags

{{% cli-global-flags %}}
