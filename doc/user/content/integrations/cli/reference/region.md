---
title: mz region
description: The `mz region` command manages regions in your organization.
menu:
  main:
    parent: cli-reference
    weight: 1
---

The `mz region` command manages regions in your organization.

## `enable`

Enable a region.

```shell
mz region enable [<REGION>]
```


If you do not specify a region, `mz sql` will use the default region for your
authentication profile.


{{< warning >}}
You cannot disable a region with `mz`. To disable a region, contact support.
{{< /warning >}}

### Flags

Flag                                | Description
------------------------------------|-----------------------
`--profile=<PROFILE>`               | Set the authentication profile to use.


## `list`

List all regions.

```shell
mz region list
```

### Flags

Flag                                | Description
------------------------------------|-----------------------
`--profile=<PROFILE>`               | Set the authentication profile to use.


### Examples

```
$ mz region list

Region                  | Status
------------------------|-----------------------
aws/us-east-1           | enabled
aws/eu-west-1           | enabled
```

## `status`

Display detailed status for a region.

```shell
mz region status [<REGION>]
```

### Flags

Flag                                | Description
------------------------------------|-----------------------
`--profile=<PROFILE>`               | Set the authentication profile to use.


### Examples

Display the status of the `aws/us-east-1` region:

```
$ mz region status aws/us-east-1
Healthy:      {yes/no}
SQL address:  2358g2t42.us-east-1.aws.materialize.cloud:6875
HTTP URL:     https://2358g2t42.us-east-1.aws.materialize.cloud
```

## Global flags

{{% cli-global-flags %}}

