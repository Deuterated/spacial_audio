// a function that takes as input the position of a noise-producing game object relative to a player
// and calculates appropriate left-right audio mix settings
//
//
//

use std::io;
use std::f32::consts::PI;


fn main() {
    //println!("Hello, world!");

//taking input from console is mostly for testing the functions themselves
    println!("enter the enemy position on the plane; x coordinate first.");
    let mut x_pos = String::new();
    io::stdin()
        .read_line(&mut x_pos)
        .expect("failed to read input");

    let x_pos: f32 =  x_pos.trim().parse().expect("please enter a number");
     println!("you entered x = {x_pos}");

println!("now enter a y coordinate.");
    let mut y_pos = String::new();
    io::stdin()
        .read_line(&mut y_pos)
        .expect("failed to read input");

    let y_pos: f32 =  y_pos.trim().parse().expect("please enter a number");
     println!("you entered y = {y_pos}");
//end of the block where we're taking inputs from console. When integrated into a game engine, these values *should* be provided by the server process, meaning this part could be commented out


    if (y_pos == 0.0) && (x_pos == 0.0) {println!("this computation will output nonsense; please make sure at least one of the inputs is non-zero"); main();} //if both values are zero, atan is undefined; start over. Yes, clippy complains about recursion to main(); too bad.
	else {
    let opp: f32 = x_pos;
    let adj: f32 = y_pos;
    let distance: f32 = ((opp * opp) + (adj * adj)).sqrt(); //apply the pythagorean theorem to get a euclidean norm. So far, only done for 2d, but should be trivial to extend to 3d
	println!("enemy is {distance} units away");

    let bearing: f32 = 
	if y_pos >= 0.0 {(opp/adj).atan()} else {(opp/adj).atan()+PI}; //This line. This function. Why doesn't trig work like trig? right, because the domain of atan is -PI to PI, and by setting the zero mark of my headings to the positve Y direction, I shot myself in the foot
    let bearing_adjusted: f32 = bearing/PI; 
	println!("enemy bearing is {bearing_adjusted}π radians");
    let intensity: f32 = 1.0/(distance * distance);
	println!("the sound is {intensity} times as loud as it would be if they were 1 unit away");


    let rightchannel: f32 = (1.0 + bearing.sin())/2.0;
    //if y_pos>=0.0 {(1.0 + bearing.sin())/2.0} else {(1.0 - bearing.sin())/2.0}; //This works. Again not quite sure why, but it compiles and outputs the correct result
    let leftchannel : f32 = 1.0 - rightchannel; //weird floating point error makes leftchannel appear as ".49999997" when doing the other way
	println!("{leftchannel} of the sound is coming through the left channel, while {rightchannel} is coming through the right channel");
	}
}
