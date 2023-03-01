---
title: "Materialize CLI Configuration"
description: "An authentication profile..."
menu:
  main:
    parent: cli
    name: Configuration
    weight: 2
---

`mz` is configured via a single TOML file stored at
`$HOME/.config/materialize/mz.toml`.

You typically manage configuration via the following two commands:

  * [`mz auth`](../reference/auth), which manages authentication profiles.
  * [`mz config`](../reference/config), which manages other configuration
    settings.

You can also edit the file directly if you prefer. The format of the file is
shown in the [Example](#example) section.

## Authentication profiles

You can configure `mz` with multiple **authentication profiles** to seamlessly
connect to multiple Materialize user accounts. Each profile has a name, an
associated app password, and a default region.

When invoking an `mz` command that requires authentication, you can explicitly
choose which profile to use by passing the `--profile` flag. For example, to use
the `staging` profile with the `mz sql` command:

```
mz sql --profile=staging aws/us-east-1
```

When the profile is not explicitly specified, `mz` uses the profile specified in
the configuration file. You can switch the active profile with the [`mz auth
switch`](../reference/auth/#switch) command:

```
mz auth switch staging
```

## Configuration parameters

Besides authentication, `mz` supports several other configuration knobs,
documented in the table below.

Name      | Description
----------|------------------------
`profile` | The active profile. Default: `default`.
`vault`   | What vault to use to store secrets: `inline` or `keychain`. When set to `inline`, app passwords are stored directly in the configuration file. When set to `keychain`, app passwords are stored in the system keychain (macOS only).

## Reference

```toml
# Activate the "production" authentication profile by default.
profile = "production"

# Store app passwords directly in the configuration file.
vault = "inline"

[profile.production]
# The ID of the production Materialize organization.
organization-id = "5a50482b-dfcf-4abf-94e1-598b23d5bd2c"
# The app password that the CLI will use to authenticate.
app-password = "mzp_fg91g4fslgq329023..."
# The default region to use for the production organization.
region = "aws/us-east-1"
# Endpoint overrides.
#
# For internal developer use when running the CLI against staging environments.
api-endpoint = "https://cloud.materialize.com"
admin-endpoint = "https://admin.cloud.materialize.com"
```
