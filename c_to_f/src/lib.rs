
//take in temp in fahrenheight and output in celcius
pub fn calculate_celcius(temp_f: i64) -> i64 {
    let temp_c =  ((temp_f - 32) * 5/9);
    println!("{} in fahrenheight is {} in celcius", temp_f, temp_c);
    temp_c
}

//take in temp in celcius and output fahrenheight
pub fn calculate_fahrenheight(temp_c: i64) -> i64 {
    let temp_f = ((temp_c * 9/5) + 32);
    println!("{} in celcius is {} in fahrenheight", temp_c, temp_f);
    temp_f
}

#[cfg(test)]
mod tests {
    use super::*;


}
