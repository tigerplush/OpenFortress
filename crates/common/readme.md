# common

The common crate is in charge of containing cross-cutting concerns, components, resources, functions, constants etc., that don't have an explicit owner. It is a top-level crate, so it should not have any dependencies except third party dependencies that simplify handling common items.