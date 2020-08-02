//oft.rs

pub fn gout<T:std::fmt::Debug>(target:T){
    println!("{:?}",target);
}

pub fn swap<T>(first:& mut T,second:& mut T)
    where T:Copy
{    
    let temp=*first;
    *first=*second;   
    *second=temp;
}

pub fn clone_swap<T>(first:&mut T,second:&mut T)
    where T:Clone
{
    let temp=first.clone();
    *first=second.clone();
    *second=temp;
}

