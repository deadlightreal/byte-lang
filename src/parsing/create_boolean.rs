use crate::datatypes::Arch;

pub fn create_boolean(arch : Arch, bool_val : u8) -> String {
    match arch {
        Arch::ARM64 => {
            return format!(r#"
    mov X1, #{}
    str X1, [sp]
    sub sp, sp, #16

"#, bool_val);
        },
        Arch::X86 => {
            return format!(r#"
    mov X1, #{}
    str X1, [sp]
    sub sp, sp, #16

"#, bool_val);
        },
    };
}
