use adequate_math::*;
use glium::texture::SrgbTexture2d;
use graphics::Mesh;


pub struct RenderCommand<'a> {
    pub mesh: &'a Mesh,
    pub color: Vec4<f32>,
    pub mvp_matrix: Mat4<f32>,
    pub colormap: &'a SrgbTexture2d,
    pub texture_scale: Vec3<f32>,
    pub texture_offset: Vec3<f32>,
}

pub struct UiRenderCommand<'a> {
    pub colormap: &'a SrgbTexture2d,
    pub pos: Vec3<f32>,
    pub scale: f32,
    pub angle: f32,
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PieceType {
    Pawn,
    Knight,
    Rook,
    Bishop,
    Queen,
    King,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GameOutcome {
    Ongoing,
    Stalemate,
    Victory(ChessColor),
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
