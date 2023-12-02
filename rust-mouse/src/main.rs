use tokio::time;
use std::time::Duration;
use mouse_rs::{Mouse};
use rand::Rng;

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
    let _ = keepawake::Builder::new()
        .display(true)
        .idle(true)
        .sleep(true)
        .create();
    let mut interval = time::interval(Duration::from_millis(50));
    let mut ctx = 300;
    let mut cty = 300;
    let mut range = rand::thread_rng();
    let randx: i32 = range.gen_range(300..600);
    let randy: i32 = range.gen_range(300..600);
    //let mut wcty = 2;
    loop {
        interval.tick().await;
        move_mouse(ctx, cty);
       //scroll_wheel(wcty);
        ctx += 1;
        cty -= 1;
        //wcty += 2;
        if ctx == 600 {
            ctx = randx;
            cty = randy;
        }
    }
}
