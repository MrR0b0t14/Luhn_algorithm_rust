/// Check a Luhn checksum.
fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

pub fn is_valid(code: &str) -> bool {
    if code.trim().len() < 2 {
        return false;
    }
    let mut s = code.to_string();
    remove_whitespace(&mut s);
    let mut v = Vec::new();

    for ch in s.trim().chars() {
        if ch.is_digit(10){
            v.push(ch.to_digit(10).unwrap());
        }else { return false }
    }

    let len = v.len();

    for (pos, ch) in v.iter_mut().enumerate(){
        match ch{
            //_ if !ch.is_digit(10) => return false,
            _ if len%2 == 0 && pos%2 == 0 => {
                if (*ch)*2 > 9{
                    *ch = (*ch)*2-9;
                }else {
                    *ch = (*ch)*2;
                }
            },
            _ if len%2 != 0 && pos%2 != 0 => {
                if (*ch)*2 > 9{
                    *ch = (*ch)*2-9;
                }else {
                    *ch = (*ch)*2;
                }
            },
            _ => (),
        }
    }
    let mut acc = 0;
    for num in v {
         acc += num;
    }
    if acc%10 == 0 {
        true
    }else { false }
}
