use crate::view::View;
use gtk4::{prelude::*, Application};
use std::{cell::RefCell, rc::Rc};

#[derive(Clone)]
pub struct Controller {
    app: Application,
    _view_container: Rc<RefCell<Option<View>>>,
}

impl Controller {
    pub fn start_ui(&self) {
        self.app.run();
    }

    pub fn new() -> Controller {
        let app = Application::builder()
            .application_id("me.zwerdlds.glyphmosaic")
            .build();

        let view_container = Rc::new(RefCell::new(None));

        app.clone().connect_activate({
            let view_container = view_container.clone();
            move |_| {
                let view = View::new();

                view.main_window().show();

                (*view_container).replace(Some(view));
            }
        });

        Controller {
            app,
            _view_container: view_container,
        }
    }
}
