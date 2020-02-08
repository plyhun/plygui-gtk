use crate::common::{self, *};

use gtk::prelude::*;
use gtk::{ButtonsType, DialogFlags, MessageDialog, MessageDialogExt, MessageType, Widget};

#[repr(C)]
pub struct GtkMessage {
    message: MessageDialog,
    actions: Vec<(String, callbacks::Action)>,
}

pub type Message = AMember<AMessage<GtkMessage>>;

impl MessageInner for GtkMessage {
    fn with_actions(content: types::TextContent, severity: types::MessageSeverity, actions: Vec<(String, callbacks::Action)>, parent: Option<&dyn controls::Member>) -> Box<Message> {
        let parent = parent.map(|parent| Object::from(common::cast_member_to_gtkwidget(parent)).downcast::<Widget>().unwrap().get_toplevel().unwrap().downcast::<gtk::Window>().unwrap());

        let mut message = Box::new(AMember::with_inner(
            AMessage::with_inner(
	            GtkMessage {
	                message: MessageDialog::new(
	                    parent.as_ref(),
	                    DialogFlags::MODAL | DialogFlags::DESTROY_WITH_PARENT,
	                    severity_to_message_type(severity),
	                    if actions.len() > 0 { ButtonsType::None } else { ButtonsType::Ok },
	                    match content {
	                        types::TextContent::Plain(ref text) => text.as_str(),
	                        types::TextContent::LabelDescription(ref label, _) => label.as_str(),
	                    },
	                ),
	                actions: actions,
	            }
            ),
        ));

        let ptr = message.as_ref() as *const _ as *mut std::os::raw::c_void;

        {
            let message = message.as_inner_mut();
            common::set_pointer(&mut message.message.clone().upcast(), ptr);
            message.message.connect_response(on_response);

            if let types::TextContent::LabelDescription(_, ref description) = content {
                message.message.set_property_secondary_text(Some(description.as_str()));
            }

            message.actions.iter().enumerate().for_each(|(i, (n, _))| {
                message.message.add_button(n, i as i32);
            });
        }
        message
    }
    fn severity(&self) -> types::MessageSeverity {
        message_type_to_severity(self.message.get_property_message_type())
    }
    fn start(self) -> Result<String, ()> {
        let pressed = self.message.run() as usize;
        self.message.close();
        self.actions.get(pressed).map(|(n, _)| n.clone()).ok_or(())
    }
}

impl HasLabelInner for GtkMessage {
    fn label(&self, _: &MemberBase) -> Cow<str> {
        Cow::Owned(self.message.get_title().unwrap_or(String::new()))
    }
    fn set_label(&mut self, _: &mut MemberBase, label: Cow<str>) {
        self.message.set_title(&label);
    }
}

impl HasNativeIdInner for GtkMessage {
    type Id = common::GtkWidget;

    unsafe fn native_id(&self) -> Self::Id {
        self.message.clone().upcast::<Object>().into()
    }
}

impl MemberInner for GtkMessage {}

fn severity_to_message_type(severity: types::MessageSeverity) -> MessageType {
    match severity {
        types::MessageSeverity::Info => MessageType::Info,
        types::MessageSeverity::Warning => MessageType::Warning,
        types::MessageSeverity::Alert => MessageType::Error,
    }
}
fn message_type_to_severity(ty: MessageType) -> types::MessageSeverity {
    match ty {
        MessageType::Info => types::MessageSeverity::Info,
        MessageType::Warning => types::MessageSeverity::Warning,
        MessageType::Error => types::MessageSeverity::Alert,
        _ => unreachable!(),
    }
}

fn on_response(this: &MessageDialog, r: i32) {
    let mut message = this.clone().upcast::<Widget>();
    let message = common::cast_gtk_widget_to_member_mut::<Message>(&mut message);
    if let Some(message) = message {
        let mut message2 = message.as_inner_mut().message.clone().upcast::<Widget>();
        let message2 = common::cast_gtk_widget_to_member_mut::<Message>(&mut message2);
        if let Some(action) = message.as_inner_mut().actions.get_mut(r as usize) {
            if let Some(message2) = message2 {
                (action.1.as_mut())(message2);
            }
        }
    }
}
