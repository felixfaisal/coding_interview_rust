/* IsUnique - Implement an algorithm to determine if a string has all unique characters, and not use any additional data structures */ 
/* PsuedoCode
START
    int isUnique(String string):
        int[128] arr; 
        for x in string: 
            if int[int(x)].exists():
                return -1 
            else: 
                int[int(x)] = 1 
        return 0
END 
Time Complexity: 0(N)
*/ 
fn main(){
    if isUnique("abcde"){
        println!("Unique");
    }
    else{
        println!("Not Unique")
    }
}
fn isUnique(some_string: &str) -> bool{
    let mut char_arr: [u8;128] = [0;128];
    for x in some_string.bytes(){
        if char_arr[x as usize] == 1{
            return false;
        }
        else{
            char_arr[x as usize] = 1;
        }
    }
    return true;
}