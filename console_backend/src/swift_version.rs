use std::{cmp::Ordering, str::FromStr};

use anyhow::anyhow;
use lazy_static::lazy_static;
use regex::Regex;

/// Represents a version string generated by git. This string will be in the
/// format v<marketing>.<major>.<minor>(-dev) where marketing, major, and minor
/// are integers and dev is an optional string that is only present for
/// non-release builds.
///
/// The marketing, major, and minor components are separated out and stored in
/// appropriately named properties. The remaining parts of the string are
/// combined into the dev string property. In practice this string will consist
/// of a leading 'v' characters, plus whatever characters trailed the minor
/// number. The devstring property will therefore always contain at least 1
/// character, a minimum of the leading 'v'. If the devstring contains /only/ a
/// 'v' the version string is not considered to be a development build, but if
/// the dev string is any longer the build is a development build and the
/// 'isdev' property will return True.
#[derive(Debug)]
pub struct SwiftVersion {
    marketing: u64,
    major: u64,
    minor: u64,
    dev: String,
}

impl FromStr for SwiftVersion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            // This regex separates out components of the input string into
            // regex match groups. The components are:
            //
            //  * leader: Any number of non-digit characters
            //  * marketing - Integer (1 of more characters, 0-9)
            //  * major - Integer
            //  * minor - Integer
            //  * dev - Any number of characters
            //
            // Leading whitespace is stripped away, any trailing whitespace is
            // included in the 'dev' group.
            static ref VERSION_RE: Regex = Regex::new(r"^\s*(?P<leader>[^0-9]*)(?P<marketing>[0-9]+)\.(?P<major>[0-9]+)\.(?P<minor>[0-9]+)(?P<dev>.*)$").unwrap();
        }

        let captured = VERSION_RE
            .captures(s)
            .ok_or_else(|| anyhow!("Invalid version: {:?}", s))?;
        let marketing = captured
            .name("marketing")
            .ok_or_else(|| anyhow!("Could not find marketing version for {:?}", s))?
            .as_str()
            .parse::<u64>()
            .map_err(|e| anyhow!(e.to_string()))?;
        let major = captured
            .name("major")
            .ok_or_else(|| anyhow!("Could not find major version for {:?}", s))?
            .as_str()
            .parse::<u64>()
            .map_err(|e| anyhow!(e.to_string()))?;
        let minor = captured
            .name("minor")
            .ok_or_else(|| anyhow!("Could not find minor version for {:?}", s))?
            .as_str()
            .parse::<u64>()
            .map_err(|e| anyhow!(e.to_string()))?;

        let leader = captured.name("leader");
        let dev = captured.name("dev");
        let mut dev_string = String::new();

        if let Some(leader) = leader {
            dev_string += leader.as_str();
        }

        if let Some(dev) = dev {
            dev_string += dev.as_str();
        }

        Ok(SwiftVersion {
            marketing,
            major,
            minor,
            dev: dev_string,
        })
    }
}

impl SwiftVersion {
    pub const fn new(marketing: u64, major: u64, minor: u64, dev: String) -> Self {
        SwiftVersion {
            marketing,
            major,
            minor,
            dev,
        }
    }

    pub fn parse(text: &str) -> Result<Self, anyhow::Error> {
        SwiftVersion::from_str(text)
    }

    pub fn is_dev(&self) -> bool {
        // A dev string of consisting of a single 'v' character is not
        // considered to be a development version. Due to the way we parse
        // strings (see module description above) the leading and trailing
        // characters of the input string are combined in to this 'dev string'.
        // Our tagging stragegy means that releases are tagged with a leading
        // 'v' (for example v2.2.17) which in turn means a release tag will end
        // up leaving a single 'v' character in this devstring property. We just
        // ignore this special case and don't consider it to be a development
        // version
        (!self.dev.is_empty()) && (self.dev != "v")
    }
}

impl PartialEq for SwiftVersion {
    fn eq(&self, other: &Self) -> bool {
        self.marketing == other.marketing && self.major == other.major && self.minor == other.minor
    }
}
impl Eq for SwiftVersion {}

impl PartialOrd for SwiftVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for SwiftVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        let marketing_ord = self.marketing.cmp(&other.marketing);
        if marketing_ord != Ordering::Equal {
            return marketing_ord;
        }

        let major_ord = self.major.cmp(&other.major);
        if major_ord != Ordering::Equal {
            return major_ord;
        }

        let minor_ord = self.minor.cmp(&other.minor);
        if minor_ord != Ordering::Equal {
            return minor_ord;
        }

        Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success_cases() {
        #[rustfmt::skip]
        let success_test_cases = [
            ("v2.1.0", 2, 1, 0, "v", false),
            ("v2.2.17-develop", 2, 2, 17, "v-develop", true),
            ("v99.99.99-arbitrary-string", 99, 99, 99, "v-arbitrary-string", true),
            ("v1.1.1 including some spaces", 1, 1, 1, "v including some spaces", true),
            ("PiksiMulti-2.0.0.bin", 2, 0, 0, "PiksiMulti-.bin", true),
            ("    v2.0.0", 2, 0, 0, "v", false),
            ("1.2.3.4", 1, 2, 3, ".4", true)
        ];

        for (version_str, marketing, major, minor, dev, isdev) in success_test_cases {
            let ver = SwiftVersion::parse(version_str).unwrap();
            assert_eq!(ver.marketing, marketing);
            assert_eq!(ver.major, major);
            assert_eq!(ver.minor, minor);
            assert_eq!(ver.dev, dev);
            assert_eq!(ver.is_dev(), isdev);
        }
    }

    #[test]
    fn test_fail_cases() {
        let fail_cases = [
            "",
            "alirjaliefjasef",
            "              ",
            "asdf1234fdsa-v1.2.3",
        ];

        for version_str in fail_cases {
            let ver = SwiftVersion::parse(version_str);
            assert!(ver.is_err(), "{:?}", version_str);
        }
    }

    #[test]
    fn test_eq() {
        let expected_eq = [
            ("1.1.1", "1.1.1", true),
            ("1.1.1", "2.2.2", false),
            ("2.2.2", "1.1.1", false),
            ("2.2.2", "2.2.2", true),
            ("2.2.2", "2.2.2-dev", true),
            ("2.2.2-dev", "1.1.1", false),
            ("1.2.1", "1.1.1", false),
            ("1.1.2", "1.1.1", false),
            ("v1.1.1", "1.1.1", true),
        ];

        for (first, second, equality) in expected_eq {
            let lhs = SwiftVersion::parse(first).unwrap();
            let rhs = SwiftVersion::parse(second).unwrap();
            assert_eq!(
                lhs == rhs,
                equality,
                "{:?} == {:?} != {:?}",
                lhs,
                rhs,
                equality
            );
        }
    }

    #[test]
    fn test_cmp() {
        #[rustfmt::skip]
        let expected_cmp = [
            ("1.1.1", "1.1.1", Ordering::Equal),
            ("1.1.2", "1.1.1", Ordering::Greater),
            ("1.2.0", "1.1.1", Ordering::Greater),
            ("2.1.0", "1.1.1", Ordering::Greater),
            ("2.1.0-dev", "1.1.1", Ordering::Greater),
            ("PiksiMulti-2.0.0.bin", "PiksiMulti-3.0.0.bin", Ordering::Less),
            ("v1.0.0", "2.0.0", Ordering::Less)
        ];

        for (first, second, cmp_result) in expected_cmp {
            let lhs = SwiftVersion::parse(first).unwrap();
            let rhs = SwiftVersion::parse(second).unwrap();
            assert_eq!(lhs.cmp(&rhs), cmp_result);
        }
    }
}
