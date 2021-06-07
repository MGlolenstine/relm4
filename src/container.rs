use glib::IsA;

/// Container trait, that adds generic `add` and `remove` functions.
/// Every container has their own functions and this trait generalizes them.
pub trait ContainerExt {
    /// Add widget to the Container
    fn add<WIDGET: IsA<gtk::Widget>>(self, child: &WIDGET) -> Self;
    /// Remove specified widget from the Container
    fn remove<WIDGET: IsA<gtk::Widget>>(self, child: &WIDGET) -> Self;
}

impl ContainerExt for gtk::Box {
    fn add<WIDGET: IsA<gtk::Widget>>(self, child: &WIDGET) -> Self {
        gtk::prelude::BoxExt::append(&self, child);
        self
    }

    fn remove<WIDGET: IsA<gtk::Widget>>(self, child: &WIDGET) -> Self {
        gtk::prelude::BoxExt::remove(&self, child);
        self
    }
}

impl ContainerExt for gtk::FlowBox {
    fn add<WIDGET: IsA<gtk::Widget>>(self, child: &WIDGET) -> Self {
        gtk::FlowBox::insert(&self, child, -1);
        self
    }

    fn remove<WIDGET: IsA<gtk::Widget>>(self, child: &WIDGET) -> Self {
        gtk::FlowBox::remove(&self, child);
        self
    }
}
