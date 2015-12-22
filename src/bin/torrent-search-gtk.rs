extern crate torrent_search;
extern crate gtk;

use torrent_search::SearchProvider;
use gtk::traits::window::WindowTrait;
use gtk::traits::widget::WidgetTrait;
use gtk::traits::container::ContainerTrait;
use gtk::signal::WidgetSignals;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");
    let window = gtk::Window::new(gtk::WindowType::Toplevel).unwrap();

    // configure window
    window.set_title("Torrent Search");
    window.set_default_size(640, 480);

    window.connect_delete_event(|_,_| {
        gtk::main_quit();
        gtk::signal::Inhibit(true)
    });


    // get torrents
    let providers = torrent_search::get_search_providers();
    let keyword = "archlinux";
    let mut torrents = vec![];
    for provider in providers.iter() {
        match provider.search(keyword) {
            Ok(results) => torrents.extend(results),
            Err(err) => println!("Error: {}", err),
        }
    }

    // add widgets
    let scrolled_window = gtk::ScrolledWindow::new(None, None).unwrap();
    scrolled_window.set_min_content_width(600);


    let container = gtk::Box::new(gtk::Orientation::Vertical, 0).unwrap();

    for torrent in torrents.iter() {
        let title_and_url = gtk::LinkButton::new_with_label(&torrent.magnet_link, &torrent.name).unwrap();
        title_and_url.set_halign(gtk::Align::Start);
        title_and_url.set_margin_start(0);
        container.add(&title_and_url);
    }
    scrolled_window.add(&container);
    window.add(&scrolled_window);

    window.show_all();

    gtk::main();
}
