extern crate piston_window;

use piston_window::*;
use piston_window::types::Color;
//use piston::input::*;


struct UiElement {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    color: Color,
    hovered: bool,
}

impl UiElement {
    fn new(x: f64, y: f64, width: f64, height: f64, color: Color) -> Self {
        Self {
            x,
            y,
            width,
            height,
            color,
            hovered: false,
        }
    }

    fn update_hover(&mut self, mouse_x: f64, mouse_y: f64) {
        let hovered = mouse_x >= self.x && mouse_x < self.x + self.width && mouse_y >= self.y && mouse_y < self.y + self.height;

        if hovered != self.hovered {
            self.color = if hovered { Color::from([0.7, 0.7, 0.7, 1.0]) } else { Color::from([0.5, 0.5, 0.5, 1.0]) };
            self.hovered = hovered;
        }
    }
}


pub struct Rectangle {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl Rectangle{
    pub fn new()->Rectangle{
        Rectangle {
            x : 100.0,
            y: 100.0,
            width: 200.0,
            height: 100.0,
        }
    }
}

pub struct Game {

}

impl Game {
    pub fn new()-> Game{
        Game{}
    }
    pub fn key_pressed(&mut self, key: Key){

    }
}

fn main() {
    let mut rec = Rectangle::new();
    let mut game = Game::new();

    let WIDTH = 800;
    let HEIGHT = 800;
    let square_size = WIDTH as f64 /8.0;

    let mut ui_elements = vec![
        UiElement::new(50.0, 50.0, 100.0, 50.0, Color::from([0.5, 0.5, 0.5, 1.0])),
        UiElement::new(150.0, 150.0, 100.0, 50.0, Color::from([0.5, 0.5, 0.5, 1.0])),
    ];


    //toujours mettre pour avoir la fenetre
    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [WIDTH, HEIGHT])
            .exit_on_esc(true).build().unwrap();


    while let Some(e) = window.next() {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            println!("pressed");
        }
        if let Some(mouse_event) = e.mouse_cursor_args() {
            for element in &mut ui_elements {
                element.update_hover(mouse_event[0], mouse_event[1]);
            }
        }

        window.draw_2d(&e, |c, g, _device| {
            clear([1.0; 4], g);

            for element in &ui_elements {
                rectangle(
                    element.color,
                    [element.x, element.y, element.width, element.height],
                    c.transform,
                    g,
                );
            }


            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [rec.x, rec.y, rec.width, rec.height], //largeur, hauteur
                      c.transform, g);
            for i in 0..8{
                for j in 0..8{
                    if (i+j)%2==0{
                        rectangle([1.0, 0.0, 0.0, 1.0],
                                  [100.0*i as f64 , 100.0*j as f64, square_size, square_size], //largeur, hauteur
                                  c.transform, g);
                    }else{
                        rectangle([0.0, 0.0, 0.0, 0.0],
                                  [100.0*i as f64 , 100.0*j as f64, square_size, square_size], //largeur, hauteur
                                  c.transform, g);
                    }

                }
            }
        });
        if let Event::Input(input, ..) = e {

            if let Input::Button(button_args) = input {
                if let Button::Keyboard(key) = button_args.button {
                    // Hold down a key, and see the message repeated in your terminal.
                    println!("Key event: {:?} {:?}", key, button_args.state);
                    match key {
                        Key::Up=>rec.width+=2.0,
                        Key::Down=>rec.width-=2.0,
                        _ => println!("todo"),
                    }

                }
            }

        }

    }
        /*window.draw_2d(&e, |c, g, _device| {
            clear([1.0; 4], g);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [rec.x, rec.y, rec.width, rec.height], //largeur, hauteur
                      c.transform, g);
        });*/
    }
