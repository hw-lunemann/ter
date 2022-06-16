extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::Duration;
use std::time::SystemTime;

use std::io::{stdout, Write};
use std::ops::{Add, Mul, Sub};


use termion::{AsyncReader, color};
use termion::raw::IntoRawMode;

struct GameState {
    screen: Screen,
    depth_buf: Vec<(char, f32)>,
    map: Map,
    cam: Camera,
}

impl GameState {
    fn new() -> GameState {
        const WIDTH: usize = 140;
        const HEIGHT: usize = 40;

        let depth_buf = vec![(' ', 0.0f32); WIDTH];

        let map = Map {
            m: vec![
		 "################################",
         "#                        #     #",
         "#    #######              #    #",
         "#    #     #               #   #",
         "#       #  #                #  #",
         "#    #     #                 # #",
         "#    ### ###       #          ##",
         "#                            # #",
         "#                           #  #",
         "# # #                      #   #",
         "#                         #    #",
         "# # # #    #             #     #",
         "#         ###           #      #",
         "# # # #    ##          #       #",
         "#                     #        #",
         "################################"]
                .iter()
                .map(|x| x.chars().collect())
                .collect(),
            w: 32,
            h: 16,
        };

        let cam = Camera {
            pos: Vector { x: 2.5, y: 2.5 },
            dir: 0.0,
            fov: 55.0 / 360.0,
        };

        GameState {
            screen: Screen::new(WIDTH, HEIGHT),
            depth_buf,
            map,
            cam,
        }
    }
}

struct Screen {
    bufs: Vec<Framebuffer>,
    stdout: termion::raw::RawTerminal<std::io::Stdout>,
    width: usize,
    height: usize,
}

impl Screen {
    fn new(width: usize, height: usize) -> Screen {
        Screen {
            bufs: vec![Framebuffer::new(width, height); 2],
            stdout: stdout().into_raw_mode().unwrap(),
            width,
            height,
        }
    }

    fn front(&mut self) -> &mut Framebuffer {
        &mut self.bufs[0]
    }

    fn swap_buf(&mut self) {
        let _ = &mut self.bufs.swap(0, 1);
    }

    fn draw(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.bufs[0].get(x, y) != self.bufs[1].get(x, y) {
                    write!(
                        &mut self.stdout,
                        "{}{}{}{}",
                        color::Fg(GRAYSCALE[self.bufs[0].get(x, y).fg as usize]),
                        color::Bg(GRAYSCALE[self.bufs[0].get(x, y).bg as usize]),
                        termion::cursor::Goto(x as u16 + 1, y as u16 + 1),
                        self.bufs[0].get(x,y).c
                    ).unwrap();
                }
            }
        }
        self.stdout.flush().unwrap();
        self.swap_buf();
    }
}

#[derive(Clone)]
struct Framebuffer {
    pixels: Vec<Pixel>,
    width: usize,
    height: usize,
}

impl Framebuffer {
    fn new(width: usize, height: usize) -> Framebuffer {
        Framebuffer {
            pixels: vec![
                Pixel {
                    c: ' ',
                    fg: 0,
                    bg: 255
                };
                width * height
            ],
            width,
            height,
        }
    }

    fn set(&mut self, x: usize, y: usize, p: Pixel) {
        self.pixels[y * self.width + x] = p;
    }

    fn get(&self, x: usize, y: usize) -> Pixel {
        self.pixels[y * self.width + x]
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
struct Pixel {
    c: char,
    fg: u8,
    bg: u8,
}

static GRAYSCALE: [color::Rgb; 256] = [
    color::Rgb(0, 0, 0),
    color::Rgb(1, 1, 1),
    color::Rgb(2, 2, 2),
    color::Rgb(3, 3, 3),
    color::Rgb(4, 4, 4),
    color::Rgb(5, 5, 5),
    color::Rgb(6, 6, 6),
    color::Rgb(7, 7, 7),
    color::Rgb(8, 8, 8),
    color::Rgb(9, 9, 9),
    color::Rgb(10, 10, 10),
    color::Rgb(11, 11, 11),
    color::Rgb(12, 12, 12),
    color::Rgb(13, 13, 13),
    color::Rgb(14, 14, 14),
    color::Rgb(15, 15, 15),
    color::Rgb(16, 16, 16),
    color::Rgb(17, 17, 17),
    color::Rgb(18, 18, 18),
    color::Rgb(19, 19, 19),
    color::Rgb(20, 20, 20),
    color::Rgb(21, 21, 21),
    color::Rgb(22, 22, 22),
    color::Rgb(23, 23, 23),
    color::Rgb(24, 24, 24),
    color::Rgb(25, 25, 25),
    color::Rgb(26, 26, 26),
    color::Rgb(27, 27, 27),
    color::Rgb(28, 28, 28),
    color::Rgb(29, 29, 29),
    color::Rgb(30, 30, 30),
    color::Rgb(31, 31, 31),
    color::Rgb(32, 32, 32),
    color::Rgb(33, 33, 33),
    color::Rgb(34, 34, 34),
    color::Rgb(35, 35, 35),
    color::Rgb(36, 36, 36),
    color::Rgb(37, 37, 37),
    color::Rgb(38, 38, 38),
    color::Rgb(39, 39, 39),
    color::Rgb(40, 40, 40),
    color::Rgb(41, 41, 41),
    color::Rgb(42, 42, 42),
    color::Rgb(43, 43, 43),
    color::Rgb(44, 44, 44),
    color::Rgb(45, 45, 45),
    color::Rgb(46, 46, 46),
    color::Rgb(47, 47, 47),
    color::Rgb(48, 48, 48),
    color::Rgb(49, 49, 49),
    color::Rgb(50, 50, 50),
    color::Rgb(51, 51, 51),
    color::Rgb(52, 52, 52),
    color::Rgb(53, 53, 53),
    color::Rgb(54, 54, 54),
    color::Rgb(55, 55, 55),
    color::Rgb(56, 56, 56),
    color::Rgb(57, 57, 57),
    color::Rgb(58, 58, 58),
    color::Rgb(59, 59, 59),
    color::Rgb(60, 60, 60),
    color::Rgb(61, 61, 61),
    color::Rgb(62, 62, 62),
    color::Rgb(63, 63, 63),
    color::Rgb(64, 64, 64),
    color::Rgb(65, 65, 65),
    color::Rgb(66, 66, 66),
    color::Rgb(67, 67, 67),
    color::Rgb(68, 68, 68),
    color::Rgb(69, 69, 69),
    color::Rgb(70, 70, 70),
    color::Rgb(71, 71, 71),
    color::Rgb(72, 72, 72),
    color::Rgb(73, 73, 73),
    color::Rgb(74, 74, 74),
    color::Rgb(75, 75, 75),
    color::Rgb(76, 76, 76),
    color::Rgb(77, 77, 77),
    color::Rgb(78, 78, 78),
    color::Rgb(79, 79, 79),
    color::Rgb(80, 80, 80),
    color::Rgb(81, 81, 81),
    color::Rgb(82, 82, 82),
    color::Rgb(83, 83, 83),
    color::Rgb(84, 84, 84),
    color::Rgb(85, 85, 85),
    color::Rgb(86, 86, 86),
    color::Rgb(87, 87, 87),
    color::Rgb(88, 88, 88),
    color::Rgb(89, 89, 89),
    color::Rgb(90, 90, 90),
    color::Rgb(91, 91, 91),
    color::Rgb(92, 92, 92),
    color::Rgb(93, 93, 93),
    color::Rgb(94, 94, 94),
    color::Rgb(95, 95, 95),
    color::Rgb(96, 96, 96),
    color::Rgb(97, 97, 97),
    color::Rgb(98, 98, 98),
    color::Rgb(99, 99, 99),
    color::Rgb(100, 100, 100),
    color::Rgb(101, 101, 101),
    color::Rgb(102, 102, 102),
    color::Rgb(103, 103, 103),
    color::Rgb(104, 104, 104),
    color::Rgb(105, 105, 105),
    color::Rgb(106, 106, 106),
    color::Rgb(107, 107, 107),
    color::Rgb(108, 108, 108),
    color::Rgb(109, 109, 109),
    color::Rgb(110, 110, 110),
    color::Rgb(111, 111, 111),
    color::Rgb(112, 112, 112),
    color::Rgb(113, 113, 113),
    color::Rgb(114, 114, 114),
    color::Rgb(115, 115, 115),
    color::Rgb(116, 116, 116),
    color::Rgb(117, 117, 117),
    color::Rgb(118, 118, 118),
    color::Rgb(119, 119, 119),
    color::Rgb(120, 120, 120),
    color::Rgb(121, 121, 121),
    color::Rgb(122, 122, 122),
    color::Rgb(123, 123, 123),
    color::Rgb(124, 124, 124),
    color::Rgb(125, 125, 125),
    color::Rgb(126, 126, 126),
    color::Rgb(127, 127, 127),
    color::Rgb(128, 128, 128),
    color::Rgb(129, 129, 129),
    color::Rgb(130, 130, 130),
    color::Rgb(131, 131, 131),
    color::Rgb(132, 132, 132),
    color::Rgb(133, 133, 133),
    color::Rgb(134, 134, 134),
    color::Rgb(135, 135, 135),
    color::Rgb(136, 136, 136),
    color::Rgb(137, 137, 137),
    color::Rgb(138, 138, 138),
    color::Rgb(139, 139, 139),
    color::Rgb(140, 140, 140),
    color::Rgb(141, 141, 141),
    color::Rgb(142, 142, 142),
    color::Rgb(143, 143, 143),
    color::Rgb(144, 144, 144),
    color::Rgb(145, 145, 145),
    color::Rgb(146, 146, 146),
    color::Rgb(147, 147, 147),
    color::Rgb(148, 148, 148),
    color::Rgb(149, 149, 149),
    color::Rgb(150, 150, 150),
    color::Rgb(151, 151, 151),
    color::Rgb(152, 152, 152),
    color::Rgb(153, 153, 153),
    color::Rgb(154, 154, 154),
    color::Rgb(155, 155, 155),
    color::Rgb(156, 156, 156),
    color::Rgb(157, 157, 157),
    color::Rgb(158, 158, 158),
    color::Rgb(159, 159, 159),
    color::Rgb(160, 160, 160),
    color::Rgb(161, 161, 161),
    color::Rgb(162, 162, 162),
    color::Rgb(163, 163, 163),
    color::Rgb(164, 164, 164),
    color::Rgb(165, 165, 165),
    color::Rgb(166, 166, 166),
    color::Rgb(167, 167, 167),
    color::Rgb(168, 168, 168),
    color::Rgb(169, 169, 169),
    color::Rgb(170, 170, 170),
    color::Rgb(171, 171, 171),
    color::Rgb(172, 172, 172),
    color::Rgb(173, 173, 173),
    color::Rgb(174, 174, 174),
    color::Rgb(175, 175, 175),
    color::Rgb(176, 176, 176),
    color::Rgb(177, 177, 177),
    color::Rgb(178, 178, 178),
    color::Rgb(179, 179, 179),
    color::Rgb(180, 180, 180),
    color::Rgb(181, 181, 181),
    color::Rgb(182, 182, 182),
    color::Rgb(183, 183, 183),
    color::Rgb(184, 184, 184),
    color::Rgb(185, 185, 185),
    color::Rgb(186, 186, 186),
    color::Rgb(187, 187, 187),
    color::Rgb(188, 188, 188),
    color::Rgb(189, 189, 189),
    color::Rgb(190, 190, 190),
    color::Rgb(191, 191, 191),
    color::Rgb(192, 192, 192),
    color::Rgb(193, 193, 193),
    color::Rgb(194, 194, 194),
    color::Rgb(195, 195, 195),
    color::Rgb(196, 196, 196),
    color::Rgb(197, 197, 197),
    color::Rgb(198, 198, 198),
    color::Rgb(199, 199, 199),
    color::Rgb(200, 200, 200),
    color::Rgb(201, 201, 201),
    color::Rgb(202, 202, 202),
    color::Rgb(203, 203, 203),
    color::Rgb(204, 204, 204),
    color::Rgb(205, 205, 205),
    color::Rgb(206, 206, 206),
    color::Rgb(207, 207, 207),
    color::Rgb(208, 208, 208),
    color::Rgb(209, 209, 209),
    color::Rgb(210, 210, 210),
    color::Rgb(211, 211, 211),
    color::Rgb(212, 212, 212),
    color::Rgb(213, 213, 213),
    color::Rgb(214, 214, 214),
    color::Rgb(215, 215, 215),
    color::Rgb(216, 216, 216),
    color::Rgb(217, 217, 217),
    color::Rgb(218, 218, 218),
    color::Rgb(219, 219, 219),
    color::Rgb(220, 220, 220),
    color::Rgb(221, 221, 221),
    color::Rgb(222, 222, 222),
    color::Rgb(223, 223, 223),
    color::Rgb(224, 224, 224),
    color::Rgb(225, 225, 225),
    color::Rgb(226, 226, 226),
    color::Rgb(227, 227, 227),
    color::Rgb(228, 228, 228),
    color::Rgb(229, 229, 229),
    color::Rgb(230, 230, 230),
    color::Rgb(231, 231, 231),
    color::Rgb(232, 232, 232),
    color::Rgb(233, 233, 233),
    color::Rgb(234, 234, 234),
    color::Rgb(235, 235, 235),
    color::Rgb(236, 236, 236),
    color::Rgb(237, 237, 237),
    color::Rgb(238, 238, 238),
    color::Rgb(239, 239, 239),
    color::Rgb(240, 240, 240),
    color::Rgb(241, 241, 241),
    color::Rgb(242, 242, 242),
    color::Rgb(243, 243, 243),
    color::Rgb(244, 244, 244),
    color::Rgb(245, 245, 245),
    color::Rgb(246, 246, 246),
    color::Rgb(247, 247, 247),
    color::Rgb(248, 248, 248),
    color::Rgb(249, 249, 249),
    color::Rgb(250, 250, 250),
    color::Rgb(251, 251, 251),
    color::Rgb(252, 252, 252),
    color::Rgb(253, 253, 253),
    color::Rgb(254, 254, 254),
    color::Rgb(255, 255, 255),
];

struct Camera {
    pos: Vector,
    dir: f32,
    fov: f32,
}

impl Camera {
    fn turn(&mut self, change: f32) {
        self.dir = bound(self.dir + change, 0.0, 1.0);
    }

    fn step(&mut self, dist: f32) {
        let r = Vector::from_dir(self.dir);
        self.pos = self.pos + r * dist;
    }

    fn strafe(&mut self, dist: f32) {
        let r = Vector::from_dir(self.dir + 0.25);
        self.pos = self.pos + r * dist;
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
struct Vector {
    x: f32,
    y: f32,
}

impl Vector {
    fn len(&self) -> f32 {
        ((self.x).powf(2.0) + (self.y).powf(2.0)).sqrt()
    }

    fn from_dir(d: f32) -> Vector {
        Vector {
            x: (d * 2.0 * std::f32::consts::PI).cos(),
            y: (d * 2.0 * std::f32::consts::PI).sin(),
        }
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, other: f32) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Map {
    m: Vec<Vec<char>>,
    w: usize,
    h: usize,
}

impl Map {
    fn at(&self, x: f32, y: f32) -> char {
        self.m[y as usize][x as usize]
    }
}

fn bound(mut f: f32, min: f32, max: f32) -> f32 {
	while f >= max {
        f -= max - min;
	}
	while f < min {
        f += max - min;
	}

	f
}

fn march(p: Vector, v: Vector, s: f32) -> Vector {
    p + v.mul(s)
}

fn shoot_vector(map: &Map, p: Vector, dir: f32) -> (char, f32) {
    let vector = Vector::from_dir(dir);
    let mut s = 0.0;
    let delta = 0.001;
    let mut new_p;

    loop {
        s += delta;
        new_p = march(p, vector, s);

        let c = map.at(new_p.x, new_p.y);
        if c != ' ' {
            return (c, (new_p - p).len());
        }

        if new_p.x < 0.0 || new_p.x > map.w as f32 || new_p.y < 0.0 || new_p.y > map.h as f32 {
            return (' ', (new_p - p).len());
        }
    }
}

fn render_background(screen: &mut Screen) {
    //#x-.
    for y in 0..screen.height {
        for x in 0..screen.width {
            let p;

            if y * 100 > screen.height * 80 {
                p = Pixel { c: '#', fg: 0, bg: 255 }
            } else if y * 100 > screen.height * 70 {
                p = Pixel { c: 'x', fg: 51, bg: 255 }
            } else if y * 100 > screen.height * 65 {
                p = Pixel { c: '-', fg: 77, bg: 255 }
            } else if y * 100 > screen.height * 50 {
                p = Pixel { c: '.', fg: 90, bg: 255 }
            } else {
                p = Pixel { c: ' ', fg: 255, bg: 255 }
            }

            screen.front().set(
                x,
                y,
                p
            );
        }
    }
}

// █▇▆▅▄▃▂▁
// █▓▒░
// █▀▔
fn render_walls(screen: &mut Screen, depth_buf: &mut Vec<(char, f32)>) {
    assert_eq!(screen.width, depth_buf.len());
    for (x, hit) in depth_buf.iter().enumerate() {
        // pixel is not pure white if depth at pixel is < 16
        let fade_col = if hit.1 < 24.0 {
            (hit.1.sqrt() / 24.0f32.sqrt() * 255.0) as u8
        } else {
            255u8
        };

        // TODO: remove code duplication between render_walls render_background
        let column_height = (screen.height as f32) * 1.25 / (hit.1).abs();
        if column_height > screen.height as f32 {
            for y in 0 .. screen.height {
                screen.front().set(
                    x,
                    y,
                    Pixel {c: '█', fg: fade_col, bg: fade_col}
                );
            }
        } else {
            let margin = (screen.height as f32 - column_height) / 2.0;
            let partial_margin = margin - margin.floor();


            // render wall edges
            const BLOCKS1: [char; 8] = ['█', '▇', '▆', '▅', '▄', '▃', '▂', '▁']; 
            screen.front().set(
                x,
                margin as usize,
                Pixel {
                    c: BLOCKS1[(partial_margin*8.0) as usize],
                    fg: fade_col,
                    bg: 255
                }
            );

            const BLOCKS2: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█', ]; 
            screen.front().set(
                x,
                (margin + column_height) as usize,
                Pixel {
                    c: BLOCKS2[(partial_margin*8.0) as usize],
                    fg: 255,
                    bg: fade_col
                }
            );

            for y in margin as usize + 1.. (column_height + margin) as usize {
                screen.front().set(
                    x,
                    y,
                    Pixel {c: '█', fg: fade_col, bg: fade_col}
                );
            }

        }

    }
}

fn render(game: &mut GameState) {

    // fill buffer with char and depth
    for i in 0..game.screen.width {
        let mut dir = (game.cam.dir - game.cam.fov / 2.0) + (game.cam.fov / game.screen.width as f32) * i as f32;
        dir = bound(dir, 0.0, 1.0);
        let (c, distance) = shoot_vector(&game.map, game.cam.pos, dir);
        game.depth_buf[i] = (
            c,
            distance * ((dir - game.cam.dir) * 2.0 * std::f32::consts::PI).cos(),
        );
    }

    // render screen
    render_background(&mut game.screen);
    render_walls(&mut game.screen, &mut game.depth_buf);
}

pub fn main() -> Result<(), String> {
    let mut game = GameState::new();

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let _window = video_subsystem.window("Keyboard", 20, 20)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut events = sdl_context.event_pump()?;
    'running: loop {
        for event in events.poll_iter() {
            if let Event::Quit {..} = event {
                break 'running;
            };
        }

        // Create a set of pressed Keys.
        for key in events.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode){
            match key {
               Keycode::Escape => break 'running,
                Keycode::W => game.cam.step(0.07),
                Keycode::A => game.cam.strafe(-0.07),
                Keycode::S => game.cam.step(-0.07),
                Keycode::D => game.cam.strafe(0.07),
                Keycode::J => game.cam.turn(1.5 / 360.0),
                Keycode::K => game.cam.turn(-1.5 / 360.0),
                _ => {}
			}
        }

	    let now = SystemTime::now();
    	render(&mut game);
    	game.screen.draw();

		if let Ok(elapsed) = now.elapsed() {
			let fps60 = Duration::new(0, 1_000_000_000u32 / 60);
			if fps60 > elapsed {
        		std::thread::sleep(fps60 - elapsed);
			}
		}

		if let Ok(elapsed) = now.elapsed() {
			write!(
				game.screen.stdout,
				"{}{}{}{}",
				color::Fg(GRAYSCALE[255]),
				color::Bg(GRAYSCALE[0]),
				termion::cursor::Goto(0u16, (game.screen.height + 1) as u16),
				1.0/elapsed.as_secs_f32()
			).map_err(|e| e.to_string())?;
		}
    }

    Ok(())
}
