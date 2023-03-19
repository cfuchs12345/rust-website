# rust-website

My website build with Rust, Actix-web, Tera and SeaORM.

It uses a workaround to enable Tera templates to switch languages.

Tera doesn't seem to support i18n out-of-the-box, so I had to to find a workaround.
I solved it by using a thread-local that holds the language, and which can be accessed by a custom function for Tera. Seems to work quite well.
But if someone with more Rust experience knows a better way (or thinks that there is a problem with this approach i.e. with threadlocal), feel free to open an issue here on Github.
I would be really interested if there is a better way.
