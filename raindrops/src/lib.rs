pub fn raindrops(n: usize) -> String {
    let mut plngs: Vec<String> = Vec::new();
    if n % 3 == 0 {
        plngs.push("Pling".to_string());
    };
    if n % 5 == 0 {
        plngs.push("Plang".to_string());
    };
    if n % 7 == 0 {
        plngs.push("Plong".to_string());
    };
    if plngs.len() == 0 {
        return n.to_string();
    } else {
        return plngs.join("");
    };
}
