use gl;
use core_foundation::bundle::{CFBundleGetBundleWithIdentifier, CFBundleGetFunctionPointerForName};
use core_foundation::string::CFString;
use core_foundation::base::TCFType;
use cocoa::base::{selector, nil, YES, NO};
use cocoa::foundation::{NSRect, NSPoint, NSSize, NSAutoreleasePool, NSProcessInfo,
                        NSString, NSDate, NSDefaultRunLoopMode};
use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicyRegular, NSWindow,
                    NSBackingStoreBuffered, NSMenu, NSMenuItem, NSWindowStyleMask,
                    NSView, NSOpenGLPixelFormat, NSOpenGLContext,
                    NSEvent, NSEventMask, NSEventModifierFlags, NSEventType};

macro_rules! s {
    ($s:expr) => { NSString::alloc(nil).init_str($s) }
}

fn main() {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);
        let app = NSApp();

        let bundle_name = NSProcessInfo::processInfo(nil).processName();

        // Create the Menu Bar
        let menu_bar = NSMenu::new(nil).autorelease();
        app.setMainMenu_(menu_bar);

        // create Application menu
        let app_menu = NSMenu::new(nil).autorelease();
        let services_menu = NSMenu::alloc(nil).autorelease();
        app_menu.addItemWithTitle_action_keyEquivalent(s!("Services"), selector("hide:"), s!("h")).autorelease().setSubmenu_(services_menu);
        app_menu.addItem_(NSMenuItem::separatorItem(nil).autorelease());
        app_menu.addItemWithTitle_action_keyEquivalent(s!("Hide"), selector("hide:"), s!("h")).autorelease();
        app_menu.addItemWithTitle_action_keyEquivalent(s!("Hide Others"), selector("hideOtherApplications:"), s!("h")).autorelease(); // setKeyEquivalentModifierMask:NSEventModifierFlagOption | NSEventModifierFlagCommand]
        app_menu.addItemWithTitle_action_keyEquivalent(s!("Show all"), selector("unhideAllApplications:"), s!("")).autorelease();
        app_menu.addItem_(NSMenuItem::separatorItem(nil).autorelease());
        app_menu.addItemWithTitle_action_keyEquivalent(s!("Quit"), selector("terminate:"), s!("q")).autorelease();
        app.setServicesMenu_(services_menu);
        let app_menu_item = NSMenuItem::new(nil).autorelease();
        menu_bar.addItem_(app_menu_item);
        app_menu_item.setSubmenu_(app_menu);

        // Create the Window Menu
        let window_menu = NSMenu::alloc(nil).initWithTitle_(s!("Window")).autorelease();
        window_menu.addItemWithTitle_action_keyEquivalent(s!("Minimize"), selector("performMiniaturize:"), s!("m")).autorelease();
        window_menu.addItemWithTitle_action_keyEquivalent(s!("Zoom"), selector("performZoom:"), s!("n")).autorelease();
        window_menu.addItemWithTitle_action_keyEquivalent(s!("Full Screen"), selector("toggleFullScreen:"), s!("f")).autorelease(); // setKeyEquivalentModifierMask:NSEventModifierFlagControl | NSEventModifierFlagCommand
        window_menu.addItemWithTitle_action_keyEquivalent(s!("Close Window"), selector("performClose:"), s!("w")).autorelease();
        window_menu.addItem_(NSMenuItem::separatorItem(nil).autorelease());
        window_menu.addItemWithTitle_action_keyEquivalent(s!("Bring All to Front"), selector("arrangeInFront:"), s!("")).autorelease();
        app.setWindowsMenu_(window_menu);
        let window_menu_item = NSMenuItem::new(nil).autorelease();
        menu_bar.addItem_(window_menu_item);
        window_menu_item.setSubmenu_(window_menu);

        // Create the Help Menu
        let help_menu = NSMenu::alloc(nil).initWithTitle_(s!("Help")).autorelease();
        help_menu.addItemWithTitle_action_keyEquivalent(s!("Documentation"), selector("docs:"), s!("")).autorelease();
        let help_menu_item = NSMenuItem::new(nil).autorelease();
        menu_bar.addItem_(help_menu_item);
        help_menu_item.setSubmenu_(help_menu);

        // create Window
        let rect = NSRect::new(NSPoint::new(0., 0.), NSSize::new(640., 480.));
        let window = NSWindow::alloc(nil)
            .initWithContentRect_styleMask_backing_defer_(rect,
                                                          NSWindowStyleMask::NSTitledWindowMask |
                                                          NSWindowStyleMask::NSResizableWindowMask |
                                                          NSWindowStyleMask::NSClosableWindowMask |
                                                          NSWindowStyleMask::NSMiniaturizableWindowMask,
                                                          NSBackingStoreBuffered,
                                                          NO)
            .autorelease();
        window.setTitle_(bundle_name);
        window.cascadeTopLeftFromPoint_(NSPoint::new(20., 20.));
        window.setMinSize_(NSSize::new(300., 200.));
        window.setAcceptsMouseMovedEvents_(YES);
        window.makeKeyAndOrderFront_(nil);
        window.center();

        // Create the View
        let view = NSView::alloc(nil).autorelease();
        window.makeFirstResponder_(view);
        window.setContentView_(view);

        // Create the Context
        let format = NSOpenGLPixelFormat::alloc(nil).initWithAttributes_(&[99, 0x4100, 0]).autorelease();
        let context = NSOpenGLContext::alloc(nil).initWithFormat_shareContext_(format, nil).autorelease();
        context.setView_(view);
        context.makeCurrentContext();

        // Setup the GL Library
        gl::load_with(|addr|
            CFBundleGetFunctionPointerForName(
                CFBundleGetBundleWithIdentifier(CFString::new("com.apple.opengl").as_concrete_TypeRef()),
                CFString::new(addr).as_concrete_TypeRef(),
            )
        );

        // Setup Observers
        let running = true;
        // NSNotificationCenter::defaultCenter().addObserverForName_(NSWindowWillCloseNotification
        //             object:window
        //              queue:nil
        //         usingBlock:^(NSNotification *notification) {
        //           running = 0;
        //         }];

        // Finish loading
        app.setActivationPolicy_(NSApplicationActivationPolicyRegular);
        app.activateIgnoringOtherApps_(YES);
        app.finishLaunching();

        while running {
            let event = app.nextEventMatchingMask_untilDate_inMode_dequeue_(
                NSEventMask::NSAnyEventMask.bits(),
                NSDate::distantFuture(nil),
                NSDefaultRunLoopMode,
                YES);
                if event.eventType() != NSEventType::NSKeyDown ||
                    event.modifierFlags().contains(NSEventModifierFlags::NSCommandKeyMask) {
                    app.sendEvent_(event);
                }
            
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Flush();

            context.flushBuffer();
        }

        // app.terminate_(nil);
    }
}
