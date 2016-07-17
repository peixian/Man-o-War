use std::ascii::AsciiExt;

static ALPHABET: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
                               'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
                               's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

fn verify(element: &str, sym: &str) -> bool{
    let symbols: Vec<String> = get_symbols(element);
    println!("{:?}", symbols);
    for possible_symbol in &symbols {
        if possible_symbol == sym {
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

fn alphabet_loc(letter: char) -> u8 {
    let ll = letter.to_ascii_lowercase();
    for i in 0..26 {
        if ALPHABET[i] == ll {
            return i as u8;
        }
    }
    //shouldn't ever happen
    return 0;
}

fn alphabetical_symbol(symbols: Vec<String>) -> Vec<String> {
    let mut alphabetical_symbols: Vec<String> = Vec::new();
    for symbol in symbols {
        let symbol_chars: Vec<char> = symbol.chars().collect();
        let symbol_locs: Vec<_> = symbol_chars.iter().map(|&x| alphabet_loc(x)).collect();
        if symbol_locs[0] < symbol_locs[1] {
            alphabetical_symbols.push(symbol);
        }
    }
    return alphabetical_symbols;
}

fn alphabetical(element: &str) -> Vec<String> {
    let symbols: Vec<String> = get_symbols(element);
    let alphabetical_symbols: Vec<String> = alphabetical_symbol(symbols);
    return alphabetical_symbols;
}

//fn get_multichar_symbols(element: &str, char_count: &i32) -> Vec<String> {
//}

fn main() {
    println!("{}", verify("Zuulon", "uu"));
    println!("{:?}", alphabetical("Zuulon"))
}
