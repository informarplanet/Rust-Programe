fn main() {
    println!("Hello Cargo !1");
    let mut a=1;
    let b=2;
    println!("The two variable is:- {},{}",a,b);

    //while loop
    while a != 5 {
        println!("The value of a is:- {:?}",a);
        a=a+1;
    }

    // if else
    let inputnum=220;
    if inputnum>200{
        println!("{} is biggest than 200", inputnum);
    }else if inputnum>100 {
        println!("{} is bigger than 100", inputnum);
    }else if inputnum>50{
        println!("{}  is big than 50", inputnum);
    }
    else{
        println!("{} is small value", inputnum);
    }

    // loop

    let mut c=0;
    loop{
        if c == 5 {
            break;
        }else{
            println!("The value of c is:- {}",c);
            c=c+1;
        }
    }

    // create a function Display First & last mane

    let full_name=displayname("Rahul", "Kumar");
    println!("The full name is:- {}",full_name);

    // added two number
    let sumresult=sum(5,3);
    println!("The sum of two number is:- {}",sumresult);

    // minus two number
    let minusresult=minus(8,2);
    println!("The minus of two number is:- {}",minusresult);

    // multiply two number
    let multiplyresult=multi(10.2,6.0);
    println!("the multiply of the two mumber is - {}", multiplyresult);

    // division two number
    let divisionresult=div(10.2,6.0);
    println!("the division of the two mumber is - {}", divisionresult);

    // reminder two number
    let reminderresult=remainder(10.0,2.0);
    println!("the reminder of the two mumber is - {}", reminderresult);

    //calculate
    calculate(10.0,2.0);

}

fn displayname(st: &str, lstname: &str) -> String {
    // let fullname = format!("{} {}", st, lstname);
    // fullname
    st.to_owned()+" "+ lstname
}

fn sum(num1:i32, num2:i32)->i32{
    num1+num2
}
fn minus(num1:i32, num2:i32)->i32{
    num1-num2
}
fn multi(num1:f64, num2:f64)->f64{
    num1*num2
}
fn div(num1:f64, num2:f64)->f64{
    num1/num2
}
fn remainder(num1:f64, num2:f64)->f64{
    num1%num2
}
fn calculate(num1:f64,num2:f64){
    println!("Extra the sum of two number is - {}",num1+num2);
    println!("extra the minus of two number is - {}",num1-num2);
    println!("extra The multiple two number is - {}", num1*num2);
    println!("extra The division two number is - {}", num1/num2);
    println!("extra The remainder two number is - {}", num1%num2);
    println!("extra The remainder two number is - {}", num1%num2);
    println!("extra The remainder two number is - {}", num1%num2);
    
}
