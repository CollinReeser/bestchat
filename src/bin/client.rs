use std::io::prelude::*;
use std::net::TcpStream;

extern crate rustbox;

use std::default::Default;

use rustbox::{Color, RustBox};
use rustbox::Key;



fn main() {
    let mut stream = TcpStream::connect("192.168.1.40:24567").unwrap();

    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    rustbox.print(
        0, 0,
        rustbox::RB_BOLD, Color::White, Color::Black,
        "Welcome to Best Chat!"
    );

    'outer_loop: loop {
        let mut width = rustbox.width();
        let mut height = rustbox.height();

        let mut line_height = height / 4 * 3;

        let mut x_reset = 0;
        let mut y_reset = line_height + 1;

        let mut x = x_reset;
        let mut y = y_reset;

        let mut line = String::new();

        loop {

            rustbox.print(
                0, line_height,
                rustbox::RB_BOLD, Color::White, Color::Black,
                &std::iter::repeat("=").take(width).collect::<String>()
            );

            rustbox.present();

            match rustbox.poll_event(false) {
                Err(e) => panic!("{}", e),
                Ok(rustbox::Event::KeyEvent(key)) => {
                    match key {
                        Key::Char(a) => {
                            rustbox.print_char(
                                x, y,
                                rustbox::RB_BOLD, Color::White, Color::Default,
                                a
                            );
                            line.push(a);
                            x += 1;
                            if x == width {
                                x = 0;
                                y += 1;
                            }
                        }
                        Key::Backspace => {
                            if x > 0 {
                                x -= 1;
                            }
                            else if y > y_reset {
                                x = width;
                                y -= 1;
                            }
                            else {
                                break;
                            }

                            line.pop();

                            rustbox.clear();

                            rustbox.print(
                                0, line_height,
                                rustbox::RB_BOLD, Color::White, Color::Black,
                                &std::iter::repeat("=").take(width)
                                                       .collect::<String>()
                            );

                            rustbox.print(
                                x_reset, y_reset,
                                rustbox::RB_BOLD, Color::White, Color::Default,
                                &line
                            );

                        }
                        Key::Enter => {
                            line.push('\n');

                            stream.write_all(&line.into_bytes()).unwrap();
                            stream.flush().unwrap();

                            break;
                        }
                        Key::Ctrl('c') => {
                            break 'outer_loop;
                        }
                        _ => {}
                    }
                },
                Ok (rustbox::Event::ResizeEvent(new_width, new_height)) => {
                    width = new_width as usize;
                    height = new_height as usize;

                    let new_line_height = height / 4 * 3;

                    y = if new_line_height > line_height {
                        y + new_line_height - line_height
                    }
                    else {
                        y + line_height - new_line_height
                    };

                    line_height = new_line_height;

                    x_reset = 0;
                    y_reset = line_height + 1;

                    rustbox.clear();

                    rustbox.print(
                        0, line_height,
                        rustbox::RB_BOLD, Color::White, Color::Black,
                        &std::iter::repeat("=").take(width).collect::<String>()
                    );

                    rustbox.print(
                        x_reset, y_reset,
                        rustbox::RB_BOLD, Color::White, Color::Default,
                        &line
                    );
                }
                _ => {},
            }
        }

        x = x_reset;
        y = y_reset;

        rustbox.clear();
    }
}
