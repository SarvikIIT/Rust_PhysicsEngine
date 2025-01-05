use std::f32::INFINITY;
use macroquad::{input, prelude::*};
use crate::KeyCode::D;
use crate::KeyCode::A;
use crate::KeyCode::W;
use crate::KeyCode::S;
use crate::KeyCode::Enter;
struct Mass{
    x:f32
}

#[macroquad::main("Physics Engine")]
async fn main() {
    //-----------------------------------INITIALIZATION AREA-------------------------------------
    let mass_floor=Mass{
        x:INFINITY
    };
    let mass_ball=Mass{
        x:1.0
    };
    let res:f32=0.7;
    let mut x:f32=screen_width()/2.0;
    let mut y:f32=screen_height()/2.0;
    let mut y1:f32=40.0;
    let mut x1:f32=40.0;
    let mut vel_x=8.0;
    let mut vel_y=8.0;
    let mut dist_x:f32=0.0;
    let mut dist_y:f32=0.0;
    let mut x_pos=false;
    let mut x_neg=false;
    let mut y_neg=false;
    let mut y_pos=false;
    // ---------------------------------END-----------------------------------------------------



    //-----------------------CODE BEGINS-----------------------------------------------------------
    loop{
        clear_background(BLACK);
        draw_circle(x,y, 30.0,WHITE);
        draw_rectangle(0.0,screen_height()-20.0, screen_width()+60.0, 20.0,BLUE);
        if (x>screen_width()||x<0.0||y>screen_height()||y<0.0){
            draw_text("OUT OF BOUNDS:PRESS ENTER TO RESET", screen_width()/2.0-200.0, screen_height()/2.0, 44.0, RED);
        }
        //-----------------------------------INPUTS------------------------------------------------
    if is_key_pressed(Enter){
        y=screen_height()/2.0;
        x=screen_width()/2.0;
    }
    if is_key_pressed(D){
        x_pos=true;
    }
    if is_key_pressed(S){
        y_pos=true;
    }
    if is_key_pressed(A){
        x_neg=true;
    }
    if is_key_pressed(W){
        y_neg=true;
    }
    if is_key_released(D){
        x_pos=false;
    }
    if is_key_released(S){
        y_pos=false;
    }
    if is_key_released(A){
        x_neg=false;
    }
    if is_key_released(W){
        y_neg=false;
    }
    //-------------------------END OF INPUTS-------------------------------------------------------





    //-------------------------BASIC MOVEMENT------------------------------------------------------
    if x_pos{
        x+=vel_x;
    }
    if x_neg{
        x-=vel_x;
    }
    if y_neg{
        y-=vel_y;
    }
    if y_pos{
        y+=vel_y;
    }
    //------------------------END OF BASIC MOVEMENTS-----------------------------------------------
    


    //-----------------------CANVAS/FLOORs CREATION------------------------------------------------------
    if(x>screen_width()){
        x=screen_width();
    }
    if(y<0.0){
        y=2.0;
    }

    if(x<0.0){
        x=0.0;
    }

    if(y>screen_width()-250.0){
        y=screen_height()-50.0;
    }
//------------------------END OF CANVAS CREATION-------------------------------------------------------------------------

        next_frame().await;
}

}