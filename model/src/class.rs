use enumset::{EnumSet, EnumSetType};

#[derive(Default, Clone, Debug)]
pub struct JvmClass {
    pub version: ClassVersion,
    pub constants: ClassConstants,

    pub access_flags: EnumSet<ClassAccessFlag>,
    pub this_class: String,
    pub super_class: Option<String>,
    pub interfaces: Vec<String>,

    pub fields: ClassFields,
    pub methods: ClassMethods,
    pub attributes: ClassAttributes,
}

#[derive(Default, Clone, Debug)]
pub struct ClassVersion {
    pub major: u16,
    pub minor: u16,
}

pub type ClassConstants = Vec<ClassConstant>;

#[derive(Clone, Debug)]
pub enum ClassConstant {
    // This will be the first element of the constants pool for each class reader. This enables
    // easier handling of index parameters since Java class indexes are not 0 based.
    Unused(),

    // name_index
    Class(String),

    // class_name, field_name, type_descriptor
    Fieldref(String, String, TypeSignature),

    // class_name, method_name, method_signature
    Methodref(String, String, MethodSignature),

    // class_name, method_name, method_signature
    InterfaceMethodref(String, String, MethodSignature),

    // string_index
    String(String),

    // Value
    Integer(i32),

    // Value
    Float(f32),

    // Value
    Long(i64),

    // Value
    Double(f64),

    // name, descriptor
    MethodNameAndType(String, MethodSignature),
    FieldNameAndType(String, TypeSignature),

    // Value
    Utf8(String),

    // reference_kind, reference_index
    // See https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-5.html#jvms-5.4.3.5
    // MethodHandle(u8, u16),

    // descriptor_index
    MethodType(MethodSignature),

    // bootstrap_method_attr_index, name_and_type_index
    Dynamic(u16, String, MethodSignature),

    // bootstrap_method_attr_index, name_and_type_index
    InvokeDynamic(u16, String, MethodSignature),

    NotImplemented,
}

#[derive(EnumSetType, Debug)]
pub enum ClassAccessFlag {
    Public,
    Private,
    Protected,
    Final,
    Super,
    Interface,
    Abstract,
    Synthetic,
    Annotation,
    Enum,
    Module,
}

#[derive(EnumSetType, Debug)]
pub enum FieldAccessFlag {
    /// Declared public; may be accesse from outside its package.
    Public,

    /// Declared private; usable only within the defining class.
    Private,

    /// Declared protected; may be accessed within subclasses.
    Protected,

    /// Declared static.
    Static,

    /// Declared final; never directly assigned to after object construction.
    Final,

    /// Declared volatile; cannot be cached.
    Volatile,

    /// Declared transient; not written or read by a persistent object manager.
    Transient,

    /// Declared synthetic; not present in the source code.
    Synthetic,

    /// Declared as an annotation type.
    Annotation,

    /// Declared as an element of an enum.
    Enum,
}

pub type ClassFields = Vec<ClassField>;

#[derive(Default, Clone, Debug)]
pub struct ClassField {
    pub access_flags: EnumSet<FieldAccessFlag>,
    pub name: String,
    pub descriptor: TypeSignature,
    pub attributes: ClassAttributes,
}

#[derive(EnumSetType, Debug)]
pub enum MethodAccessFlag {
    /// Declared public; may be accessed from outside its package.
    Public,

    /// Declared private; accessible only within the defining class.
    Private,

    /// Declared protected; may be accessed within subclasses.
    Protected,

    /// Declared static.
    Static,

    /// Declared final; must not be overridden.
    Final,

    /// Declared synchronized; invocation is wrapped by a monitor use.
    Synchronized,

    /// A bridge method, generated by the compiler.
    Bridge,

    /// Declared with variable number of arguments.
    Varargs,

    /// Declared native; implemented in a language other than Java
    Native,

    /// Declared abstract; no implementation is provided.
    Abstract,

    /// Declared strictfp; floating-point mode is FP-strict.
    Strict,

    /// Declared synthetic; not present in the source code.
    Synthetic,
}

pub type ClassMethods = Vec<ClassMethod>;

#[derive(Default, Clone, Debug)]
pub struct ClassMethod {
    pub access_flags: EnumSet<MethodAccessFlag>,
    pub name: String,
    pub descriptor: MethodSignature,
    pub attributes: ClassAttributes,
}

pub type ClassAttributes = Vec<ClassAttribute>;

#[derive(Clone, Debug)]
pub enum ClassAttribute {
    Code(Code),
    LineNumberTable(Vec<SourceLineNumber>),
    SourceFile(String),
    Exceptions(Vec<u16>),
    ConstantValue(ClassConstant),
    BootstrapMethods(Vec<BootstrapMethod>),
    NotImplemented,
}

#[derive(Default, Clone, Debug)]
pub struct Code {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
    pub exception_table: Vec<ExceptionTable>,
    pub attributes: Vec<ClassAttribute>,
}

#[derive(Default, Clone, Debug)]
pub struct ExceptionTable {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    /// Some(String): String identifies the class of exceptions that this exception handler is designated to catch.
    /// None: this exception handler is called for all exceptions.
    pub catch_type: Option<String>,
}

#[derive(Default, Clone, Debug)]
pub struct SourceLineNumber {
    pub start_pc: u16,
    pub line_number: u16,
}

#[derive(Default, Clone, Debug)]
pub struct BootstrapMethod {
    pub method_ref: u16,
    pub arguments: Vec<u16>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeSignature {
    Void,
    Boolean,
    Byte,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Class(String),
    Array(Box<TypeSignature>),
}

impl Default for TypeSignature {
    fn default() -> TypeSignature {
        TypeSignature::Void
    }
}

#[derive(Default, Clone, Debug)]
pub struct MethodSignature {
    pub parameters: Vec<TypeSignature>,
    pub return_type: TypeSignature,
}

impl std::fmt::Display for TypeSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            TypeSignature::Void => "V".to_string(),
            TypeSignature::Boolean => "Z".to_string(),
            TypeSignature::Byte => "B".to_string(),
            TypeSignature::Char => "C".to_string(),
            TypeSignature::Short => "S".to_string(),
            TypeSignature::Int => "I".to_string(),
            TypeSignature::Long => "J".to_string(),
            TypeSignature::Float => "F".to_string(),
            TypeSignature::Double => "D".to_string(),
            TypeSignature::Class(class_path) => format!("L{};", class_path),
            TypeSignature::Array(inner_type) => format!("[{}", inner_type),
        };
        write!(f, "{}", text)
    }
}

impl std::fmt::Display for MethodSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params = self
            .parameters
            .iter()
            .map(|p| format!("{}", p))
            .collect::<Vec<_>>()
            .join("");
        write!(f, "({}){}", params, self.return_type)
    }
}
