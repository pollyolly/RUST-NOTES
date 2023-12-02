use tokio::time;
use std::time::Duration;
use mouse_rs::{types::keys::Keys, Mouse};

fn move_mouse(ctx: i32, cty: i32){
       let mouse = Mouse::new();
        mouse.move_to(ctx, cty).expect("Unable to move mouse");
}
/* fn scroll_wheel(wcty: i32){
    let mouse = Mouse::new();
    mouse.wheel(wcty);
}*/
#[tokio::main]
async fn main() {
    let mut interval = time::interval(Duration::from_millis(1000));
    let mut ctx = 500;
    let mut cty = 500;
    //let mut wcty = 2;
    loop {
        interval.tick().await;
        move_mouse(ctx, cty);
        interval.tick().await;
       //scroll_wheel(wcty);
        ctx += 2;
        cty -= 2;
        //wcty += 2;
    }
}
