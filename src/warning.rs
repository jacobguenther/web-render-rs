// File: src/warning.rs
// Author: Jacob Guenther
// Date created: March 2021
// License: AGPLv3
//
// Description:

use std::fmt;

#[derive(Clone, Debug)]
pub enum Warning {
	ShaderWarning(ShaderWarning),
	Custom(String),
}
impl std::fmt::Display for Warning {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Warning::ShaderWarning(shader_warning) => {
				write!(f, "{}", shader_warning)
			}
			Warning::Custom(warning) => write!(f, "{}", warning),
		}
	}
}
impl From<&ShaderWarning> for Warning {
	fn from(shader_warning: &ShaderWarning) -> Warning {
		Warning::ShaderWarning(shader_warning.clone())
	}
}

#[derive(Clone, Debug)]
pub enum ShaderWarning {
	AttributeNotFound(String),
	UniformNotFound(String),
}
impl std::fmt::Display for ShaderWarning {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			ShaderWarning::AttributeNotFound(attribute) => {
				write!(f, "Warning: Attribute not found {}", attribute)
			}
			ShaderWarning::UniformNotFound(uniform) => {
				write!(f, "Warning: Uniform not found {}", uniform)
			}
		}
	}
}
