// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

//! Types and methods for external dependencies (of the form `{ r.<res> = data }`)

use std::{collections::BTreeMap, path::Path};

use serde::{Deserialize, Serialize};
use serde_spanned::Spanned;

use crate::{
    errors::{PackageError, PackageResult},
    flavor::MoveFlavor,
};

use super::{DependencySet, PinnedDependencyInfo};

/// An external dependency has the form `{ r.<res> = "<data>" }`; it is resolved by invoking the
/// binary `<res>` (from the `PATH`), and passing `<data>` on the command line. The binary is
/// expected to output a single resolved dependency on the command line.
#[derive(Serialize, Deserialize, Clone)]
pub struct ExternalDependency {
    /// Should be a table with a single entry; the name of the entry is the resolver binary to run
    /// and the value should be the argument passed to the resolver
    r: toml::Value,

    #[serde(flatten)]
    fields: BTreeMap<String, String>,
}

impl ExternalDependency {
    /// Invoke the external binaries and deserialize their outputs as dependencies, then pin the
    /// dependencies.
    pub fn resolve<F: MoveFlavor>(
        deps: &DependencySet<ExternalDependency>,
    ) -> PackageResult<DependencySet<PinnedDependencyInfo<F>>> {
        todo!()
    }
}
