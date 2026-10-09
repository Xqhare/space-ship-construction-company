use nemesis::NemesisError;
use std::fmt;

/// Crate-level Result type using `NemesisError`
pub type SSCCResult<T> = Result<T, NemesisError>;

#[derive(Debug)]
pub enum SSCCError {
    Generic(String),
    ComponentBuilder(String),
    CapabilityBuilder(String),
    ShipClassBuilder(String),
    Io(std::io::Error),
}

pub fn new_component_builder_error() -> NemesisError {
    SSCCError::ComponentBuilder(
        "Component Builder was unable to successfully create a component".to_string(),
    )
    .into()
}
pub fn new_capability_builder_error() -> NemesisError {
    SSCCError::CapabilityBuilder(
        "Capability Builder was unable to successfully create a capability".to_string(),
    )
    .into()
}
pub fn new_ship_class_builder_error() -> NemesisError {
    SSCCError::ShipClassBuilder(
        "Ship Class Builder was unable to successfully create a ship class".to_string(),
    )
    .into()
}

impl std::error::Error for SSCCError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SSCCError::Io(err) => Some(err),
            _ => None,
        }
    }
    fn description(&self) -> &str {
        match self {
            SSCCError::Generic(msg) => msg,
            SSCCError::ComponentBuilder(msg) => msg,
            SSCCError::CapabilityBuilder(msg) => msg,
            SSCCError::ShipClassBuilder(msg) => msg,
            SSCCError::Io(_) => "IO Error",
        }
    }
}

impl From<SSCCError> for NemesisError {
    fn from(err: SSCCError) -> Self {
        let (ctx, source) = match err {
            SSCCError::Generic(ref msg) => (msg.clone(), "Generic"),
            SSCCError::ComponentBuilder(ref msg) => (msg.clone(), "ComponentBuilder"),
            SSCCError::CapabilityBuilder(ref msg) => (msg.clone(), "CapabilityBuilder"),
            SSCCError::ShipClassBuilder(ref msg) => (msg.clone(), "ShipClassBuilder"),
            SSCCError::Io(ref err) => (err.to_string(), "std::io::Error"),
        };
        // NOTE: This is a bit of a hack, but it works
        // While leaking memory on every error creation is bad, I am expecting to not create a
        // large amount of errors.
        // Further, if all text remains ASCII, each leaked string will take around 40-50 bytes.
        //
        // Of course this will mean that the longer the program runs, the more memory will
        // be consumed, relative to the amount of errors raised.
        //
        // For general release unacceptable, for now its fine
        let tmp_box = Box::new(format!("SSCC Error raised in: '{}'", source));
        let leaked_static_ptr: &'static str = Box::leak(tmp_box);

        NemesisError::new(leaked_static_ptr, err).add_ctx(ctx)
    }
}

impl fmt::Display for SSCCError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SSCCError::Generic(msg) => write!(f, "{}", msg),
            SSCCError::ComponentBuilder(msg) => write!(f, "{}", msg),
            SSCCError::CapabilityBuilder(msg) => write!(f, "{}", msg),
            SSCCError::ShipClassBuilder(msg) => write!(f, "{}", msg),
            SSCCError::Io(err) => write!(f, "{}", err),
        }
    }
}

impl From<std::io::Error> for SSCCError {
    fn from(err: std::io::Error) -> Self {
        SSCCError::Io(err)
    }
}
