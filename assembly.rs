//! Эквивалент метаданных сборки Stardew Valley 1.6.15.

pub const ASSEMBLY_VERSION: &str = "1.6.15.24356";
pub const ASSEMBLY_COMPANY: &str = "ConcernedApe";
pub const ASSEMBLY_COPYRIGHT: &str = "Copyright © ConcernedApe 2013";
pub const ASSEMBLY_CONFIGURATION: &str = "Release";
pub const TARGET_FRAMEWORK: &str = ".NETCoreApp,Version=v6.0";
pub const INTERNALS_VISIBLE_TO: &[&str] = &["StardewModdingAPI"];
pub const COMPILATION_RELAXATIONS: u32 = 8;
pub const RUNTIME_COMPATIBILITY_WRAP_NON_EXCEPTION_THROWS: bool = true;
pub const DEBUGGABLE_IGNORE_SYMBOL_STORE_SEQUENCE_POINTS: bool = true;

pub fn wrap_non_exception_throws<F, T>(f: F) -> Result<T, Box<dyn std::any::Any + Send>>
where
    F: FnOnce() -> T + std::panic::UnwindSafe,
{
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(f))
        .map_err(|e| e)
}
