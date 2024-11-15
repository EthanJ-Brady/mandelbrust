pub fn mandelbrot(cx: f64, cy: f64, max_iter: u32) -> u32 {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut iteration = 0;

    while x * x + y * y <= 4.0 && iteration < max_iter {
        let x_temp = x * x - y * y + cx;
        y = 2.0 * x * y + cy;
        x = x_temp;

        iteration += 1;
    }

    iteration
}

pub fn burning_ship(cx: f64, cy: f64, max_iter: u32) -> u32 {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut iteration = 0;

    while x * x + y * y <= 4.0 && iteration < max_iter {
        let x_temp = x * x - y * y + cx;
        y = 2.0 * x.abs() * y.abs() + cy;
        x = x_temp.abs();

        iteration += 1;
    }

    iteration
}
