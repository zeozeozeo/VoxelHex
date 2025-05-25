mod object_pool;
mod spatial;

pub mod boxtree;

#[cfg(any(
    feature = "bytecode",
    feature = "serialization",
    feature = "dot_vox_support"
))]
pub mod convert;

#[cfg(feature = "raytracing")]
pub mod raytracing;

#[derive(Debug, Eq, PartialEq)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

impl Version {
    /// Returns the major version of the library
    /// It increments with major API changes
    pub fn major(&self) -> u32 {
        self.major
    }

    /// Returns the minor version of the library.
    /// It increments with minor API changes
    pub fn minor(&self) -> u32 {
        self.minor
    }

    /// Returns the patch version of the library.
    /// It increments with code modifications not modifying the API
    /// or extending it without making changes to existing API
    pub fn patch(&self) -> u32 {
        self.patch
    }

    /// True if the current version is guaranteed to handle the given tree version without errors
    /// IMPORTANT: the operation is not commutative! Meaning
    /// If v is compatible with v'; v' might not be compatible with v!
    /// i.e. A version of the library might be compatible with models created by previous versions,
    /// but librarys of lower versions can not handle a model created in newer versions
    pub fn compatible(&self, tree_version: &Version) -> bool {
        self.major() == tree_version.major()
            && self.minor() == tree_version.minor()
            && self.patch() >= tree_version.patch()
    }
}

pub fn version() -> Version {
    let numbers: Vec<u32> = env!("CARGO_PKG_VERSION")
        .split(".")
        .into_iter()
        .map(|i| {
            i.parse::<u32>()
                .expect("Expected to be able to parse version string into u32")
        })
        .collect();

    Version {
        major: numbers[0],
        minor: numbers[1],
        patch: numbers[2],
    }
}
