use macroquad::prelude::*;

// quick dirty code
// todo: clean up later

#[derive(Clone, Copy)]
pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl std::ops::Add for Colour {
    type Output = Colour;

    fn add(mut self, other: Colour) -> Self::Output {
        self.r = if self.r + other.r <= 1. {
            self.r + other.r
        } else {
            1.
        };
        self.g = if self.g + other.g <= 1. {
            self.g + other.g
        } else {
            1.
        };
        self.b = if self.b + other.b <= 1. {
            self.b + other.b
        } else {
            1.
        };
        self.a = if self.a + other.a <= 1. {
            self.a + other.a
        } else {
            1.
        };

        self
    }
}

impl std::ops::AddAssign for Colour {
    fn add_assign(&mut self, other: Self) {
        self.r = if self.r + other.r <= 1. {
            self.r + other.r
        } else {
            1.
        };
        self.g = if self.g + other.g <= 1. {
            self.g + other.g
        } else {
            1.
        };
        self.b = if self.b + other.b <= 1. {
            self.b + other.b
        } else {
            1.
        };
        self.a = if self.a + other.a <= 1. {
            self.a + other.a
        } else {
            1.
        };
    }
}

impl std::ops::Sub for Colour {
    type Output = Colour;
    fn sub(mut self, other: Self) -> Self::Output {
        self.r = if self.r - other.r >= 0. {
            self.r - other.r
        } else {
            0.
        };
        self.g = if self.g - other.g >= 0. {
            self.g - other.g
        } else {
            0.
        };
        self.b = if self.b - other.b >= 0. {
            self.b - other.b
        } else {
            0.
        };
        self.a = if self.a - other.a >= 0. {
            self.a - other.a
        } else {
            0.
        };

        self
    }
}

impl std::ops::SubAssign for Colour {
    fn sub_assign(&mut self, other: Self) {
        self.r = if self.r - other.r >= 0. {
            self.r - other.r
        } else {
            0.
        };
        self.g = if self.g - other.g >= 0. {
            self.g - other.g
        } else {
            0.
        };
        self.b = if self.b - other.b >= 0. {
            self.b - other.b
        } else {
            0.
        };
        self.a = if self.a - other.a >= 0. {
            self.a - other.a
        } else {
            0.
        };
    }
}

impl Colour {
    fn to_color(&self) -> Color {
        Color::new(self.r, self.g, self.b, self.a)
    }
}
pub struct Square {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub colour: Colour,
    hover: bool, 
    drag: bool,
}

impl Square{

    pub fn update(&mut self){
        let x = self.x - (self.size / 2.);
        let y = self.y - (self.size / 2.);

        let (mx, my) = mouse_position();

        if mx >= x && mx <= (x + self.size) && my >= y && my <= (y + self.size) {
            if is_mouse_button_down(MouseButton::Left) {
                self.drag = true;
            } else {
                self.drag = false;
            }
            self.hover = true;
        } else {
            self.hover = false;
        }
    }

    pub fn draw (&mut self) {
        let x = self.x - (self.size / 2.);
        let y = self.y - (self.size / 2.);
        if self.drag {
            (self.x, self.y) = mouse_position();
            draw_rectangle(
                x,
                y,
                self.size,
                self.size,
                (self.colour
                    + Colour {
                        r: 0.3,
                        g: 0.3,
                        b: 0.3,
                        a: 0.0,
                    }).to_color(),
            )
        } else if self.hover{
            draw_rectangle(
                x,
                y,
                self.size,
                self.size,
                (self.colour
                    + Colour {
                        r: 0.2,
                        g: 0.2,
                        b: 0.2,
                        a: 0.0,
                    }).to_color())
        } else{
            draw_rectangle(x, y, self.size, self.size, self.colour.to_color());

        }
    }
    /// Create new square, 
    pub fn new (x: f32, y: f32, size: f32, colour: Colour,) -> Square{
        Square { x, y, size, colour, hover: false, drag: false,}
    }
}
