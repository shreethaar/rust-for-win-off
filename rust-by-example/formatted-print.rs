// printing is handled by a series of macros defined in std::fmt 

fn main() {
    println!("{} days",31);   // {} will be replaced with arguments 
    println!("{0}, this is {1}. {1}, this {0}","Alice","Bob");  // positional arguments 

    // named arguments 
    println!("{subject} {verb} {object}",
            object="the lazy dog",
            subject="the quick brown fox",
            verb="jumps over");
    
    // different formatting with format character 
    println!("Base 10:              {}",69420);
    println!("Base 2 (Binary):      {:b}",69420);
    println!("Base 8 (Octal):       {:o}",69420);
    println!("Base 16 (Hexadecimal):{:x}",69420);
        
    // right-justify text 
    println!("{number:>5}",number=1); // four white spaces and a "1", for a total width of 5
    
    // pad numbers with extra zeros 
    println!("{number:0>5}",number=1); 
    println!("{number:0<5}",number=1);

    // can use named arguments in the format specifier by appending a `$`
    println!("{number:0>width$}",number=1,width=5);

    println!("My name is {0},{1} {0}","James","Bond");
    println!("My name is {0},{1} {0}","Bond", "James");

    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt:Display by default. 

    #[allow(dead_code)]
    struct Structure(i32);

    //println!("This struct `{}` won't print...",Structure(3));

    let number: f64=1.0;
    let width: usize=5;
    println!("{number:>width$}");

}

