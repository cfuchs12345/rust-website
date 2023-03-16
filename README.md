# rust-website

My website build with Rust, Actix-web, Tera and SeaORM.

It uses a workaround to enable Rust to switch language.
Tera doesn't seem to support i18n out-of-the-box, so I found a way by using a thread-local value which holds the language to use and which can be accessed by a custom function for Tera.
Seems to work quite well.
But if someone with more Rust experience knows a better way, or thinks that there is a problem with this approach (i.e. with threadlocal), please write me.
I would be really interested!