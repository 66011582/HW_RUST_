use std::env;

fn main() {
    // THIS PROGRAM MUST KEY ONLY INTEGER
    let score : Vec<String> = env::args().collect();
    // I think this solution don't over kill me like clap.
        if score.len() < 2
            {
                println!("please key your score , I want to know \
                your TERRIBLE!! grade");
            }
        
            if score.len() > 2
            {
                println!("yor don't know how to use tjis program \
                right?");
            }

            if score.len() == 2
            {
                let grade: &i32 = &score[1].parse().unwrap_or(0);
                //if grade > &100 {print!("Invalid score");}
                //if grade > &94 && grade < &101 {print!("Excellent with A+");}
                //if grade > &80 && grade < &95 {print!("A");}
                //if grade > &70 && grade < &81 {print!("B");}
                //if grade > &60 && grade < &71 {print!("C");}
                //if grade > &49 && grade < &61 {print!("D");}
                //if grade > &-1 && grade < &50 {print!("Failed with F");}
                //if grade < &0 {print!("Invalid score");}

                if grade > &100 {print!("Invalid score");}
                else if  grade > &94 {print!("Excellent with A+");}
                else if  grade > &80 {print!("A");}
                else if  grade > &70 {print!("B");}
                else if  grade > &60 {print!("C");}
                else if  grade > &49 {print!("D");}
                else if  grade > &-1 {print!("Failed with F");}
                else if  grade < &0 {print!("Invalid score");}
            }
}
