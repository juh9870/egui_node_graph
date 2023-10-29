use egui::epaint::{Vertex, WHITE_UV};
use egui::{Color32, Pos2, Rect, Stroke, Ui};

pub fn draw_circle_port(
    ui: &mut Ui,
    wide_port: bool,
    port_rect: Rect,
    zoom: f32,
    port_color: Color32,
) {
    if wide_port {
        ui.painter().rect_filled(port_rect, 5.0 * zoom, port_color);
    } else {
        ui.painter()
            .circle(port_rect.center(), 5.0 * zoom, port_color, Stroke::NONE);
    }
}

pub fn draw_hollow_circle_port(
    ui: &mut Ui,
    wide_port: bool,
    port_rect: Rect,
    zoom: f32,
    port_color: Color32,
) {
    if wide_port {
        ui.painter().rect(
            port_rect.shrink(2.0 * zoom),
            port_rect.width(),
            Color32::BLACK,
            Stroke::new(4.0 * zoom, port_color),
        );
    } else {
        ui.painter().circle(
            port_rect.center(),
            3.0 * zoom,
            Color32::BLACK,
            Stroke::new(4.0 * zoom, port_color),
        );
    }
}

pub fn draw_diamond_port(
    ui: &mut Ui,
    wide_port: bool,
    port_rect: Rect,
    zoom: f32,
    port_color: Color32,
) {
    if wide_port {
        let center = port_rect.center();
        ui.painter().add(egui::Mesh {
            indices: vec![0, 1, 2, 2, 1, 3, 2, 3, 4, 4, 3, 5],
            vertices: vec![
                Vertex {
                    pos: Pos2::new(center.x, port_rect.top()),
                    uv: WHITE_UV,
                    color: port_color,
                },
                Vertex {
                    pos: Pos2::new(port_rect.left(), port_rect.top() + 5.0 * zoom),
                    uv: WHITE_UV,
                    color: port_color,
                },
                Vertex {
                    pos: Pos2::new(port_rect.right(), port_rect.top() + 5.0 * zoom),
                    uv: WHITE_UV,
                    color: port_color,
                },
                Vertex {
                    pos: Pos2::new(port_rect.left(), port_rect.bottom() - 5.0 * zoom),
                    uv: WHITE_UV,
                    color: port_color,
                },
                Vertex {
                    pos: Pos2::new(port_rect.right(), port_rect.bottom() - 5.0 * zoom),
                    uv: WHITE_UV,
                    color: port_color,
                },
                Vertex {
                    pos: Pos2::new(center.x, port_rect.bottom()),
                    uv: WHITE_UV,
                    color: port_color,
                },
            ],
            texture_id: Default::default(),
        });
    } else {
        let port_rect = port_rect.expand(0.5 * zoom);
        let center = port_rect.center();
        ui.painter().add(egui::Mesh {
            indices: vec![0, 1, 2, 2, 1, 3],
            vertices: vec![
                Vertex {
                    pos: Pos2::new(center.x, port_rect.top()),
                    uv: WHITE_UV,
                    color: port_color,
                },
                Vertex {
                    pos: Pos2::new(port_rect.left(), center.y),
                    uv: WHITE_UV,
                    color: port_color,
                },
                Vertex {
                    pos: Pos2::new(port_rect.right(), center.y),
                    uv: WHITE_UV,
                    color: port_color,
                },
                Vertex {
                    pos: Pos2::new(center.x, port_rect.bottom()),
                    uv: WHITE_UV,
                    color: port_color,
                },
            ],
            texture_id: Default::default(),
        });
    }
}

pub fn draw_rect_port(
    ui: &mut Ui,
    _wide_port: bool,
    port_rect: Rect,
    _zoom: f32,
    port_color: Color32,
) {
    ui.painter().rect_filled(port_rect, 0.0, port_color);
}
