use adw::{prelude::*,glib,Application,ApplicationWindow};
use gtk::{gio, Button, Orientation};
mod custom_button;

use gtk::Box;

use custom_button::CustomButton;

const APP_ID:&str = "com.sanjai.org";

fn main() -> glib::ExitCode{
  gio::resources_register_include!("sanjai.gresource")
          .expect("Failed to register resources.");

  let app = Application::builder()
    .application_id(APP_ID)
    .build();

  app.connect_activate(build_ui);

  app.run()
}

fn build_ui(app: &adw::Application) {

  let button = Button::builder()
    .label("click me")
    .margin_top(12)
    .margin_bottom(12)
    .margin_start(12)
    .margin_end(12)
    .build();

  button.connect_clicked(|button|{
    button.set_label("you clicked me ");
  });

  let button2 =  CustomButton::with_label("Press me!");
     button.set_margin_top(12);
     button.set_margin_bottom(12);
     button.set_margin_start(12);
     button.set_margin_end(12);




  let gtk_box = gtk::Box::builder()
    .orientation(Orientation::Vertical)

    .spacing(10)
    // .margin_top(12)
    // .margin_end(12)
    // .margin_start(12)
    // .margin_end(12)
    .build();

  gtk_box.append(&adw::HeaderBar::new());
  gtk_box.append(&button);
  gtk_box.append(&button2);
  //
  // let content = Box::new(Orientation::Vertical,0);
  //   content.append(&adw::HeaderBar::new());
  //   content.append(&button2);
  //   content.append(&button);


  let window = ApplicationWindow::builder()
    .application(app)
    .content(&gtk_box)
    .title("sanjai")
    .build();

  window.present();

}