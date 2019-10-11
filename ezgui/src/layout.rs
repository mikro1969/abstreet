use crate::{Canvas, ScreenDims, ScreenPt};
use ordered_float::NotNan;

// TODO Move this to widgets/mod

pub trait Widget {
    fn get_dims(&self) -> ScreenDims;
    fn set_pos(&mut self, top_left: ScreenPt, total_width: f64);
}

pub enum ContainerOrientation {
    TopLeft,
    TopRight,
    Centered,
}

pub fn stack_vertically(
    orientation: ContainerOrientation,
    canvas: &Canvas,
    widgets: Vec<&mut dyn Widget>,
) {
    assert!(!widgets.is_empty());

    let dims_per_widget: Vec<ScreenDims> = widgets.iter().map(|w| w.get_dims()).collect();
    let total_width = dims_per_widget
        .iter()
        .map(|d| d.width)
        .max_by_key(|x| NotNan::new(*x).unwrap())
        .unwrap();
    let total_height: f64 = dims_per_widget.iter().map(|d| d.height).sum();

    let mut top_left = match orientation {
        ContainerOrientation::TopLeft => ScreenPt::new(0.0, 0.0),
        ContainerOrientation::TopRight => ScreenPt::new(canvas.window_width - total_width, 0.0),
        ContainerOrientation::Centered => {
            let mut pt = canvas.center_to_screen_pt();
            pt.x -= total_width / 2.0;
            pt.y -= total_height / 2.0;
            pt
        }
    };
    for (w, dims) in widgets.into_iter().zip(dims_per_widget) {
        w.set_pos(top_left, total_width);
        top_left.y += dims.height;
    }
}
