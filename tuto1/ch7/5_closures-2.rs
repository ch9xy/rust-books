fn main () {

    let square = |num| num * num;
    let x = 5;
    square(x);

    let y = 105.5;
    square(y); // WON't RUN 
    // CODE COMPILES ONLY IF YOU COMMENT OUT LINE 5 OR 8. CLOSURE ACCEPTS ONE TYPE AT A TIME
}