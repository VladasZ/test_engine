use crate::gm::{Size, Rect, Color, Point};
use crate::te::{TEUIDrawer, Assets};
use crate::ui::View;
use crate::gl_wrapper::gl_drawer::{Updatable, MouseButton, ButtonState};
use crate::ui::input::Touch;
use crate::ui::input::touch::Event;
use crate::utils::{Shared};
use crate::ui::view::WeakView;

pub struct Screen {
    cursor_position: Point,
    root_view: Shared<View>,
    touch_views: Vec<WeakView>,
    ui_drawer: TEUIDrawer
}

impl Screen {

    fn on_touch(&mut self, mut touch: Touch) {
        self.root_view.try_borrow_mut().unwrap().check_touch(&mut touch)
    }
}

impl Updatable for Screen {

    fn new() -> Screen {
        let assets = Assets::init();
        let ui_drawer = TEUIDrawer::new(assets);
        Screen {
            cursor_position: Point::new(),
            root_view: View::new(),
            touch_views: vec![],
            ui_drawer
        }
    }

    fn init(&mut self) {
        self.root_view.borrow_mut().make_subview(|view|{

            view.set_frame(Rect::make(200.0, 200.0, 300.0, 300.0));
            view.color = Color::BLUE;

            view.make_subview(|view|{
                view.set_frame(Rect::make(20.0, 20.0, 100.0, 100.0));
                view.color = Color::GREEN;

                view.make_subview(|view| {
                    view.set_frame(Rect::make(10.0, 10.0, 20.0, 20.0));
                    view.touch_enabled = true;
                    view.color = Color::TURQUOISE;
                });

            });

        });
    }

    fn set_size(&mut self, size: Size) {
        self.ui_drawer.set_size(&size);
        self.root_view.borrow_mut().set_frame(Rect::from_size(&size));
    }

    fn on_cursor_moved(&mut self, position: Point) {
        self.cursor_position = position
    }

    fn on_mouse_key_pressed(&mut self, button: MouseButton, state: ButtonState) {
        self.on_touch(Touch {
            id: 1,
            position: self.cursor_position,
            event: Event::from_state(state)
        })
    }

    fn update(&mut self) {
        self.ui_drawer.draw_view(self.root_view.clone());

        let font = &self.ui_drawer.assets.fonts.default;
        let image = &font.glyph_for_char('b').image;
        let rect = Rect::make(10.0, 10.0, 300.0, 300.0);
        let color = Color::WHITE;

        self.ui_drawer.draw_image_in_rect(image, &rect, &color);
    }
}
