#![allow(unknown_lints)] // TODO(***realname***): Can go when clippy lints go.

extern crate chessjam;

#[macro_use]
extern crate static_assets;
#[macro_use]
extern crate glium;

extern crate adequate_math;
extern crate glium_text;
extern crate pleco;
extern crate pleco_engine;
extern crate rand;
extern crate rodio;
extern crate wavefront_obj;

mod audio;
mod chess;
mod data;
mod graphics;
mod input;
mod ui;

use std::time::Instant;

use adequate_math::*;
use glium::{glutin::EventsLoop, Display};
use rodio::Device;

use chessjam::config::Config;
use data::*;
use input::*;


fn random_piece(config: &Config) -> PieceType {
    use rand::distributions::{IndependentSample, Weighted, WeightedChoice};

    let mut choices = [
        Weighted {
            weight: config.weights.pawn as u32,
            item: PieceType::Pawn,
        },
        Weighted {
            weight: config.weights.knight as u32,
            item: PieceType::Knight,
        },
        Weighted {
            weight: config.weights.rook as u32,
            item: PieceType::Rook,
        },
        Weighted {
            weight: config.weights.bishop as u32,
            item: PieceType::Bishop,
        },
        Weighted {
            weight: config.weights.queen as u32,
            item: PieceType::Queen,
        },
    ];
    let wc = WeightedChoice::new(&mut choices);
    let mut rng = rand::thread_rng();

    wc.ind_sample(&mut rng)
}


#[allow(unused_variables)]
fn stopclock(title: &str, last_tick: &mut Instant, buffer: &mut String) {
    #[cfg(debug_assertions)]
    {
        use std::fmt::Write;

        let now = Instant::now();
        let elapsed = now.duration_since(*last_tick);
        *last_tick = now;
        let elapsed_millis = f64::from(elapsed.subsec_nanos()) / 1_000_000.0;
        writeln!(buffer, "{}: {:.3}ms", title, elapsed_millis).unwrap();
    }
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

    let speaker = rodio::default_output_device().unwrap();

    loop {
        let rerun = run_game(display, &mut events_loop, &speaker);

        if !rerun {
            break;
        }
    }
}


#[allow(cyclomatic_complexity)]
fn run_game(
    display: &Display,
    events_loop: &mut EventsLoop,
    speaker: &Device,
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

    let ui_shader = graphics::create_shader(
        display,
        asset_str!("assets/shaders/ui.glsl").as_ref(),
    );

    let skyball_shader = graphics::create_shader(
        display,
        asset_str!("assets/shaders/skyball.glsl").as_ref(),
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

    let table_mesh = graphics::create_obj_mesh(
        display,
        asset_str!("assets/meshes/table.obj").as_ref(),
    );

    let skyball_mesh = graphics::create_obj_mesh(
        display,
        asset_str!("assets/meshes/skyball.obj").as_ref(),
    );

    let quad_mesh = graphics::create_obj_mesh(
        display,
        asset_str!("assets/meshes/quad.obj").as_ref(),
    );

    let coin_mesh = graphics::create_obj_mesh(
        display,
        asset_str!("assets/meshes/coin.obj").as_ref(),
    );

    let white_texture = graphics::create_texture(
        display,
        asset_bytes!("assets/textures/white.png").as_ref(),
    );

    let checker_texture = graphics::create_texture(
        display,
        asset_bytes!("assets/textures/checker.png").as_ref(),
    );

    let wood_texture = graphics::create_texture(
        display,
        asset_bytes!("assets/textures/wood.png").as_ref(),
    );

    let black_marble_texture = graphics::create_texture(
        display,
        asset_bytes!("assets/textures/marble_black.png").as_ref(),
    );

    let white_marble_texture = graphics::create_texture(
        display,
        asset_bytes!("assets/textures/marble_white.png").as_ref(),
    );

    let plastic_marble_texture = graphics::create_texture(
        display,
        asset_bytes!("assets/textures/marble_plastic.png").as_ref(),
    );

    let skyball_texture = graphics::create_texture(
        display,
        asset_bytes!("assets/textures/skyball.png").as_ref(),
    );

    let ui_frame_texture = graphics::create_texture(
        display,
        asset_bytes!("assets/textures/ui_frame.png").as_ref(),
    );

    let ui_white_tile = graphics::create_texture(
        display,
        asset_bytes!("assets/textures/ui_tile_white.png").as_ref(),
    );

    let ui_black_tile = graphics::create_texture(
        display,
        asset_bytes!("assets/textures/ui_tile_black.png").as_ref(),
    );

    let coin_icon = graphics::create_texture(
        display,
        asset_bytes!("assets/textures/coin_icon.png").as_ref(),
    );


    // Start playing music
    let mut music = audio::play_music(
        speaker,
        &asset_bytes!("assets/music/the_line.ogg"),
    );

    let tap_sound = asset_bytes!("assets/audio/tap.ogg");
    let coin_sound = asset_bytes!("assets/audio/coins.ogg");

    let text_system = TextSystem::new(display);
    let font = Cursor::new(asset_bytes!("assets/fonts/bombardier.ttf"));
    let font_texture =
        FontTexture::new(display, font, config.text.size as u32).unwrap();

    let mut label_renderer = LabelRenderer::new();
    let mut price_tag_renderer = LabelRenderer::new();

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

    let shadow_direction = Vec4(config.light.key_dir).norm();

    let light_direction_matrix: Mat4<f32> = {
        let key = shadow_direction;
        let fill = Vec4(config.light.fill_dir).norm();
        let back = Vec4(config.light.back_dir).norm();

        Mat4([key.0, fill.0, back.0, [0.0, 0.0, 0.0, 1.0]]).transpose()
    };

    let light_color_matrix: Mat4<f32> = Mat4([
        config.light.key_color,
        config.light.fill_color,
        config.light.back_color,
        config.light.amb_color,
    ]);

    let shadow_color_matrix: Mat4<f32> = Mat4([
        config.shadow.key_color,
        config.shadow.fill_color,
        config.shadow.back_color,
        config.shadow.amb_color,
    ]);

    let sell_tile = Vec2(config.game.sell_tile).as_i32();
    let buy_tiles = config
        .game
        .buy_tiles
        .iter()
        .map(|&slice| Vec2(slice))
        .collect::<Vec<Vec2<i32>>>();

    let mut pieces = {
        let mut pieces = Vec::new();
        for x in 0..8 {
            pieces.push(Piece {
                position: vec2(x, 1),
                color: ChessColor::White,
                piece_type: PieceType::Pawn,
                moved: false,
                animation: None,
                delete_after_animation: false,
            });
            pieces.push(Piece {
                position: vec2(x, 6),
                color: ChessColor::Black,
                piece_type: PieceType::Pawn,
                moved: false,
                animation: None,
                delete_after_animation: false,
            });
        }
        pieces.push(Piece {
            position: vec2(4, 0),
            color: ChessColor::White,
            piece_type: PieceType::King,
            moved: false,
            animation: None,
            delete_after_animation: false,
        });
        pieces.push(Piece {
            position: vec2(4, 7),
            color: ChessColor::Black,
            piece_type: PieceType::King,
            moved: false,
            animation: None,
            delete_after_animation: false,
        });
        pieces
    };

    let mut pieces_for_sale = [
        Some(PieceForSale {
            piece_type: random_piece(&config),
            discounted: false,
        }),
        Some(PieceForSale {
            piece_type: random_piece(&config),
            discounted: false,
        }),
        Some(PieceForSale {
            piece_type: random_piece(&config),
            discounted: false,
        }),
    ];

    let mut white_coins = 0;
    let mut black_coins = 0;
    let mut game_outcome = GameOutcome::Ongoing;

    let mut control_state = ControlState::Idle;
    let mut valid_destinations: Vec<Vec2<i32>> = vec![];
    let mut whos_turn = ChessColor::White;
    let ai_player = Some(ChessColor::Black);
    let mut ai_pawns_to_sell = {
        use rand::distributions::{IndependentSample, Range};

        let between = Range::new(4, 8);
        let mut rng = rand::thread_rng();
        between.ind_sample(&mut rng)
    };

    let mut lit_render_buffer = Vec::new();
    let mut highlight_render_buffer = Vec::new();

    // Profiling
    let mut timer = Instant::now();
    let mut stats_text = String::new();
    let mut show_stats = true;
    let start_time = Instant::now();
    let mut game_end_time = None;

    loop {
        let (dt, now) = chessjam::delta_time(frame_time);
        let elapsed = chessjam::elapsed_time(start_time);
        frame_time = now;
        let timer = &mut timer;
        let stats_text = &mut stats_text;

        if let Some(game_end_time) = game_end_time {
            let elapsed = chessjam::elapsed_time(game_end_time);
            let volume = (3.0 - elapsed).max(0.0) / 3.0;
            music.set_volume(volume);
        }

        stopclock("between-frames", timer, stats_text);

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

        stopclock("inputs", timer, stats_text);

        if closed || keyboard.pressed(Key::Escape) {
            // TODO(***realname***): Find out why `return false` here crashes.
            std::process::exit(0);
        }
        if keyboard.pressed(Key::R) && keyboard.modifiers.logo {
            return true;
        }
        if keyboard.pressed(Key::H) {
            show_stats = !show_stats;
        }

        camera_angle += camera_motion.0[0] * dt;
        camera_tilt += camera_motion.0[1] * dt;
        camera_tilt = camera_tilt.min(89.0).max(10.0);

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

        let (vx, vy) = chessjam::viewport_stretch(
            display.get_framebuffer_dimensions(),
            viewport.width,
            viewport.height,
        ).as_tuple();

        // TODO(***realname***): Why are these two matrices not interchangeable?
        let text_projection = Mat4::scale(
            vec4(2.0 / vx, 2.0 / vy, 1.0, 1.0)
                / Vec4(config.text.viewport),
        );

        let ui_projection = matrix::ortho_projection(TARGET_ASPECT, 4.5, -1.0, 1.0);

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

        stopclock("pre-update", timer, stats_text);

        let mut valid_purchase_placements = Vec::new();
        let mut can_sell = false;

        // update
        {
            use pleco::Board;

            match control_state {
                ControlState::SelectedPieceIndex(index) => {
                    can_sell = pieces[index].piece_type != PieceType::King;
                }
                ControlState::SelectedPurchaseIndex(index) => {
                    let piece_for_sale = pieces_for_sale[index];

                    if let Some(piece_for_sale) = piece_for_sale {
                        valid_purchase_placements =
                            chess::valid_purchase_placements(
                                &pieces,
                                piece_for_sale.piece_type,
                                whos_turn,
                            );
                    }
                }
                ControlState::Idle => (),
            }

            // animation
            let animations_playing = {
                let mut animating = false;
                let mut trash = Vec::new();

                for (index, piece) in pieces.iter_mut().enumerate() {
                    let mut anim_done = false;

                    if let Some(ref mut anim) = piece.animation {
                        animating = true;
                        anim.t += dt;
                        if anim.t > 1.0 {
                            anim_done = true;
                        }
                    }

                    if anim_done {
                        piece.animation = None;

                        if piece.delete_after_animation {
                            trash.push(index);
                            audio::play_sound(speaker, &coin_sound);
                        }
                        else {
                            audio::play_sound(speaker, &tap_sound);
                        }
                    }
                }

                trash.reverse();
                for index in trash {
                    pieces.swap_remove(index);
                }

                animating
            };

            stopclock("animation", timer, stats_text);


            // Player actions
            let mut player_move = None;
            let mut piece_promotion = None;
            let mut piece_to_sell = None;
            let mut player_purchase = None;

            let allow_player_actions =
                game_outcome == GameOutcome::Ongoing && !animations_playing;

            if allow_player_actions {
                if ai_player == Some(whos_turn) {
                    if ai_pawns_to_sell > 0 {
                        let pawns = pieces
                            .iter()
                            .enumerate()
                            .filter(|(_, p)| {
                                p.color == whos_turn
                                    && p.piece_type == PieceType::Pawn
                            })
                            .map(|(i, _)| i)
                            .collect::<Vec<usize>>();

                        let mut rng = rand::thread_rng();
                        let index = rand::seq::sample_slice(&mut rng, &pawns, 1)[0];
                        piece_to_sell = Some(index);
                        ai_pawns_to_sell -= 1;
                    }
                    else {
                        let coins = match whos_turn {
                            ChessColor::Black => black_coins,
                            ChessColor::White => white_coins,
                        };

                        let best_purchase = pieces_for_sale
                            .iter()
                            .enumerate()
                            .filter_map(|(index, piece)| piece.map(|p| (index, p)))
                            .map(|(index, piece)| {
                                (index, piece.piece_type, chess::buy_price(piece))
                            })
                            .filter(|(_, _, price)| *price <= coins)
                            .max_by_key(|(_, piece, _)| *piece)
                            .map(|(index, _, _)| index);

                        if let Some(index) = best_purchase {
                            let piece_for_sale = pieces_for_sale[index].unwrap();
                            let valid_purchase_placements =
                                chess::valid_purchase_placements(
                                    &pieces,
                                    piece_for_sale.piece_type,
                                    whos_turn,
                                );
                            let mut rng = rand::thread_rng();
                            let place = rand::seq::sample_slice(
                                &mut rng,
                                &valid_purchase_placements,
                                1,
                            )[0];
                            player_purchase = Some((index, place));
                        }
                        else {
                            let mov = chess::decide_move(&pieces, whos_turn);
                            let from = chessjam::grid_from_u8(mov.get_src_u8());
                            let to = chessjam::grid_from_u8(mov.get_dest_u8());
                            player_move = Some((from, to));

                            if mov.is_promo() {
                                use pleco::PieceType::*;

                                let piece = match mov.promo_piece() {
                                    Q => PieceType::Queen,
                                    R => PieceType::Rook,
                                    B => PieceType::Bishop,
                                    N => PieceType::Knight,
                                    P => PieceType::Pawn,
                                    _ => unreachable!(
                                        "Invalid promotion was attempted."
                                    ),
                                };

                                piece_promotion = Some(piece);
                            }
                        }
                    }
                }
                else if mouse.pressed(Button::Left) {
                    match control_state {
                        ControlState::Idle => {
                            for (index, &tile) in buy_tiles.iter().enumerate() {
                                if tile == tile_cursor {
                                    control_state =
                                        ControlState::SelectedPurchaseIndex(index);
                                }
                            }
                            control_state = match chess::piece_at(
                                tile_cursor,
                                &pieces,
                            ) {
                                Some(index) if pieces[index].color == whos_turn => {
                                    ControlState::SelectedPieceIndex(index)
                                }
                                _ => control_state,
                            };
                        }
                        ControlState::SelectedPieceIndex(index) => {
                            if valid_destinations.contains(&tile_cursor) {
                                let from_tile = pieces[index].position;
                                let to_tile = tile_cursor;
                                player_move = Some((from_tile, to_tile));
                            }
                            else if tile_cursor == sell_tile && can_sell
                                && pieces[index].color == whos_turn
                            {
                                piece_to_sell = Some(index);
                            }
                            control_state = ControlState::Idle;
                        }
                        ControlState::SelectedPurchaseIndex(index) => {
                            let piece_type = pieces_for_sale[index];

                            if piece_type.is_some() {
                                if valid_purchase_placements.contains(&tile_cursor)
                                {
                                    player_purchase = Some((index, tile_cursor));
                                }
                            }

                            control_state = ControlState::Idle;
                        }
                    }

                    // Recalculate possible moves
                    // TODO(***realname***): Put this at an outer scope, invalidate it safely
                    valid_destinations.clear();
                    if let ControlState::SelectedPieceIndex(index) = control_state {
                        let piece = &pieces[index];
                        let (px, py) = piece.position.as_tuple();
                        let piece_pos_u8 = (py * 8 + px) as u8;

                        let fen = chess::generate_fen(&pieces, whos_turn);
                        let board = Board::from_fen(&fen).unwrap();
                        let moves = board.generate_moves();

                        for chessmove in moves.iter() {
                            let from_u8 = chessmove.get_src_u8();
                            let to_u8 = chessmove.get_dest_u8();
                            let dest = chessjam::grid_from_u8(to_u8);
                            if from_u8 == piece_pos_u8 {
                                valid_destinations.push(dest);
                            }
                        }
                    }
                }
            }

            if let Some((from, to)) = player_move {
                {
                    let moved_index = chess::piece_at(from, &pieces).unwrap();
                    let taken_index = chess::piece_at(to, &pieces);
                    pieces[moved_index].position = to;
                    pieces[moved_index].moved = true;
                    pieces[moved_index].animation = Some(Animation {
                        from,
                        to,
                        t: 0.0,
                    });

                    if pieces[moved_index].piece_type == PieceType::Pawn
                        && (to.0[1] == 0 || to.0[1] == 7)
                    {
                        pieces[moved_index].piece_type = PieceType::Queen;
                    }

                    if let Some(promotion) = piece_promotion {
                        pieces[moved_index].piece_type = promotion;
                    }

                    if let Some(index) = taken_index {
                        if pieces[index].color == whos_turn {
                            // This must be a castle
                            pieces[index].position = from;
                            pieces[index].moved = true;
                            pieces[index].animation = Some(Animation {
                                from: to,
                                to: from,
                                t: 0.0,
                            });
                        }
                        else {
                            let refund = chess::sell_price(
                                pieces[index].piece_type,
                                pieces[index].moved,
                            );

                            pieces[index].animation = Some(Animation {
                                from: to,
                                to: sell_tile,
                                t: 0.0,
                            });
                            pieces[index].position = sell_tile;
                            pieces[index].delete_after_animation = true;

                            match pieces[index].color {
                                ChessColor::White => white_coins += refund,
                                ChessColor::Black => black_coins += refund,
                            }
                        }
                    }
                }

                let prev_turn = whos_turn;
                whos_turn = match whos_turn {
                    ChessColor::White => ChessColor::Black,
                    ChessColor::Black => ChessColor::White,
                };

                let fen = chess::generate_fen(&pieces, whos_turn);
                let board = Board::from_fen(&fen).unwrap();

                if board.checkmate() {
                    game_outcome = GameOutcome::Victory(prev_turn);
                    game_end_time = Some(Instant::now());
                }
                else if board.stalemate() {
                    game_outcome = GameOutcome::Stalemate;
                }

                // Restock shop
                for piece in &mut pieces_for_sale {
                    match piece {
                        Some(piece) => piece.discounted = true,
                        None => {
                            *piece = Some(PieceForSale {
                                piece_type: random_piece(&config),
                                discounted: false,
                            });
                        }
                    }
                }
            }

            if let Some(index) = piece_to_sell {
                let refund = chess::sell_price(
                    pieces[index].piece_type,
                    pieces[index].moved,
                );

                pieces[index].animation = Some(Animation {
                    from: pieces[index].position,
                    to: sell_tile,
                    t: 0.0,
                });
                pieces[index].position = sell_tile;
                pieces[index].delete_after_animation = true;

                match whos_turn {
                    ChessColor::White => white_coins += refund,
                    ChessColor::Black => black_coins += refund,
                }
            }

            if let Some((index, place)) = player_purchase {
                let piece_for_sale = pieces_for_sale[index].unwrap();

                let price = chess::buy_price(piece_for_sale);

                let wallet = match whos_turn {
                    ChessColor::White => &mut white_coins,
                    ChessColor::Black => &mut black_coins,
                };
                if price <= *wallet {
                    *wallet -= price;
                    pieces.push(Piece {
                        position: place,
                        color: whos_turn,
                        piece_type: piece_for_sale.piece_type,
                        moved: false,
                        animation: Some(Animation {
                            from: buy_tiles[index],
                            to: place,
                            t: 0.0,
                        }),
                        delete_after_animation: false,
                    });
                    pieces_for_sale[index] = None;
                }
            }
        }

        stopclock("update", timer, stats_text);


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

            let specular_color =
                Vec3(config.light.specular_color);

            let saturation: f32 = match game_outcome {
                GameOutcome::Ongoing => 1.0,
                _ => 0.0,
            };

            lit_render_buffer.clear();
            highlight_render_buffer.clear();

            // Add some chessboard squares
            for y in 0..8 {
                for x in 0..8 {
                    let position = vec3(-3.5, 0.0, -3.5) + vec3(x, 0, y).as_f32();
                    //                     let color = match (x + y) % 2 {
                    //                         0 => Vec4(config.colors.black),
                    //                         _ => Vec4(config.colors.white),
                    //                     };
                    let texture = match (x + y) % 2 {
                        0 => &black_marble_texture,
                        _ => &white_marble_texture,
                    };
                    let mvp_matrix =
                        view_projection_matrix * Mat4::translation(position);
                    lit_render_buffer.push(RenderCommand {
                        mesh: &cube_mesh,
                        color: vec4(0.9, 0.9, 0.9, 1.0),
                        mvp_matrix,
                        colormap: texture,
                        texture_scale: vec3(1.0, 1.0, 1.0),
                        texture_offset: vec3(0.0, 0.0, 0.0),
                    });
                }
            }

            // Sell square
            let position = chessjam::grid_to_world(sell_tile);
            lit_render_buffer.push(RenderCommand {
                mesh: &cube_mesh,
                color: vec4(0.5, 1.0, 0.5, 1.0),
                mvp_matrix: view_projection_matrix * Mat4::translation(position),
                colormap: &white_texture,
                texture_scale: vec3(1.0, 1.0, 1.0),
                texture_offset: vec3(0.0, 0.0, 0.0),
            });

            // Buy squares
            for &tile in &buy_tiles {
                let position = chessjam::grid_to_world(tile);
                lit_render_buffer.push(RenderCommand {
                    mesh: &cube_mesh,
                    color: vec4(0.5, 0.5, 0.25, 1.0),
                    mvp_matrix: view_projection_matrix
                        * Mat4::translation(position),
                    colormap: &white_texture,
                    texture_scale: vec3(1.0, 1.0, 1.0),
                    texture_offset: vec3(0.0, 0.0, 0.0),
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
                let (texture_scale, texture) = match piece.color {
                    ChessColor::Black => {
                        (vec3(1.0, 1.0, 1.0), &black_marble_texture)
                    }
                    ChessColor::White => {
                        (vec3(2.0, 1.0, 2.0), &white_marble_texture)
                    }
                };

                let mesh = mesh_for_piece(piece.piece_type);

                let position = match piece.animation {
                    Some(ref anim) => {
                        let t = anim.t;
                        let from_down = chessjam::grid_to_world(anim.from);
                        let to_down = chessjam::grid_to_world(anim.to);
                        let from_up = from_down + vec3(0.0, 2.0, 0.0);
                        let to_up = to_down + vec3(0.0, 2.0, 0.0);

                        let sink = if anim.to == sell_tile {
                            vec3(0.0, -2.0, 0.0)
                        }
                        else {
                            vec3(0.0, 0.0, 0.0)
                        };
                        let to_down = to_down + sink;

                        let (from, to, t) = match t {
                            t if t < 0.33 => (from_down, from_up, t * 3.0),
                            t if t < 0.66 => (from_up, to_up, (t - 0.33) * 3.0),
                            t => (to_up, to_down, (t - 0.66) * 3.0),
                        };

                        let t = t.powf(0.5);

                        math::lerp(from, to, t)
                    }
                    None => chessjam::grid_to_world(piece.position),
                };

                lit_render_buffer.push(RenderCommand {
                    mesh,
                    color: vec4(1.0, 1.0, 1.0, 1.0),
                    mvp_matrix: view_projection_matrix
                        * Mat4::translation(position),
                    colormap: texture,
                    texture_scale,
                    texture_offset: vec3(0.5, 0.0, 0.5),
                });
            }

            for (index, &tile) in buy_tiles.iter().enumerate() {
                let piece_for_sale = pieces_for_sale[index];

                if let Some(piece_for_sale) = piece_for_sale {
                    let position = chessjam::grid_to_world(tile);
                    let mesh = mesh_for_piece(piece_for_sale.piece_type);
                    let color = Vec4(config.colors.forsale);
                    let mvp_matrix =
                        view_projection_matrix * Mat4::translation(position);
                    lit_render_buffer.push(RenderCommand {
                        mesh,
                        color,
                        mvp_matrix,
                        colormap: &plastic_marble_texture,
                        texture_scale: vec3(2.0, 2.0, 2.0),
                        texture_offset: vec3(0.5, 0.0, 0.5),
                    });
                }
            }

            lit_render_buffer.push(RenderCommand {
                mesh: &table_mesh,
                color: vec4(1.0, 1.0, 1.0, 1.0),
                mvp_matrix: view_projection_matrix
                    * Mat4::translation(vec3(0.0, -0.2, 0.0)),
                colormap: &wood_texture,
                texture_scale: vec3(1.0 / 32.0, 1.0, 1.0 / 16.0),
                texture_offset: vec3(0.5, 0.0, 0.5),
            });

            // Add tile highlights
            let height_offset = vec3(0.0, 0.2, 0.0);

            let selection_tile = match control_state {
                ControlState::SelectedPieceIndex(index) => {
                    Some(pieces[index].position)
                }
                ControlState::SelectedPurchaseIndex(index) => {
                    Some(buy_tiles[index])
                }
                ControlState::Idle => None,
            };

            if let Some(position) = selection_tile {
                let position = chessjam::grid_to_world(position) + height_offset;
                highlight_render_buffer.push(RenderCommand {
                    mesh: &cube_mesh,
                    color: Vec4(config.colors.selected),
                    mvp_matrix: view_projection_matrix
                        * Mat4::translation(position),
                    colormap: &white_texture,
                    texture_scale: vec3(1.0, 1.0, 1.0),
                    texture_offset: vec3(0.0, 0.0, 0.0),
                });
            }

            let position = chessjam::grid_to_world(tile_cursor) + height_offset;
            highlight_render_buffer.push(RenderCommand {
                mesh: &cube_mesh,
                color: Vec4(config.colors.cursor),
                mvp_matrix: view_projection_matrix * Mat4::translation(position),
                colormap: &white_texture,
                texture_scale: vec3(1.0, 1.0, 1.0),
                texture_offset: vec3(0.0, 0.0, 0.0),
            });

            for &dest in &valid_destinations {
                let position = chessjam::grid_to_world(dest) + height_offset;
                highlight_render_buffer.push(RenderCommand {
                    mesh: &cube_mesh,
                    color: Vec4(config.colors.dest),
                    mvp_matrix: view_projection_matrix
                        * Mat4::translation(position),
                    colormap: &white_texture,
                    texture_scale: vec3(1.0, 1.0, 1.0),
                    texture_offset: vec3(0.0, 0.0, 0.0),
                });
            }

            if can_sell {
                let position = chessjam::grid_to_world(sell_tile) + height_offset;
                highlight_render_buffer.push(RenderCommand {
                    mesh: &cube_mesh,
                    color: Vec4(config.colors.dest),
                    mvp_matrix: view_projection_matrix
                        * Mat4::translation(position),
                    colormap: &white_texture,
                    texture_scale: vec3(1.0, 1.0, 1.0),
                    texture_offset: vec3(0.0, 0.0, 0.0),
                });
            }

            for &place in &valid_purchase_placements {
                let position = chessjam::grid_to_world(place) + height_offset;
                highlight_render_buffer.push(RenderCommand {
                    mesh: &cube_mesh,
                    color: Vec4(config.colors.place),
                    mvp_matrix: view_projection_matrix
                        * Mat4::translation(position),
                    colormap: &white_texture,
                    texture_scale: vec3(1.0, 1.0, 1.0),
                    texture_offset: vec3(0.0, 0.0, 0.0),
                });
            }

            stopclock("buffers", timer, stats_text);


            let mut frame = display.draw();
            frame.clear_all_srgb((0.0, 0.0, 0.0, 1.0), 1.0, 0);

            let sky_draw_params = DrawParameters {
                depth: Depth {
                    test: DepthTest::Overwrite,
                    write: false,
                    ..Default::default()
                },
                backface_culling: BackfaceCullingMode::CullClockwise,
                viewport: Some(viewport),
                ..Default::default()
            };

            let skyball_transform =
                view_projection_matrix * Mat4::scale(vec4(40.0, 40.0, 40.0, 1.0));

            frame
                .draw(
                    &skyball_mesh.vertices,
                    &skyball_mesh.indices,
                    &skyball_shader,
                    &uniform!{
                        transform: skyball_transform.0,
                        colormap: &skyball_texture,
                        saturation: saturation,
                    },
                    &sky_draw_params,
                )
                .unwrap();

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

            stopclock("pre-draw", timer, stats_text);

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
                            texture_scale: command.texture_scale.0,
                            texture_offset: command.texture_offset.0,
                            colormap: command.colormap,
                            light_direction_matrix: light_direction_matrix.0,
                            light_color_matrix: shadow_color_matrix.0,
                            albedo: command.color.0,
                            view_vector: view_vector.0,
                            specular_power: config.light.specular_power as f32,
                            specular_color: [0.0, 0.0, 0.0_f32],
                            saturation: saturation,
                        },
                        &draw_params,
                    )
                    .unwrap();
            }

            stopclock("dark-pass", timer, stats_text);

            let shadow_front_draw_params = DrawParameters {
                depth: Depth {
                    test: DepthTest::IfLess,
                    write: false,
                    ..Default::default()
                },
                color_mask: (false, false, false, false),
                stencil: Stencil {
                    depth_pass_operation_counter_clockwise:
                        StencilOperation::Increment,
                    ..Default::default()
                },
                backface_culling: BackfaceCullingMode::CullClockwise,
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
                    depth_pass_operation_clockwise: StencilOperation::Decrement,
                    ..Default::default()
                },
                backface_culling: BackfaceCullingMode::CullCounterClockwise,
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
                stopclock("shadow-pass", timer, stats_text);
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

                frame
                    .draw(
                        &command.mesh.vertices,
                        &command.mesh.indices,
                        &model_shader,
                        &uniform!{
                            transform: command.mvp_matrix.0,
                            normal_matrix: normal_matrix.0,
                            texture_scale: command.texture_scale.0,
                            texture_offset: command.texture_offset.0,
                            colormap: command.colormap,
                            light_direction_matrix: light_direction_matrix.0,
                            light_color_matrix: light_color_matrix.0,
                            albedo: command.color.0,
                            view_vector: view_vector.0,
                            specular_power: config.light.specular_power as f32,
                            specular_color: specular_color.0,
                            saturation: saturation,
                        },
                        &fully_lit_draw_params,
                    )
                    .unwrap();
            }

            stopclock("light-pass", timer, stats_text);

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
                                texture_scale: highlight.texture_scale.0,
                                texture_offset: highlight.texture_offset.0,
                                colormap: highlight.colormap,
                                light_direction_matrix: light_direction_matrix.0,
                                light_color_matrix: light_color_matrix.0,
                                albedo: highlight.color.0,
                                view_vector: view_vector.0,
                                specular_power: config.light.specular_power as f32,
                                specular_color: [0.0, 0.0, 0.0_f32],
                                saturation: saturation,
                            },
                            &highlight_draw_params,
                        )
                        .unwrap();
                }
            }

            stopclock("highlight-pass", timer, stats_text);


            price_tag_renderer.clear();

            for (index, &tile) in buy_tiles.iter().enumerate() {
                let piece_for_sale = pieces_for_sale[index];

                if let Some(piece_for_sale) = piece_for_sale {
                    let price = chess::buy_price(piece_for_sale);

                    let tag = if piece_for_sale.discounted {
                        "SALE!"
                    }
                    else {
                        ""
                    };

                    price_tag_renderer.add_label(
                        &format!("{} {}", price, tag),
                        chessjam::grid_to_world(tile) + vec3(0.0, 1.75, 0.0),
                        0.05,
                        &text_system,
                        &font_texture,
                    );
                }
            }

            if let ControlState::SelectedPieceIndex(index) = control_state {
                if can_sell {
                    let piece = &pieces[index];
                    let refund = chess::sell_price(piece.piece_type, piece.moved);

                    price_tag_renderer.add_label(
                        &format!("{} Sell?", refund),
                        chessjam::grid_to_world(sell_tile) + vec3(0.0, 2.0, 0.0),
                        0.05,
                        &text_system,
                        &font_texture,
                    );
                }
            }

            let ui_draw_parameters = DrawParameters {
                depth: Depth {
                    test: DepthTest::Overwrite,
                    write: false,
                    ..Default::default()
                },
                blend: Blend::alpha_blending(),
                backface_culling: BackfaceCullingMode::CullClockwise,
                viewport: Some(viewport),
                ..Default::default()
            };

            let world_text_projection =
                Mat4::scale(vec4(1.0 / vx, 1.0 / vy, 1.0, 1.0));

            if game_outcome == GameOutcome::Ongoing {
                for &(ref label, pos, scale) in price_tag_renderer.labels() {
                    let screen_pos = view_projection_matrix * pos.extend(1.0);
                    let screen_pos = (screen_pos / screen_pos.0[3]).retract();
                    let shadow_pos = screen_pos + vec3(0.0, -0.008, 0.0);

                    let icon_scale = Mat4::scale(vec4(
                        1.2 * scale / TARGET_ASPECT,
                        1.2 * scale,
                        1.0,
                        1.0,
                    ));
                    let scale =
                        Mat4::scale(vec4(scale / TARGET_ASPECT, scale, 1.0, 1.0));
                    let label_transform = world_text_projection
                        * Mat4::translation(screen_pos)
                        * scale;
                    let shadow_transform = world_text_projection
                        * Mat4::translation(shadow_pos)
                        * scale;
                    let icon_transform = world_text_projection
                        * Mat4::translation(screen_pos + vec3(-0.02, 0.02, 0.0))
                        * icon_scale;
                    let shadow_icon_transform = world_text_projection
                        * Mat4::translation(shadow_pos + vec3(-0.02, 0.02, 0.0))
                        * icon_scale;

                    let color = vec4(0.5, 1.0, 0.5, 1.0_f32);

                    frame
                        .draw(
                            &quad_mesh.vertices,
                            &quad_mesh.indices,
                            &ui_shader,
                            &uniform!{
                                transform: shadow_icon_transform.0,
                                colormap: &coin_icon,
                                tint: [0.0, 0.0, 0.0, 0.6_f32],
                            },
                            &ui_draw_parameters,
                        )
                        .unwrap();

                    frame
                        .draw(
                            &quad_mesh.vertices,
                            &quad_mesh.indices,
                            &ui_shader,
                            &uniform!{
                                transform: icon_transform.0,
                                colormap: &coin_icon,
                                tint: color.0,
                            },
                            &ui_draw_parameters,
                        )
                        .unwrap();

                    glium_text::draw(
                        &label,
                        &text_system,
                        &mut frame,
                        shadow_transform.0,
                        (0.0, 0.0, 0.0, 0.6),
                    );

                    glium_text::draw(
                        &label,
                        &text_system,
                        &mut frame,
                        label_transform.0,
                        color.as_tuple(),
                    );
                }
            }

            stopclock("world-text-pass", timer, stats_text);

            let (black_turn_pos, white_turn_pos) = match game_outcome {
                GameOutcome::Ongoing => match whos_turn {
                    ChessColor::Black => (vec3(0.0, 4.6, 0.0), vec3(0.0, 7.0, 0.0)),
                    ChessColor::White => (vec3(0.0, 7.0, 0.0), vec3(0.0, 4.6, 0.0)),
                },
                GameOutcome::Stalemate => {
                    (vec3(-1.0, 3.0, 0.0), vec3(1.0, 3.0, 0.0))
                }
                GameOutcome::Victory(winner) => match winner {
                    ChessColor::Black => (vec3(0.0, 3.0, 0.0), vec3(0.0, 7.0, 0.0)),
                    ChessColor::White => (vec3(0.0, 7.0, 0.0), vec3(0.0, 3.0, 0.0)),
                },
            };

            let ui_render_commands = {
                let mut game_ui = vec![
                    UiRenderCommand {
                        colormap: &ui_frame_texture,
                        pos: vec3(-6.0, 3.0, 0.0),
                        scale: 1.5,
                        angle: -consts::TAU32 / 8.0,
                    },
                    UiRenderCommand {
                        colormap: &ui_frame_texture,
                        pos: vec3(6.0, 3.0, 0.0),
                        scale: 1.5,
                        angle: -consts::TAU32 / 8.0,
                    },
                    UiRenderCommand {
                        colormap: &ui_white_tile,
                        pos: vec3(-6.0, 3.7, 0.0),
                        scale: 0.8,
                        angle: -consts::TAU32 / 8.0,
                    },
                    UiRenderCommand {
                        colormap: &ui_black_tile,
                        pos: vec3(6.0, 3.7, 0.0),
                        scale: 0.8,
                        angle: -consts::TAU32 / 8.0,
                    },
                ];

                let mut ui_render_commands = vec![
                    UiRenderCommand {
                        colormap: &ui_white_tile,
                        pos: white_turn_pos,
                        scale: 1.2,
                        angle: -consts::TAU32 / 8.0,
                    },
                    UiRenderCommand {
                        colormap: &ui_black_tile,
                        pos: black_turn_pos,
                        scale: 1.2,
                        angle: -consts::TAU32 / 8.0,
                    },
                ];

                if game_outcome == GameOutcome::Ongoing {
                    ui_render_commands.append(&mut game_ui);
                }

                ui_render_commands
            };

            for command in &ui_render_commands {
                let transform = ui_projection * Mat4::translation(command.pos)
                    * matrix::euler_rotation(vec3(0.0, 0.0, command.angle))
                    * Mat4::scale(vec4(command.scale, command.scale, 1.0, 1.0));

                frame
                    .draw(
                        &quad_mesh.vertices,
                        &quad_mesh.indices,
                        &ui_shader,
                        &uniform!{
                            colormap: command.colormap,
                            tint: [1.0, 1.0, 1.0, 1.0_f32],
                            transform: transform.0,
                        },
                        &ui_draw_parameters,
                    )
                    .unwrap();
            }

            if game_outcome == GameOutcome::Ongoing {
                let coin_positions = &[vec3(-6.45, 3.0, 0.0), vec3(5.55, 3.0, 0.0)];

                for &coin_pos in coin_positions {
                    let coin_scale = 0.6;

                    let transform = ui_projection * Mat4::translation(coin_pos)
                        * matrix::euler_rotation(vec3(0.0, -2.0 * elapsed, 0.0))
                        * Mat4::scale(vec4(
                            coin_scale, coin_scale, coin_scale, 1.0,
                        ));

                    frame
                        .draw(
                            &coin_mesh.vertices,
                            &coin_mesh.indices,
                            &model_shader,
                            &uniform!{
                                transform: transform.0,
                                normal_matrix: Mat3::<f32>::identity().0,
                                texture_scale: vec3(1.0, 1.0, 1.0_f32).0,
                                texture_offset: vec3(0.5, 0.5, 0.25_f32).0,
                                colormap: &checker_texture,
                                light_direction_matrix: light_direction_matrix.0,
                                light_color_matrix: light_color_matrix.0,
                                albedo: vec4(1.2, 1.2, 1.2, 1.0_f32).0,
                                view_vector: vec3(0.0, 0.0, 1.0_f32).0,
                                specular_power: config.light.specular_power as f32,
                                specular_color: specular_color.0,
                                saturation: saturation,
                            },
                            &DrawParameters {
                                depth: Depth {
                                    test: DepthTest::Overwrite,
                                    write: false,
                                    ..Default::default()
                                },
                                blend: Blend::alpha_blending(),
                                backface_culling:
                                    BackfaceCullingMode::CullClockwise,
                                viewport: Some(viewport),
                                ..Default::default()
                            },
                        )
                        .unwrap();
                }
            };

            stopclock("ui-pass", timer, stats_text);

            label_renderer.clear();

            if show_stats {
                label_renderer.add_label(
                    &format!("FPS {}", (1.0 / dt).round()),
                    vec3(7.5, -4.0, 0.0),
                    0.1,
                    &text_system,
                    &font_texture,
                );
            }

            if game_outcome == GameOutcome::Ongoing {
                label_renderer.add_label(
                    &white_coins.to_string(),
                    vec3(-5.8, 2.8, 0.0),
                    0.4,
                    &text_system,
                    &font_texture,
                );

                label_renderer.add_label(
                    &black_coins.to_string(),
                    vec3(6.2, 2.8, 0.0),
                    0.4,
                    &text_system,
                    &font_texture,
                );
            }


            #[cfg(debug_assertions)]
            {
                if show_stats {
                    for (i, line) in stats_text.lines().enumerate() {
                        let y = -0.2 * i as f32;
                        label_renderer.add_label(
                            line,
                            vec3(-7.9, y, 0.0),
                            0.1,
                            &text_system,
                            &font_texture,
                        );
                    }
                }
            }

            let status_label = match game_outcome {
                GameOutcome::Ongoing => "".into(),
                GameOutcome::Stalemate => "Stalemate".into(),
                GameOutcome::Victory(x) => format!("Checkmate: {:?} wins", x),
            };

            label_renderer.add_label(
                &status_label,
                vec3(-7.9, -4.4, 0.0),
                0.5,
                &text_system,
                &font_texture,
            );

            stats_text.clear();
            {
                use std::fmt::Write;

                writeln!(
                    stats_text,
                    "Resolution: {:?}",
                    display.get_framebuffer_dimensions()
                ).unwrap();
            }

            for &(ref label, pos, scale) in label_renderer.labels() {
                let scale = Mat4::scale(vec4(scale, scale, 1.0, 1.0));
                let label_transform =
                    text_projection * Mat4::translation(pos) * scale;
                glium_text::draw(
                    &label,
                    &text_system,
                    &mut frame,
                    label_transform.0,
                    (1.0, 1.0, 1.0, 1.0),
                );
            }


            stopclock("text-pass", timer, stats_text);

            frame.finish().unwrap();

            stopclock("end-frame", timer, stats_text);
        }
    }
}
