fn get_month() -> i32{
    let mut month = 0;
    let mut input_text = String::new();
    println!("Enter a month number: ");
    std::io::stdin().read_line(&mut input_text).unwrap();
    month = input_text.trim().parse::<i32>().unwrap();
    if month > 12 || month < 1 {
        println!("Month must be between 1 and 12: ");
        month = get_month();
    }
    return month;
}

fn get_year() -> i32{
    let mut year = 0;
    let mut input_text = String::new();
    println!("Enter a year: ");
    std::io::stdin().read_line(&mut input_text).unwrap();
    year = input_text.trim().parse::<i32>().unwrap();
    if year < 1753{
        println!("Year must be later than 1753");
        year = get_year();
    }
    return year;
}

fn is_leap(param_year: i32) -> bool{
    if (param_year % 4 == 0 && param_year % 100 != 0) || (param_year % 400 == 0){
        return true;
    }
    else{
        return false;
    }
}

fn get_num_days(param_month: i32, param_year: i32) -> i32{
    if param_month == 1{
        return 31;
    } else if param_month == 2{
        if is_leap(param_year){return 29;}
        else{return 28;}
    } else if param_month == 3{
        return 31;
    } else if param_month == 4{
        return 30;
    } else if param_month == 5{
        return 31;
    } else if param_month == 6{
        return 30;
    } else if param_month == 7{
        return 31;
    } else if param_month == 8{
        return 31;
    } else if param_month == 9{
        return 30;
    } else if param_month == 10{
        return 31;
    }else if param_month == 11{
        return 30;
    } else{
        return 31;
    }
}

fn get_month_name(param_month: i32){
    if param_month == 1{
        print!("January");
    } else if param_month == 2{
        print!("February");
    } else if param_month == 3{
        print!("March");
    } else if param_month == 4{
        print!("April");
    } else if param_month == 5{
        print!("May");
    } else if param_month == 6{
        print!("June");
    } else if param_month == 7{
        print!("July");
    } else if param_month == 8{
        print!("August");
    } else if param_month == 9{
        print!("September");
    } else if param_month == 10{
        print!("October");
    }else if param_month == 11{
        print!("November");
    } else{
        print!("December");
    }
}

fn get_offset(param_month: i32, param_year: i32) -> i32{
    let mut sum = 0;
    for x in 1753..param_year{
        sum += 365;
        if is_leap(x){
            sum += 1;
        }
    }
    for x in 1..param_month{
        sum += get_num_days(x, param_year);
    }
    let offset = sum % 7;
    return offset;
}

fn display(param_days: i32, param_offset: i32, param_month: i32, param_year: i32){
    println!();
    get_month_name(param_month);
    println!(", {}", param_year);
    println!("  Su  Mo  Tu  We  Th  Fr  Sa");
    if param_offset != 6{
        for _x in -1..param_offset{
            print!("    ");
        }
        for i in 1..=param_days{
            if (i+param_offset) % 7 == 0{
                println!();
            }
            print!("  {:02}", i);
        }
    }
    else if param_offset == 6{
        for i in 1..=param_days{
            if (i+param_offset) % 7 == 0 && i+param_offset != 7{
                println!();
            }
            print!("  {:02}", i);
        }
    }
}

fn main(){
    let month = get_month();
    let year = get_year();
    let days = get_num_days(month, year);
    let offset = get_offset(month, year);
    display(days, offset, month, year)
}