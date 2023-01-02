fn main() {
    use human_sort_patched::sort;
    let mut arr = ["187-4-37-103.paebv701.e.brasiltelecom.net.br.", "5136541026.e.brasiltelecom.net.br."];
    sort(&mut arr);
}

#[cfg(test)]
mod tests {
    #[test] fn main() { super::main() }
}