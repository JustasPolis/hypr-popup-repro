use gtk::{gdk, glib, prelude::*};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.basic")
        .build();

    application.connect_startup(|app| {
        let provider = gtk::CssProvider::new();
        provider.load_from_string(include_str!("style.css"));
        gtk::style_context_add_provider_for_display(
            &gdk::Display::default().expect("Could not connect to a display."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    });

    application.connect_activate(build_ui);
    application.run()
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_css_classes(&["main-window"]);
    window.set_default_height(36);

    window.set_title(Some("First GTK Program"));
    window.init_layer_shell();
    window.set_layer(Layer::Top);
    window.auto_exclusive_zone_enable();
    window.set_margin(Edge::Left, 20);
    window.set_margin(Edge::Right, 20);
    window.set_margin(Edge::Top, 20);
    window.set_namespace("testing");
    window.set_keyboard_mode(KeyboardMode::OnDemand);

    let anchors = [(Edge::Left, true), (Edge::Right, true), (Edge::Top, true)];

    for (anchor, state) in anchors {
        window.set_anchor(anchor, state);
    }

    let button = gtk::Button::with_label("Click me!");

    let popover = gtk::Popover::builder().css_classes(["popover"]).build();
    popover.set_position(gtk::PositionType::Bottom);

    popover.set_parent(&button);

    let item1 = gtk::Button::with_label("item 1");
    let item2 = gtk::Button::with_label("item 2");
    let item3 = gtk::Button::with_label("item 3");

    item1.connect_clicked(move |_button| {
        println!("1 clicked");
    });
    item2.connect_clicked(move |_button| {
        println!("2 clicked");
    });
    item3.connect_clicked(move |_button| {
        println!("3 clicked");
    });

    let menu_items = gtk::Box::default();
    menu_items.append(&item1);
    menu_items.append(&item2);
    menu_items.append(&item3);

    popover.set_child(Some(&menu_items));
    button.connect_clicked(move |_button| {
        popover.popup();
    });

    window.set_child(Some(&button));

    window.present();
}
