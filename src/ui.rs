use adequate_math::*;
use glium_text::{FontTexture, TextDisplay, TextSystem};


pub struct LabelRenderer<'a> {
    labels: Vec<(TextDisplay<&'a FontTexture>, Vec3<f32>, f32)>,
    label_count: usize,
}

impl<'a> LabelRenderer<'a> {
    pub fn new() -> Self {
        LabelRenderer {
            labels: Vec::new(),
            label_count: 0,
        }
    }

    pub fn clear(&mut self) {
        self.label_count = 0;
    }

    pub fn add_label(
        &mut self,
        text: &str,
        pos: Vec3<f32>,
        scale: f32,
        system: &TextSystem,
        font: &'a FontTexture,
    ) {
        if self.label_count < self.labels.len() {
            let index = self.label_count;

            let label = &mut self.labels[index];
            label.0.set_text(text);
            label.1 = pos;
            label.2 = scale;
        }
        else {
            let label = TextDisplay::new(system, font, text);
            self.labels.push((label, pos, scale));
        }
        self.label_count += 1;
    }

    pub fn labels(&self) -> &[(TextDisplay<&'a FontTexture>, Vec3<f32>, f32)] {
        &self.labels
    }
}
