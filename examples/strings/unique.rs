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
    let mut str_name = "abcde"; 
    for x in str_name{
        println("{}", x);
    }
}