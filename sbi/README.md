# Introduction

5G Service Based Interface - Structures.


# Approach

Currently following approach is chosen for implementation, including the rationale for it. While at some point of time it will be great to have our own code generator for all the specs (without having to modify the specs at all, or very little), waiting for that to happen would mean simply spending a lot of time in trying to get code generator working, without making any real progress, which appears to be a somewhat involved activity than originally envisaged. Instead the following approach is followed -

1. We will generate (again) models for the specs as we need them and those models will be saved inside `models/` modules. Any such generated models that are required to be edited by hand are maintained separately inside `models/edited/` module, so that we know how much of this manual work is required. This `models/` directory will be maintained in the repo.

2. In parallel, we'll be working with `generator` code (on a lower priority for now).

3. At some point of time the generator code will be ready, then we will switch to use the `generator` code  and won't use the `models/` anymore.

This should allow us to make progress while still working on a 'better' code generator. At some point of time maybe we will 'publish' the generator independently.

This document will be updated as we make more progress.

