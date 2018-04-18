#![allow(unknown_lints)] // TODO(***realname***): Can go when clippy lints go.

extern crate chessjam;

#[macro_use]
extern crate static_assets;
#[macro_use]
extern crate glium;

extern crate adequate_math;
extern crate glium_text;
extern crate pleco;
extern crate rand;
extern crate wavefront_obj;

mod graphics;
mod input;
mod ui;

use std::time::Instant;

use adequate_math::*;
use glium::glutin::EventsLoop;
use glium::Display;

use graphics::Mesh;
use input::*;


struct RenderCommand<'a> {
    mesh: &'a Mesh,
    color: Vec4<f32>,
    mvp_matrix: Mat4<f32>,
}


#[derive(Debug)]
pub struct Piece {
    pub position: Vec2<i32>,
    pub color: ChessColor,
    pub piece_type: PieceType,
    pub moved: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ChessColor {
    Black,
    White,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Knight,
    Rook,
    Bishop,
    Queen,
    King,
}

#[derive(Debug, Default)]
pub struct PiecePrice {
    pub buy_price: u32,
    pub discount_price: u32,
    pub sell_price: u32,
    pub unmoved_sell_price: u32,
}

#[derive(Debug, Copy, Clone)]
pub enum ControlState {
    Idle,
    SelectedPieceIndex(usize),
    SelectedPurchaseIndex(usize),
}


fn piece_price(piece_type: PieceType) -> &'static PiecePrice {
    const PAWN: PiecePrice = PiecePrice {
        buy_price: 4,
        discount_price: 3,
        sell_price: 2,
        unmoved_sell_price: 4,
    };
    const KNIGHT: PiecePrice = PiecePrice {
        buy_price: 5,
        discount_price: 3,
        sell_price: 3,
        unmoved_sell_price: 3,
    };
    const ROOK: PiecePrice = PiecePrice {
        buy_price: 6,
        discount_price: 4,
        sell_price: 3,
        unmoved_sell_price: 3,
    };
    const BISHOP: PiecePrice = PiecePrice {
        buy_price: 7,
        discount_price: 5,
        sell_price: 4,
        unmoved_sell_price: 4,
    };
    const QUEEN: PiecePrice = PiecePrice {
        buy_price: 9,
        discount_price: 6,
        sell_price: 5,
        unmoved_sell_price: 5,
    };

    match piece_type {
        PieceType::Pawn => &PAWN,
        PieceType::Knight => &KNIGHT,
        PieceType::Rook => &ROOK,
        PieceType::Bishop => &BISHOP,
        PieceType::Queen => &QUEEN,
        PieceType::King => unreachable!("Do not buy or sell kings!")
    }
}


fn random_piece() -> PieceType {
    use rand::distributions::{Weighted, WeightedChoice, IndependentSample};

    let mut choices = [
        Weighted { weight: 24, item: PieceType::Pawn },
        Weighted { weight: 20, item: PieceType::Knight },
        Weighted { weight: 16, item: PieceType::Rook },
        Weighted { weight: 14, item: PieceType::Bishop },
        Weighted { weight: 10, item: PieceType::Queen },
    ];
    let wc = WeightedChoice::new(&mut choices);
    let mut rng = rand::thread_rng();

    wc.ind_sample(&mut rng)
}


fn stopclock(title: &str, last_tick: &mut Instant, buffer: &mut String) {
    use std::fmt::Write;

    let now = Instant::now();
    let elapsed = now.duration_since(*last_tick);
    *last_tick = now;
    let elapsed_millis = f64::from(elapsed.subsec_nanos()) / 1_000_000.0;
    writeln!(buffer, "{}: {:.3}ms", title, elapsed_millis).unwrap();
}


fn main() {
    use glium::glutin::{Api, ContextBuilder, GlProfile, GlRequest, WindowBuilder};

    let config = chessjam::config::load_config();
    let res = &config.graphics.resolution;

    let mut events_loop = EventsLoop::new();

    let window = WindowBuilder::new()
        .with_dimensions(res[0] as u32, res[1] as u32)
        .with_title("Purchess");

    let context = {
        ContextBuilder::new()
            .with_depth_buffer(24)
            .with_gl_profile(GlProfile::Core)
            .with_gl(GlRequest::Specific(Api::OpenGl, (4, 0)))
            .with_vsync(config.graphics.vsync)
            .with_multisampling(config.graphics.multisampling as u16)
    };

    let display = &Display::new(window, context, &events_loop).unwrap();

    loop {
        let rerun = run_game(display, &mut events_loop);

        if !rerun {
            break;
        }
    }
}


#[allow(cyclomatic_complexity)]
fn run_game(
    display: &Display,
    events_loop: &mut EventsLoop,
) -> bool {
    use std::io::Cursor;

    use glium::Rect;
    use glium_text::{FontTexture, TextSystem};

    use ui::LabelRenderer;

    let config = chessjam::config::load_config();

    let model_shader = graphics::create_shader(
        display,
        asset_str!("assets/shaders/model.glsl").as_ref(),
    );

    let shadow_shader = graphics::create_shader(
        display,
        asset_str!("assets/shaders/shadow.glsl").as_ref(),
    );

    let cube_mesh = graphics::create_obj_mesh(
        display,
        asset_str!("assets/meshes/tile.obj").as_ref(),
    );

    let pawn_mesh = graphics::create_obj_mesh(
        display,
        asset_str!("assets/meshes/pawn.obj").as_ref(),
    );

    let king_mesh = graphics::create_obj_mesh(
        display,
        asset_str!("assets/meshes/king.obj").as_ref(),
    );

    let queen_mesh = graphics::create_obj_mesh(
        display,
        asset_str!("assets/meshes/queen.obj").as_ref(),
    );

    let bishop_mesh = graphics::create_obj_mesh(
        display,
        asset_str!("assets/meshes/bishop.obj").as_ref(),
    );

    let rook_mesh = graphics::create_obj_mesh(
        display,
        asset_str!("assets/meshes/rook.obj").as_ref(),
    );

    let knight_mesh = graphics::create_obj_mesh(
        display,
        asset_str!("assets/meshes/knight.obj").as_ref(),
    );

    let text_system = TextSystem::new(display);
    let font = Cursor::new(asset_bytes!("assets/fonts/bombardier.ttf"));
    let font_texture =
        FontTexture::new(display, font, config.text.size as u32).unwrap();

    let mut label_renderer = LabelRenderer::new();
    let mut world_label_renderer = LabelRenderer::new();

    let mut frame_time = Instant::now();
    let mut keyboard = Keyboard::default();
    let mut mouse = Mouse::default();


    const TARGET_ASPECT: f32 = 16.0 / 9.0;
    const CAMERA_NEAR_PLANE: f32 = 0.1;
    let camera_fov = consts::TAU32 * config.camera.fov as f32;
    let projection_matrix = matrix::perspective_projection(
        TARGET_ASPECT,
        camera_fov,
        CAMERA_NEAR_PLANE,
        100.0,
    );

    let mut camera_angle = config.camera.angle as f32;
    let mut camera_tilt = config.camera.tilt as f32;

    let shadow_direction = Vec4::from_slice(&config.light.key_dir)
        .norm()
        .as_f32();
    let light_direction_matrix: Mat4<f32> = {
        let key = shadow_direction;
        let fill = Vec4::from_slice(&config.light.fill_dir)
            .norm()
            .as_f32();
        let back = Vec4::from_slice(&config.light.back_dir)
            .norm()
            .as_f32();

        Mat4([key.0, fill.0, back.0, [0.0, 0.0, 0.0, 1.0]]).transpose()
    };

    let light_color_matrix: Mat4<f32> = Mat4([
        Vec4::from_slice(&config.light.key_color)
            .as_f32()
            .0,
        Vec4::from_slice(&config.light.fill_color)
            .as_f32()
            .0,
        Vec4::from_slice(&config.light.back_color)
            .as_f32()
            .0,
        Vec4::from_slice(&config.light.amb_color)
            .as_f32()
            .0,
    ]);

    let shadow_color_matrix: Mat4<f32> = Mat4([
        Vec4::from_slice(&config.shadow.key_color)
            .as_f32()
            .0,
        Vec4::from_slice(&config.shadow.fill_color)
            .as_f32()
            .0,
        Vec4::from_slice(&config.shadow.back_color)
            .as_f32()
            .0,
        Vec4::from_slice(&config.shadow.amb_color)
            .as_f32()
            .0,
    ]);

    let sell_tile = Vec2::from_slice(&config.game.sell_tile).as_i32();
    let buy_tiles = config
        .game
        .buy_tiles
        .iter()
        .map(|slice| Vec2::from_slice(slice).as_i32())
        .collect::<Vec<Vec2<i32>>>();

    let mut pieces = {
        let mut pieces = Vec::new();
        for x in 0..8 {
            pieces.push(Piece {
                position: vec2(x, 1),
                color: ChessColor::White,
                piece_type: PieceType::Pawn,
                moved: false,
            });
            pieces.push(Piece {
                position: vec2(x, 6),
                color: ChessColor::Black,
                piece_type: PieceType::Pawn,
                moved: false,
            });
        }
        pieces.push(Piece {
            position: vec2(4, 0),
            color: ChessColor::White,
            piece_type: PieceType::King,
            moved: false,
        });
        pieces.push(Piece {
            position: vec2(4, 7),
            color: ChessColor::Black,
            piece_type: PieceType::King,
            moved: false,
        });
        pieces
    };

    let mut pieces_for_sale = [
        Some(random_piece()),
        Some(random_piece()),
        Some(random_piece())];

    let mut white_coins = 0;
    let mut black_coins = 0;

    let mut control_state = ControlState::Idle;
    let mut valid_destinations: Vec<Vec2<i32>> = vec![];
    let mut whos_turn = ChessColor::White;

    let mut lit_render_buffer = Vec::new();
    let mut highlight_render_buffer = Vec::new();

    // Profiling
    let mut timer = Instant::now();
    let mut timesheet = String::new();

    loop {
        let (dt, now) = chessjam::delta_time(frame_time);
        frame_time = now;
        let timer = &mut timer;
        let timesheet = &mut timesheet;

        stopclock("between-frames", timer, timesheet);

        // handle_events
        let mut closed = false;
        let mut camera_motion = vec2(0.0, 0.0);

        {
            use glium::glutin::{
                ElementState, Event, MouseScrollDelta, WindowEvent,
            };

            let mut keyboard = keyboard.begin_frame_input();
            let mut mouse = mouse.begin_frame_input();

            #[allow(single_match)]
            events_loop.poll_events(|event| match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::KeyboardInput { input, .. } => {
                        let pressed = input.state == ElementState::Pressed;

                        if let Some(key) = input.virtual_keycode {
                            if pressed {
                                keyboard.press(key, input.modifiers);
                            }
                            else {
                                keyboard.release(key, input.modifiers);
                            }
                        }
                    }
                    WindowEvent::MouseInput {
                        state, button, ..
                    } => {
                        let pressed = state == ElementState::Pressed;
                        if pressed {
                            mouse.press(button);
                        }
                        else {
                            mouse.release(button);
                        }
                    }
                    WindowEvent::MouseWheel { delta, .. } => {
                        if let MouseScrollDelta::PixelDelta(x, y) = delta {
                            camera_motion = vec2(x, y);
                        }
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        let (x, y) = position;
                        mouse.move_cursor_to(x, y);
                    }
                    WindowEvent::Closed => closed = true,
                    _ => (),
                },
                _ => (),
            });
        }

        stopclock("inputs", timer, timesheet);

        if closed || keyboard.pressed(Key::Escape) {
            return false;
        }
        if keyboard.pressed(Key::R) && keyboard.modifiers.logo {
            return true;
        }

        camera_angle += camera_motion.0[0] * dt;
        camera_tilt += camera_motion.0[1] * dt;

        let camera_position: Vec3<f32>;
        let camera_direction: Vec3<f32>;

        let view_matrix = {
            let angles = vec3(camera_tilt, camera_angle, 0.0).map(f32::to_radians);
            let orientation = matrix::euler_rotation(angles);
            camera_direction = (orientation * vec4(0.0, 0.0, 1.0, 0.0)).retract();
            camera_position = -camera_direction * config.camera.distance as f32;
            let translation = Mat4::translation(-camera_position);
            orientation.transpose() * translation
        };

        let view_vector = -camera_direction.norm();

        let view_projection_matrix = projection_matrix * view_matrix;

        let viewport = {
            let (left, bottom, width, height) = chessjam::viewport_rect(
                display.get_framebuffer_dimensions(),
                TARGET_ASPECT,
            );
            Rect {
                left,
                bottom,
                width,
                height,
            }
        };


        let tile_cursor = {
            let camera_forward = camera_direction.norm();
            let camera_right = vec3(0.0, 1.0, 0.0).cross(camera_forward).norm();
            let camera_up = camera_forward.cross(camera_right);

            let (w, h) = display.get_framebuffer_dimensions();
            let mouse_pos = (Vec2(mouse.position()) / vec2(w, h).as_f64()).as_f32();
            let screen_pos = (mouse_pos - vec2(0.5, 0.5)) * vec2(2.0, -2.0);
            let viewport_pos = screen_pos
                * chessjam::viewport_stretch(
                    (w, h),
                    viewport.width,
                    viewport.height,
                );
            let (mx, my) = viewport_pos.as_tuple();
            let near_plane_half_height =
                (camera_fov / 2.0).tan() * CAMERA_NEAR_PLANE;
            let near_plane_half_width = near_plane_half_height * TARGET_ASPECT;

            let mouse_ray = {
                (camera_right * mx * near_plane_half_width)
                    + (camera_up * my * near_plane_half_height)
                    + (camera_forward * CAMERA_NEAR_PLANE)
            };

            let t = -(camera_position.0[1] / mouse_ray.0[1]);
            let hit = camera_position + mouse_ray * t;

            chessjam::world_to_grid(hit)
        };

        stopclock("pre-update", timer, timesheet);

        // update
        {
            fn piece_at(position: Vec2<i32>, pieces: &[Piece]) -> Option<usize> {
                let mut result = None;
                for (index, piece) in pieces.iter().enumerate() {
                    if piece.position == position {
                        result = Some(index);
                    }
                }
                result
            }

            if mouse.pressed(Button::Left) {
                match control_state {
                    ControlState::Idle => {
                        for (index, &tile) in buy_tiles.iter().enumerate() {
                            if tile == tile_cursor {
                                control_state = ControlState::SelectedPurchaseIndex(index);
                            }
                        }
                        control_state = match piece_at(tile_cursor, &pieces) {
                            Some(index) => ControlState::SelectedPieceIndex(index),
                            _ => control_state,
                        };
                    }
                    ControlState::SelectedPieceIndex(index) => {
                        if valid_destinations.contains(&tile_cursor) {
                            let taken_piece = piece_at(tile_cursor, &pieces);
                            pieces[index].position = tile_cursor;
                            pieces[index].moved = true;

                            // TODO(***realname***): Big game-flow stuff like this should
                            // be done in a less nested scope.
                            whos_turn = match whos_turn {
                                ChessColor::White => ChessColor::Black,
                                ChessColor::Black => ChessColor::White,
                            };

                            // Restock shop
                            for piece in &mut pieces_for_sale {
                                if piece.is_none() {
                                    *piece = Some(random_piece());
                                }
                            }

                            // TODO(***realname***): When castling, this will do Bad Things.
                            if let Some(index) = taken_piece {
                                pieces.swap_remove(index);
                            }
                        }
                        else if tile_cursor == sell_tile {
                            if pieces[index].piece_type != PieceType::King {
                                if pieces[index].color == whos_turn {
                                    let prices = piece_price(pieces[index].piece_type);
                                    let refund = if pieces[index].moved { prices.sell_price } else { prices.unmoved_sell_price };

                                    match whos_turn {
                                        ChessColor::White => white_coins += refund,
                                        ChessColor::Black => black_coins += refund,
                                    }
                                    pieces.swap_remove(index);
                                }
                            }
                        }
                        control_state = ControlState::Idle;
                    }
                    ControlState::SelectedPurchaseIndex(index) => {
                        // TODO(***realname***): Limit (and display) valid placements
                        if piece_at(tile_cursor, &pieces).is_none() && chessjam::valid_square(tile_cursor) {
                            let piece_type = pieces_for_sale[index];

                            if let Some(piece_type) = piece_type {
                                let price = piece_price(piece_type).buy_price;

                                let wallet = match whos_turn {
                                    ChessColor::White => &mut white_coins,
                                    ChessColor::Black => &mut black_coins,
                                };
                                if price <= *wallet {
                                    *wallet -= price;
                                    pieces.push(Piece {
                                        position: tile_cursor,
                                        color: whos_turn,
                                        piece_type,
                                        moved: false,
                                    });
                                    pieces_for_sale[index] = None;
                                }
                            }
                        }

                        control_state = ControlState::Idle;
                    }
                }

                // Recalculate possible moves
                valid_destinations.clear();
                if let ControlState::SelectedPieceIndex(index) = control_state {
                    use pleco::Board;

                    let piece = &pieces[index];
                    let (px, py) = piece.position.as_tuple();
                    let piece_pos_u8 = (py * 8 + px) as u8;

                    // First generate FEN
                    let fen = {
                        use std::fmt::Write;

                        let mut buffer = String::with_capacity(128);
                        let mut empty_stretch = 0;

                        for y in 0..8 {
                            for x in 0..8 {
                                match piece_at(vec2(x, 7 - y).as_i32(), &pieces) {
                                    Some(index) => {
                                        use ChessColor::*;
                                        use PieceType::*;

                                        if empty_stretch > 0 {
                                            write!(buffer, "{}", empty_stretch)
                                                .unwrap();
                                            empty_stretch = 0;
                                        }

                                        let piece = &pieces[index];
                                        let ch =
                                            match (piece.color, piece.piece_type) {
                                                (White, Pawn) => "P",
                                                (White, King) => "K",
                                                (White, Queen) => "Q",
                                                (White, Bishop) => "B",
                                                (White, Rook) => "R",
                                                (White, Knight) => "N",
                                                (Black, Pawn) => "p",
                                                (Black, King) => "k",
                                                (Black, Queen) => "q",
                                                (Black, Bishop) => "b",
                                                (Black, Rook) => "r",
                                                (Black, Knight) => "n",
                                            };
                                        buffer.push_str(ch);
                                    }
                                    None => {
                                        empty_stretch += 1;
                                    }
                                }
                            }

                            if empty_stretch > 0 {
                                write!(buffer, "{}", empty_stretch).unwrap();
                                empty_stretch = 0;
                            }

                            if y < 7 {
                                buffer.push_str("/");
                            }
                        }

                        match whos_turn {
                            ChessColor::White => buffer.push_str(" w "),
                            ChessColor::Black => buffer.push_str(" b "),
                        }

                        buffer.push_str("KQkq - 0 1");

                        buffer
                    };

                    let board = Board::from_fen(&fen).unwrap();
                    let moves = board.generate_moves();

                    for chessmove in moves.iter() {
                        let from_u8 = chessmove.get_src_u8();
                        let to_u8 = chessmove.get_dest_u8();
                        let dest = vec2(to_u8 % 8, to_u8 / 8).as_i32();
                        if from_u8 == piece_pos_u8 {
                            valid_destinations.push(dest);
                        }
                    }
                }
            }
        }

        stopclock("update", timer, timesheet);

        // render
        {
            use glium::{
                draw_parameters::{Stencil, StencilOperation, StencilTest},
                BackfaceCullingMode,
                Blend,
                Depth,
                DepthTest,
                DrawParameters,
                Surface,
            };

            lit_render_buffer.clear();
            highlight_render_buffer.clear();

            // Add some chessboard squares
            for y in 0..8 {
                for x in 0..8 {
                    let position = vec3(-3.5, 0.0, -3.5) + vec3(x, 0, y).as_f32();
                    let color = match (x + y) % 2 {
                        0 => Vec4::from_slice(&config.colors.black).as_f32(),
                        _ => Vec4::from_slice(&config.colors.white).as_f32(),
                    };
                    let mvp_matrix =
                        view_projection_matrix * Mat4::translation(position);
                    lit_render_buffer.push(RenderCommand {
                        mesh: &cube_mesh,
                        color,
                        mvp_matrix,
                    });
                }
            }

            // Sell square
            let position = chessjam::grid_to_world(sell_tile);
            lit_render_buffer.push(RenderCommand {
                mesh: &cube_mesh,
                color: vec4(0.5, 1.0, 0.5, 1.0),
                mvp_matrix: view_projection_matrix * Mat4::translation(position),
            });

            // Buy squares
            for &tile in &buy_tiles {
                let position = chessjam::grid_to_world(tile);
                lit_render_buffer.push(RenderCommand {
                    mesh: &cube_mesh,
                    color: vec4(0.5, 0.5, 0.25, 1.0),
                    mvp_matrix: view_projection_matrix
                        * Mat4::translation(position),
                });
            }

            // Add some chess pieces
            let mesh_for_piece = |piece_type| match piece_type {
                PieceType::Pawn => &pawn_mesh,
                PieceType::King => &king_mesh,
                PieceType::Queen => &queen_mesh,
                PieceType::Bishop => &bishop_mesh,
                PieceType::Rook => &rook_mesh,
                PieceType::Knight => &knight_mesh,
            };

            for piece in &pieces {
                let color = match piece.color {
                    ChessColor::Black => {
                        Vec4::from_slice(&config.colors.grey).as_f32()
                    }
                    ChessColor::White => {
                        Vec4::from_slice(&config.colors.white).as_f32()
                    }
                };

                let mesh = mesh_for_piece(piece.piece_type);

                let position = chessjam::grid_to_world(piece.position);
                lit_render_buffer.push(RenderCommand {
                    mesh,
                    color,
                    mvp_matrix: view_projection_matrix
                        * Mat4::translation(position),
                });
            }

            for (index, &tile) in buy_tiles.iter().enumerate() {
                let piece_for_sale = pieces_for_sale[index];

                if let Some(piece_for_sale) = piece_for_sale {
                    let position = chessjam::grid_to_world(tile);
                    let mesh = mesh_for_piece(piece_for_sale);
                    let color = Vec4::from_slice(&config.colors.forsale).as_f32();
                    let mvp_matrix =
                        view_projection_matrix * Mat4::translation(position);
                    lit_render_buffer.push(RenderCommand {
                        mesh,
                        color,
                        mvp_matrix,
                    });
                }
            }

            // Add tile highlights
            let height_offset = vec3(0.0, 0.2, 0.0);

            if let ControlState::SelectedPieceIndex(index) = control_state {
                let position = pieces[index].position;
                let position = chessjam::grid_to_world(position) + height_offset;
                highlight_render_buffer.push(RenderCommand {
                    mesh: &cube_mesh,
                    color: Vec4::from_slice(&config.colors.selected).as_f32(),
                    mvp_matrix: view_projection_matrix
                        * Mat4::translation(position),
                });
            }

            let position = chessjam::grid_to_world(tile_cursor) + height_offset;
            highlight_render_buffer.push(RenderCommand {
                mesh: &cube_mesh,
                color: Vec4::from_slice(&config.colors.cursor).as_f32(),
                mvp_matrix: view_projection_matrix * Mat4::translation(position),
            });

            for &dest in &valid_destinations {
                let position = chessjam::grid_to_world(dest) + height_offset;
                highlight_render_buffer.push(RenderCommand {
                    mesh: &cube_mesh,
                    color: Vec4::from_slice(&config.colors.dest).as_f32(),
                    mvp_matrix: view_projection_matrix
                        * Mat4::translation(position),
                });
            }

            stopclock("buffers", timer, timesheet);


            let mut frame = display.draw();
            frame.clear_all_srgb((0.0, 0.0, 0.0, 1.0), 1.0, 0);

            let clear_color = Vec4::from_slice(&config.colors.sky)
                .as_f32()
                .as_tuple();
            frame.clear(
                Some(&viewport),
                Some(clear_color),
                true,
                None,
                None,
            );

            let draw_params = DrawParameters {
                depth: Depth {
                    test: DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                backface_culling: BackfaceCullingMode::CullClockwise,
                viewport: Some(viewport),
                ..Default::default()
            };

            stopclock("pre-draw", timer, timesheet);

            // Render all objects as if in shadow
            for command in &lit_render_buffer {
                let normal_matrix = Mat3::<f32>::identity();

                frame
                    .draw(
                        &command.mesh.vertices,
                        &command.mesh.indices,
                        &model_shader,
                        &uniform!{
                            transform: command.mvp_matrix.0,
                            normal_matrix: normal_matrix.0,
                            light_direction_matrix: light_direction_matrix.0,
                            light_color_matrix: shadow_color_matrix.0,
                            albedo: command.color.0,
                            view_vector: view_vector.0,
                            specular_power: config.light.specular_power as f32,
                            specular_color: [0.0, 0.0, 0.0_f32],
                        },
                        &draw_params,
                    )
                    .unwrap();
            }

            stopclock("dark-pass", timer, timesheet);

            let shadow_front_draw_params = DrawParameters {
                depth: Depth {
                    test: DepthTest::IfLess,
                    write: false,
                    ..Default::default()
                },
                color_mask: (false, false, false, false),
                stencil: Stencil {
                    depth_pass_operation_clockwise: StencilOperation::Increment,
                    ..Default::default()
                },
                backface_culling: BackfaceCullingMode::CullCounterClockwise,
                viewport: Some(viewport),
                ..Default::default()
            };

            let shadow_back_draw_params = DrawParameters {
                depth: Depth {
                    test: DepthTest::IfLess,
                    write: false,
                    ..Default::default()
                },
                color_mask: (false, false, false, false),
                stencil: Stencil {
                    depth_pass_operation_counter_clockwise:
                        StencilOperation::Decrement,
                    ..Default::default()
                },
                backface_culling: BackfaceCullingMode::CullClockwise,
                viewport: Some(viewport),
                ..Default::default()
            };

            // Render shadows: front then back
            for draw_params in &[
                shadow_front_draw_params,
                shadow_back_draw_params,
            ] {
                for command in &lit_render_buffer {
                    let model_space_shadow_direction = shadow_direction.retract();


                    frame
                        .draw(
                            &command.mesh.shadow_vertices,
                            &command.mesh.shadow_indices,
                            &shadow_shader,
                            &uniform!{
                                transform: command.mvp_matrix.0,
                                model_space_shadow_direction:
                                    model_space_shadow_direction.0,
                            },
                            draw_params,
                        )
                        .unwrap();
                }
                stopclock("shadow-pass", timer, timesheet);
            }


            // Render objects fully lit outside shadow volumes
            let fully_lit_draw_params = DrawParameters {
                depth: Depth {
                    test: DepthTest::IfLessOrEqual,
                    write: false,
                    ..Default::default()
                },
                stencil: Stencil {
                    test_counter_clockwise: StencilTest::IfEqual { mask: !0 },
                    reference_value_counter_clockwise: 0,
                    ..Default::default()
                },
                backface_culling: BackfaceCullingMode::CullClockwise,
                viewport: Some(viewport),
                ..Default::default()
            };

            for command in &lit_render_buffer {
                let normal_matrix = Mat3::<f32>::identity();
                let specular_color =
                    Vec3::from_slice(&config.light.specular_color).as_f32();

                frame
                    .draw(
                        &command.mesh.vertices,
                        &command.mesh.indices,
                        &model_shader,
                        &uniform!{
                            transform: command.mvp_matrix.0,
                            normal_matrix: normal_matrix.0,
                            light_direction_matrix: light_direction_matrix.0,
                            light_color_matrix: light_color_matrix.0,
                            albedo: command.color.0,
                            view_vector: view_vector.0,
                            specular_power: config.light.specular_power as f32,
                            specular_color: specular_color.0,
                        },
                        &fully_lit_draw_params,
                    )
                    .unwrap();
            }

            stopclock("light-pass", timer, timesheet);

            // Render highlights
            {
                let highlight_draw_params = DrawParameters {
                    depth: Depth {
                        test: DepthTest::IfLess,
                        write: false,
                        ..Default::default()
                    },
                    blend: Blend::alpha_blending(),
                    backface_culling: BackfaceCullingMode::CullClockwise,
                    viewport: Some(viewport),
                    ..Default::default()
                };

                for highlight in &highlight_render_buffer {
                    let normal_matrix = Mat3::<f32>::identity();

                    frame
                        .draw(
                            &highlight.mesh.vertices,
                            &highlight.mesh.indices,
                            &model_shader,
                            &uniform!{
                                transform: highlight.mvp_matrix.0,
                                normal_matrix: normal_matrix.0,
                                light_direction_matrix: light_direction_matrix.0,
                                light_color_matrix: light_color_matrix.0,
                                albedo: highlight.color.0,
                            },
                            &highlight_draw_params,
                        )
                        .unwrap();
                }
            }

            stopclock("highlight-pass", timer, timesheet);

            label_renderer.clear();
            world_label_renderer.clear();

            label_renderer.add_label(
                &format!("{:?}", whos_turn),
                Vec3::from_slice(&config.text.turnlabel.pos).as_f32(),
                (config.text.turnlabel.size / config.text.size as f64) as f32,
                &text_system,
                &font_texture,
            );

            label_renderer.add_label(
                &format!("FPS {}", (1.0 / dt).round()),
                vec3(7.5, -4.0, 0.0),
                0.1,
                &text_system,
                &font_texture,
            );

            label_renderer.add_label(
                &format!("White's coins: {}", white_coins),
                vec3(-7.0, 4.0, 0.0),
                0.2,
                &text_system,
                &font_texture,
            );

            label_renderer.add_label(
                &format!("Black's coins: {}", black_coins),
                vec3(5.5, 4.0, 0.0),
                0.2,
                &text_system,
                &font_texture,
            );


            for (i, line) in timesheet.lines().enumerate() {
                let y = -0.2 * i as f32;
                label_renderer.add_label(
                    line,
                    vec3(-7.9, y, 0.0),
                    0.1,
                    &text_system,
                    &font_texture,
                );
            }

            for (index, &tile) in buy_tiles.iter().enumerate() {
                let piece_for_sale = pieces_for_sale[index];

                if let Some(piece_for_sale) = piece_for_sale {
                    let price = piece_price(piece_for_sale).buy_price;

                    world_label_renderer.add_label(
                        &format!("${}", price),
                        chessjam::grid_to_world(tile) + vec3(0.0, 2.0, 0.0),
                        0.05,
                        &text_system,
                        &font_texture,
                        );
                }
            }

            timesheet.clear();


            let (vx, vy) = chessjam::viewport_stretch(
                display.get_framebuffer_dimensions(),
                viewport.width,
                viewport.height,
                ).as_tuple();

            let text_scale = Mat4::scale(
                vec4(2.0 / vx, 2.0 / vy, 1.0, 1.0)
                    / Vec4::from_slice(&config.text.viewport).as_f32(),
            );

            for &(ref label, pos, scale) in label_renderer.labels() {
                let scale = Mat4::scale(vec4(scale, scale, 1.0, 1.0));
                let label_transform = text_scale * Mat4::translation(pos) * scale;
                glium_text::draw(
                    &label,
                    &text_system,
                    &mut frame,
                    label_transform.0,
                    (1.0, 1.0, 1.0, 1.0),
                );
            }

            let text_scale = Mat4::scale(vec4(1.0 / vx, 1.0 / vy, 1.0, 1.0));

            for &(ref label, pos, scale) in world_label_renderer.labels() {
                let screen_pos = view_projection_matrix * pos.extend(1.0);
                let screen_pos = (screen_pos / screen_pos.0[3]).retract();
                let scale = Mat4::scale(vec4(scale / TARGET_ASPECT, scale, 1.0, 1.0));
                let label_transform = text_scale * Mat4::translation(screen_pos) * scale;
                glium_text::draw(
                    &label,
                    &text_system,
                    &mut frame,
                    label_transform.0,
                    (0.5, 1.0, 0.5, 1.0),
                );
            }


            stopclock("text-pass", timer, timesheet);

            frame.finish().unwrap();

            stopclock("end-frame", timer, timesheet);
        }
    }
}
