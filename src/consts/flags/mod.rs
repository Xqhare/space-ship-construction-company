pub mod capabilities;
pub const TEST_FLAG: &str = "test_flag";
/// For internal use only.
///
/// E.g. As a stand in for Builders that need an allocated String.
///
/// If used, ensure that this cannot be encountered by users.
///
/// Example usage can be found within [`CapabilityBuilder`]
pub const INTERNAL_NONE: &str = "internal_none";
