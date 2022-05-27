/// Error should be divided per module.
/// Each module should have its own error code starting number
/// That means a different starting code for error inside contract methods, and inside each of the
/// other structs
/// There's also the need for errors specific to the actions section

/// Error code format is: ERR_{section_number}{error_in_section_number}
/// Error message format is "{smart-contract-name}: {struct or module name}: fn {function_name}: {message_describing_error}"

/// Contract errors - code 0
pub(crate) const ERR_001: &str = "Sample-App: Contract: fn initialize_function: Contract already initialized";
pub(crate) const ERR_002: &str = "Sample-App: Contract: fn only_owner: This function can only be called by the owner";
pub(crate) const ERR_003: &str = "Sample-App: Contract: fn only_guardians_or_owner: This function can only be called by the owner or guardians";
pub(crate) const ERR_004: &str = "Sample-App: Contract: fn add_personalized_struct: This account already created a struct";

/// ComplexPersonalizedStruct (complex_personalized_structs folder) errors - code 1
pub(crate) const ERR_101: &str = "Sample-App: ComplexPersonalizedStruct: fn change_in_map: AccountId not found in account_relationship_map";
pub(crate) const ERR_102: &str = "Sample-App: ComplexPersonalizedStruct: generic: Method not implemented for this enum variant";


/// PersonalizedStructs (personalized_structs.rs) errors - code 2
pub(crate) const ERR_201: &str = "Sample-App: PersonalizedStruct: fn change_data: method not implemented for this enum variant";
