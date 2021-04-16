use pwn::elf::Elf;
use std::path::Path;

// `gcc binary.c -o binary -no-pie`
const PATH: &str = "tests/compile/binary";
// `gcc binary.c -o binary -pie`
const PATH2: &str = "tests/compile/binary_pie";

#[test]
pub fn test_elf_syms() {
    // Compile binary.c to an ELF for these tests ()
    assert!(Path::new(PATH).exists());
    let e = Elf::new(PATH);
    println!("{}", e.sym);
    println!("{}", e.sym["main"])
}

#[test]
#[should_panic]
pub fn test_elf_invalid_sym() {
    assert!(Path::new(PATH).exists());
    let e = Elf::new(PATH);
    println!("{}", e.sym["blahblah"]);
}

#[test]
pub fn test_pie() {
    assert!(Path::new(PATH).exists());
    let e = Elf::new(PATH);
    assert_eq!(e.pie, false);
    let e2 = Elf::new(PATH2);
    assert!(e2.pie);
}