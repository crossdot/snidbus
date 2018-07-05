extern crate dbus;

use std::sync::Arc;

use dbus::{Connection, BusType, NameFlag};
use dbus::tree::{Access, Factory};

fn main() {
    // Let's start by starting up a connection to the session bus and register a name.
    let c = Connection::get_private(BusType::Session).unwrap();
    c.register_name("org.kde.StatusNotifierWatcher", NameFlag::ReplaceExisting as u32).unwrap();

    // The choice of factory tells us what type of tree we want,
    // and if we want any extra data inside. We pick the simplest variant.
    let f = Factory::new_fn::<()>();

    // We create the signal first, since we'll need it in both inside the method callback
    // and when creating the tree.
    let signal_status_notifier_item_registered = Arc::new(f.signal("StatusNotifierItemRegistered", ()).sarg::<&str,_>("service"));
    let signal_status_notifier_item_registered2 = signal_status_notifier_item_registered.clone();

    let signal_status_notifier_item_unregistered = Arc::new(f.signal("StatusNotifierItemUnregistered", ()).sarg::<&str,_>("service"));
    let signal_status_notifier_item_unregistered2 = signal_status_notifier_item_unregistered.clone();

    let signal_status_notifier_host_registered = Arc::new(f.signal("StatusNotifierHostRegistered", ()).sarg::<&str,_>("sender"));
    let signal_status_notifier_host_registered2 = signal_status_notifier_host_registered.clone();

    // We create a tree with one object path inside and make that path introspectable.
    let tree = f.tree(()).add(f.object_path("/StatusNotifierWatcher", ()).introspectable().add(

        // We add an interface to the object path...
        f.interface("org.kde.StatusNotifierWatcher", ())
            .add_m(
                // ...and a method inside the interface.
                f   .method("RegisterStatusNotifierItem", (), move |m| {

                    // This is the callback that will be called when another peer on the bus calls our method.
                    // the callback receives "MethodInfo" struct and can return either an error, or a list of
                    // messages to send back.

                    let service: &str = m.msg.read1()?;
                    println!("Hello {}!", service);

                    // Two messages will be returned - one is the method return (and should always be there),
                    // and in our case we also have a signal we want to send at the same time.
                    Ok(vec![])

                // Our method has one output argument and one input argument.
                })
                .inarg::<&str,_>("service")
            )
            .add_m(
                // ...and a method inside the interface.
                f   .method("RegisterStatusNotifierHost", (), move |m| {

                    // This is the callback that will be called when another peer on the bus calls our method.
                    // the callback receives "MethodInfo" struct and can return either an error, or a list of
                    // messages to send back.

                    let service: &str = m.msg.read1()?;
                    let s = format!("Hello {}!", service);
                    println!("{}", s);
                    let mret = m.msg.method_return().append1(s);

                    let sig = signal_status_notifier_host_registered
                        .msg(m.path.get_name(), m.iface.get_name())
                        .append1(&*service);

                    // Two messages will be returned - one is the method return (and should always be there),
                    // and in our case we also have a signal we want to send at the same time.
                    Ok(vec!(mret, sig))

                // Our method has one output argument and one input argument.
                })
                .inarg::<&str,_>("service")
                .outarg::<&str,_>("reply")
            )
            .add_p(
                f   .property::<bool,_>("RegisteredStatusNotifierItems", ())
                    .access(Access::ReadWrite)
                    .on_get(|_i, _m| {
                        Ok(())
                    })
                    .on_set(|_i, _m| {
                        Ok(())
                    })
            )
            .add_p(
                f   .property::<bool,_>("IsStatusNotifierHostRegistered", ())
                    .access(Access::ReadWrite)
                    .on_get(|_i, _m| {
                        Ok(())
                    })
                    .on_set(|_i, _m| {
                        Ok(())
                    })
            )
            .add_p(
                f   .property::<i32,_>("ProtocolVersion", ())
                    .access(Access::ReadWrite)
                    .on_get(|_i, _m| {
                        Ok(())
                    })
                    .on_set(|_i, _m| {
                        Ok(())
                    })
            )
             // We also add the signal to the interface. This is mainly for introspection.
            .add_s(signal_status_notifier_item_registered2)
            .add_s(signal_status_notifier_item_unregistered2)
            .add_s(signal_status_notifier_host_registered2)
    ));

    // We register all object paths in the tree.
    tree.set_registered(&c, true).unwrap();

    // We add the tree to the connection so that incoming method calls will be handled
    // automatically during calls to "incoming".
    c.add_handler(tree);

    // Serve other peers forever.
    loop { c.incoming(1000).next(); }
}