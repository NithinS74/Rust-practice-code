pub fn infix_to_postfix() {
    let eq = "K + L - M*N + (O^P) * W/U/V * T + Q";
    let mut posteq = String::new();
    let mut stack = String::new();
    for i in eq.chars() {
        if i == '(' {
            stack.push(i);
        } else if i == ' ' {
            continue;
        } else if i == ')' {
            loop {
                if let Some(c) = stack.pop() {
                    if c == '(' {
                        break;
                    } else {
                        posteq.push(c);
                    }
                } else {
                    break;
                }
            }
        } else if precedence(i) == -1 {
            posteq.push(i);
        } else {
            if let Some(ps) = stack.chars().last() {
                if precedence(i) > precedence(ps) {
                    stack.push(i);
                } else {
                    loop {
                        if let Some(c) = stack.pop() {
                            if c == '(' {
                                break;
                            } else if precedence(ps) >= precedence(i) {
                                posteq.push(c);
                            }
                        } else {
                            break;
                        }
                    }
                    stack.push(i);
                }
            } else {
                stack.push(i);
            }
        }
    }

    loop {
        if let Some(c) = stack.pop() {
            if c == '(' {
                continue;
            } else {
                posteq.push(c);
            }
        } else {
            break;
        }
    }
    println!("{posteq}");
}

fn precedence(i: char) -> i32 {
    match i {
        '(' => 0,
        '*' | '/' => 4,
        '+' | '-' => 2,
        '^' => 3,
        _ => -1,
    }
}
