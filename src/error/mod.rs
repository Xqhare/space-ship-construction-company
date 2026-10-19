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
    Startup(String),
    Io(std::io::Error),
}

pub fn new_generic_error(msg: &str) -> NemesisError {
    SSCCError::Generic(msg.to_string()).into()
}

pub fn new_startup_error(msg: &str) -> NemesisError {
    SSCCError::Startup(msg.to_string()).into()
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
            SSCCError::Startup(msg) => msg,
            SSCCError::Io(_) => "IO Error",
        }
    }
}

const ERROR_TITLE: &str = "Space Ship Construction Company Error";

impl From<SSCCError> for NemesisError {
    fn from(err: SSCCError) -> Self {
        let (ctx, source) = match err {
            SSCCError::Generic(ref msg) => (msg.clone(), "Generic"),
            SSCCError::ComponentBuilder(ref msg) => (msg.clone(), "ComponentBuilder"),
            SSCCError::CapabilityBuilder(ref msg) => (msg.clone(), "CapabilityBuilder"),
            SSCCError::ShipClassBuilder(ref msg) => (msg.clone(), "ShipClassBuilder"),
            SSCCError::Startup(ref msg) => (msg.clone(), "Startup"),
            SSCCError::Io(ref err) => (err.to_string(), "std::io::Error"),
        };
        let source = format!("SSCC Error raised in: '{}'", source);

        NemesisError::new(ERROR_TITLE, err)
            .add_ctx(source)
            .add_ctx(ctx)
    }
}

impl fmt::Display for SSCCError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SSCCError::Generic(msg) => write!(f, "{}", msg),
            SSCCError::ComponentBuilder(msg) => write!(f, "{}", msg),
            SSCCError::CapabilityBuilder(msg) => write!(f, "{}", msg),
            SSCCError::ShipClassBuilder(msg) => write!(f, "{}", msg),
            SSCCError::Startup(msg) => write!(f, "{}", msg),
            SSCCError::Io(err) => write!(f, "{}", err),
        }
    }
}

impl From<std::io::Error> for SSCCError {
    fn from(err: std::io::Error) -> Self {
        SSCCError::Io(err)
    }
}
