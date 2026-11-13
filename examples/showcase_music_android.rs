#[path = "showcase_music.rs"]
mod app;

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(android_app: gpui_android::AndroidApp) {
    gpui_android::init_android_app(android_app);
    app::main();
}
