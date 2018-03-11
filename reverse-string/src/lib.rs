pub fn reverse(s: &str) -> String {
    let mut ssplit: Vec<&str> = s.split("").collect();
    ssplit.reverse();
    let rev: String = ssplit.join("");
    rev
}


