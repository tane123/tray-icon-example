// use cocoa::appkit::{
//     NSApp, NSApplication, NSApplicationActivationPolicyRegular, NSButton, NSMenu, NSMenuItem,
//     NSStatusBar, NSStatusItem,
// };
// use cocoa::base::nil;
// use cocoa::foundation::{NSAutoreleasePool, NSString};
// use objc::{sel, sel_impl};

// fn main() {
//     unsafe {
//         let app = NSApp();
//         let status_bar = NSStatusBar::systemStatusBar(nil);
//         let status_item = status_bar.statusItemWithLength_(-1.0);

//         let menu = NSMenu::new(nil);
//         let quit_item = NSMenuItem::new(nil).autorelease();
//         quit_item.setTitle_(NSString::alloc(nil).init_str("Quit"));
//         quit_item.setAction_(sel!(terminate:));
//         menu.addItem_(quit_item);

//         status_item.setMenu_(menu);
//         status_item.setTitle_(NSString::alloc(nil).init_str("Tray Example"));

//         app.run()
//     }
// }

use cocoa::appkit::{
    NSApp, NSApplication, NSButton, NSMenu, NSMenuItem, NSStatusBar, NSStatusItem,
};
use cocoa::base::nil;
use cocoa::foundation::{NSAutoreleasePool, NSString};
use objc::{msg_send, sel, sel_impl};

fn main() {
    unsafe {
        let app = NSApp();

        let status_bar = NSStatusBar::systemStatusBar(nil);
        let status_item = status_bar.statusItemWithLength_(-1.0);
        let status_button = status_item.button();

        status_button.setTitle_(NSString::alloc(nil).init_str("Tray example"));

        let menu = NSMenu::new(nil);
        let quit_item = NSMenuItem::new(nil).autorelease();
        quit_item.setTitle_(NSString::alloc(nil).init_str("Quit"));
        quit_item.setAction_(sel!(terminate:));
        menu.addItem_(quit_item);

        status_item.setMenu_(menu);

        app.run()
    }
}
