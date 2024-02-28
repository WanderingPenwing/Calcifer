use super::Syntax;
use std::collections::BTreeSet;

impl Syntax {
	pub fn javascript() -> Syntax {
		Syntax {
			language: "Javascript",
			case_sensitive: true,
			comment: "//",
			comment_multiline: ["/*", "*/"],
			keywords: BTreeSet::from([
				"&&", "||", "!", "let", "var", "abstract", "arguments", "await", "break", "case", "catch", "class", "const", "continue",
				"debugger", "default", "delete", "do", "else", "enum", "eval", "export", "extends", "final", "finally", "for", "function",
				"goto", "if", "implements", "import", "in", "instanceof", "interface", "let", "native", "new", "package", "private", "protected",
				"public", "return", "static", "super", "switch", "synchronized", "this","throw", "throws", "transient", "try", "typeof", 
				"var", "volatile", "while", "with", "yield",
			]),
			types: BTreeSet::from([
				"Boolean", "Number", "BigInt", "Undefined", "Null", "String", "Symbol", "byte", "char", "float", "int", "long", "short", "void",
			]),
			special: BTreeSet::from(["false", "null", "true"]),
		}
	}
}
