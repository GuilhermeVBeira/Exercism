pub fn raindrops(n: u32) -> String {
    if n % 3 == 0 {
        return "Pling".to_string()
    }else if n % 5 == 0 {
        return "Plang".to_string()
    }else if n % 7 == 0  {
        return "Plong".to_string()
    }else {
        return n.to_string()
    }
}
