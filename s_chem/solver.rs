fn verify(element: &str, sym: &str) -> bool{
    let symbols: Vec<String> = get_symbols(element);
    println!("{:?}", symbols);
    for possible_symbol in &symbols {
        if possible_symbol == sym{
            return true;
        }
    }
    return false;
}

fn get_symbols(element: &str) -> Vec<String> {
    let chars: Vec<char> = element.chars().collect();
    let mut symbols: Vec<String> = Vec::new();
    for i in 0..chars.len() {
        if i+1 < chars.len() {
            let slice: Vec<char> = chars[i+1..chars.len()].to_vec();
            let syms: Vec<_> = slice.iter().map(|&x| chars[i].to_string() + &x.to_string()).collect();
            symbols.extend(syms.iter().cloned());
        }
    }
    symbols.sort();
    symbols.dedup();
    return symbols
}

fn alphabetical_symbol(symbols: &Vec<String>) -> String {
}

fn get_multichar_symbols(element: &str, char_count: &i32) -> Vec<String> {
}

fn main() {
    println!("{}", verify("Zuulon", "uu"));
}
