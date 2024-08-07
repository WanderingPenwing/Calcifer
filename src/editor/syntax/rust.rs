use super::Syntax;
use std::collections::BTreeSet;

impl Syntax {
    pub fn rust() -> Self {
        Syntax {
            language: "Rust",
            case_sensitive: true,
            comment: "//",
            comment_multiline: ["/*", "*/"],
            keywords: BTreeSet::from([
                "as", "break", "const", "continue", "crate", "else", "enum", "extern", "fn", "for",
                "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
                "return", "self", "struct", "super", "trait", "type", "use", "where", "while",
                "async", "await", "abstract", "become", "box", "do", "final", "macro", "override",
                "priv", "typeof", "unsized", "virtual", "yield", "try", "unsafe", "dyn",
            ]),
            types: BTreeSet::from([
                "Option",
                "Result",
                "Error",
                "Box",
                "Cow",
                // Primitives
                "bool",
                "i8",
                "u8",
                "i16",
                "u16",
                "i32",
                "u32",
                "i64",
                "u64",
                "i128",
                "u128",
                "isize",
                "usize",
                "f32",
                "f64",
                "char",
                "str",
                "String",
                // STD Collections
                "Vec",
                "BTreeMap",
                "BTreeSet",
                "BTreeMap",
                "BTreeSet",
                "VecDeque",
                "BinaryHeap",
                "LinkedList",
                // RC
                "Rc",
                "Weak",
                "LazyCell",
                "SyncUnsafeCell",
                "BorrowErrorl",
                "BorrowMutErrorl",
                "Celll",
                "OnceCelll",
                "Refl",
                "RefCelll",
                "RefMutl",
                "UnsafeCell",
                "Exclusive",
                "LazyLock",
                // ARC
                "Arc",
                "Barrier",
                "BarrierWaitResult",
                "Condvar",
                "Mutex",
                "MutexGuard",
                "Once",
                "OnceLock",
                "OnceState",
                "PoisonError",
                "RwLock",
                "RwLockReadGuard",
                "RwLockWriteGuard",
                "WaitTimeoutResult",
                "Weak",
            ]),
            special: BTreeSet::from(["Self", "static", "true", "false"]),
        }
    }
}
