//ust.rs

pub fn arrayfill<T>(array:& mut [T],into:T)
    where T:Copy
{
    for run in array.iter_mut(){
        *run=into;
    }
}

/*
pub fn arrayfill<T>(array:& mut [T],into:&mut T)
{
    for mut run in array.iter_mut(){
        run=into;
    }
}
*/