//! Genealogy shader kernel for SigmaOS.
//!
//! Provides shader compilation, linking, and rendering pipeline for genealogy
//! visualization. Simulates GPU shader operations without actual hardware.

use std::collections::HashMap;
use std::fmt;

/// Errors for shader operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShaderError {
    ShaderNotFound(String),
    CompilationFailed(String),
    InvalidSource(String),
    LinkFailed(String),
}

impl fmt::Display for ShaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ShaderNotFound(s) => write!(f, "shader not found: {s}"),
            Self::CompilationFailed(m) => write!(f, "compilation failed: {m}"),
            Self::InvalidSource(m) => write!(f, "invalid source: {m}"),
            Self::LinkFailed(m) => write!(f, "link failed: {m}"),
        }
    }
}

impl std::error::Error for ShaderError {}

/// Shader types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderType {
    Vertex,
    Fragment,
    Compute,
}

impl ShaderType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Vertex => "vertex",
            Self::Fragment => "fragment",
            Self::Compute => "compute",
        }
    }
}

/// Shader source.
#[derive(Debug, Clone, PartialEq)]
pub struct ShaderSource {
    pub name: String,
    pub shader_type: ShaderType,
    pub source: String,
}

impl ShaderSource {
    pub fn new(name: impl Into<String>, shader_type: ShaderType, source: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            shader_type,
            source: source.into(),
        }
    }

    pub fn validate(&self) -> Result<(), ShaderError> {
        if self.name.trim().is_empty() {
            return Err(ShaderError::InvalidSource("shader name must not be empty".into()));
        }
        if self.source.trim().is_empty() {
            return Err(ShaderError::InvalidSource("shader source must not be empty".into()));
        }
        Ok(())
    }
}

/// Compiled shader.
#[derive(Debug, Clone, PartialEq)]
pub struct CompiledShader {
    pub name: String,
    pub shader_type: ShaderType,
    pub compiled_source: String,
}

/// Shader program (linked shaders).
#[derive(Debug, Clone, PartialEq)]
pub struct ShaderProgram {
    pub name: String,
    pub vertex_shader: String,
    pub fragment_shader: String,
    pub compute_shader: Option<String>,
}

/// Shader manager.
#[derive(Debug, Clone, Default)]
pub struct ShaderManager {
    shaders: HashMap<String, CompiledShader>,
    programs: HashMap<String, ShaderProgram>,
    next_id: u64,
}

impl ShaderManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn compile(&mut self, source: ShaderSource) -> Result<String, ShaderError> {
        source.validate()?;
        
        // Simulate compilation
        let compiled_source = format!("// compiled {} shader: {}\n{}", source.shader_type.as_str(), source.name, source.source);
        
        self.next_id += 1;
        let id = format!("shader-{}", self.next_id);
        
        let shader = CompiledShader {
            name: source.name.clone(),
            shader_type: source.shader_type,
            compiled_source,
        };
        
        self.shaders.insert(id.clone(), shader);
        Ok(id)
    }

    pub fn get_shader(&self, id: &str) -> Option<&CompiledShader> {
        self.shaders.get(id)
    }

    pub fn remove_shader(&mut self, id: &str) -> Result<(), ShaderError> {
        self.shaders.remove(id)
            .map(|_| ())
            .ok_or_else(|| ShaderError::ShaderNotFound(id.to_string()))
    }

    pub fn link_program(&mut self, name: impl Into<String>, vertex_id: &str, fragment_id: &str, compute_id: Option<&str>) -> Result<String, ShaderError> {
        let vertex = self.get_shader(vertex_id)
            .ok_or_else(|| ShaderError::ShaderNotFound(vertex_id.to_string()))?;
        let fragment = self.get_shader(fragment_id)
            .ok_or_else(|| ShaderError::ShaderNotFound(fragment_id.to_string()))?;
        
        if vertex.shader_type != ShaderType::Vertex {
            return Err(ShaderError::LinkFailed("first shader must be vertex".into()));
        }
        if fragment.shader_type != ShaderType::Fragment {
            return Err(ShaderError::LinkFailed("second shader must be fragment".into()));
        }
        
        let compute_name = if let Some(cid) = compute_id {
            let compute = self.get_shader(cid)
                .ok_or_else(|| ShaderError::ShaderNotFound(cid.to_string()))?;
            if compute.shader_type != ShaderType::Compute {
                return Err(ShaderError::LinkFailed("third shader must be compute".into()));
            }
            Some(compute.name.clone())
        } else {
            None
        };
        
        let vertex_name = vertex.name.clone();
        let fragment_name = fragment.name.clone();
        
        self.next_id += 1;
        let id = format!("program-{}", self.next_id);
        
        let program = ShaderProgram {
            name: name.into(),
            vertex_shader: vertex_name,
            fragment_shader: fragment_name,
            compute_shader: compute_name,
        };
        
        self.programs.insert(id.clone(), program);
        Ok(id)
    }

    pub fn get_program(&self, id: &str) -> Option<&ShaderProgram> {
        self.programs.get(id)
    }

    pub fn list_shaders(&self) -> Vec<&CompiledShader> {
        self.shaders.values().collect()
    }

    pub fn list_programs(&self) -> Vec<&ShaderProgram> {
        self.programs.values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_shader_aaa() {
        // Arrange
        let mut mgr = ShaderManager::new();
        let source = ShaderSource::new(
            "genealogy-vertex",
            ShaderType::Vertex,
            "void main() { gl_Position = vec4(0.0); }"
        );

        // Act
        let res = mgr.compile(source);

        // Assert
        assert!(res.is_ok());
        assert_eq!(mgr.list_shaders().len(), 1);
    }

    #[test]
    fn test_link_program_aaa() {
        // Arrange
        let mut mgr = ShaderManager::new();
        let vertex = mgr.compile(ShaderSource::new(
            "genealogy-vertex",
            ShaderType::Vertex,
            "void main() { gl_Position = vec4(0.0); }"
        )).unwrap();
        let fragment = mgr.compile(ShaderSource::new(
            "genealogy-fragment",
            ShaderType::Fragment,
            "void main() { gl_FragColor = vec4(1.0); }"
        )).unwrap();

        // Act
        let res = mgr.link_program("genealogy-program", &vertex, &fragment, None);

        // Assert
        assert!(res.is_ok());
        assert_eq!(mgr.list_programs().len(), 1);
    }

    #[test]
    fn test_shader_not_found_aaa() {
        // Arrange
        let mgr = ShaderManager::new();

        // Act
        let res = mgr.get_shader("nonexistent");

        // Assert
        assert!(res.is_none());
    }
}
