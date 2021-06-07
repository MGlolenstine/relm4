use gtk::prelude::*;

pub trait ContainerExt {
    fn add<WIDGET: IsA<gtk::Widget>>(self, child: &WIDGET) -> Self;
    fn remove<WIDGET: IsA<gtk::Widget>>(self, child: &WIDGET) -> Self;
}

impl ContainerExt for gtk::Box {
    fn add<WIDGET: IsA<gtk::Widget>>(self, child: &WIDGET) -> Self {
        self.append(child);
        self
    }

    fn remove<WIDGET: IsA<gtk::Widget>>(self, child: &WIDGET) -> Self {
        BoxExt::remove(&self, child);
        self
    }
}
