//! Conveniences for managing the entire collection of dependencies (including overrides) for a
//! package
use std::collections::BTreeMap;

use crate::package::{EnvironmentName, PackageName};

/// A set of default dependencies and dep overrides. Within each environment, package names are
/// unique
pub struct DependencySet<T> {
    inner: BTreeMap<Option<EnvironmentName>, BTreeMap<PackageName, T>>,
}

impl<T> DependencySet<T> {
    /// Return a DependencySet with the same structure as [self] but with each entry transformed by
    /// [f].
    fn map<R, F>(&self, f: F) -> DependencySet<R>
    where
        F: Fn(&T) -> R,
    {
        self.iter()
            .map(|(env, package, v)| (env.clone(), package.clone(), f(v)))
            .collect()
    }

    /// Iterate over all entries in the set
    fn iter(&self) -> impl Iterator<Item = (&Option<EnvironmentName>, &PackageName, &T)> {
        self.inner.iter().flat_map(move |(env, deps)| {
            deps.iter()
                .map(move |(package_name, dep)| (env, package_name, dep))
        })
    }
}

impl<T> FromIterator<(Option<EnvironmentName>, PackageName, T)> for DependencySet<T> {
    /// If [iter] produces multiple items with the same environment and package,
    fn from_iter<I: IntoIterator<Item = (Option<EnvironmentName>, PackageName, T)>>(
        iter: I,
    ) -> Self {
        let mut result: BTreeMap<Option<EnvironmentName>, BTreeMap<PackageName, T>> =
            BTreeMap::new();

        for (env, package_name, value) in iter {
            result.entry(env).or_default().insert(package_name, value);
        }

        Self { inner: result }
    }
}
