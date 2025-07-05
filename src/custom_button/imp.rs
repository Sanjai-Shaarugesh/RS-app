use std::cell::Cell;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

#[derive(Default)]
pub struct CustomButton{
    number: Cell<i32>,
}

#[glib::object_subclass]
impl ObjectSubclass for CustomButton {
    const NAME:&'static str = "CustomButton";
    type Type = super::CustomButton;
    type ParentType = gtk::Button;
}

impl ObjectImpl for CustomButton{
  fn constructed(&self){
      self.parent_constructed();
      self.obj().set_label(&self.number.get().to_string());
  }
}

impl WidgetImpl for CustomButton{}
impl ButtonImpl for CustomButton{}
