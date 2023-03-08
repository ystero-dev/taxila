# Introduction

This repository hosts an implementation of 5G Core network in Rust. This is a Work In Progress and is in actively being developed. This repository cannot really be used at the moment to test out anything.

## Sub Projects

This project hosts following sub projects which are released as their own crates -

1. `ngap` - This is a wrapper over 3GPP NGAP Specifications from Release 17. See the README.md file of the sub project for more details.
2. `nas` - This crate provides implementation of 5G NAS Messages that are required by core implementations.
3. `netfns` - This is an implementation of network functions like AMF, SMF  etc.
4. `sbi` - Service Based Interface. Includes data structures and stubs for service based interface. Please see the [README.md](https://github.com/gabhijit/taxila/blob/main/sbi/README.md) for the latest support.

# License

See LICENSE information for each of the sub projects.
