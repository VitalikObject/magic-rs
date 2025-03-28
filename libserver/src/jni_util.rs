use jni::JNIEnv;

pub fn get_package_name(mut env: JNIEnv) -> String {
    let activity_thread = env.find_class("android/app/ActivityThread").unwrap();
    let current_application = env
        .call_static_method(
            &activity_thread,
            "currentApplication",
            "()Landroid/app/Application;",
            &[],
        )
        .unwrap()
        .l()
        .unwrap();

    let context = env
        .call_method(
            current_application,
            "getApplicationContext",
            "()Landroid/content/Context;",
            &[],
        )
        .unwrap()
        .l()
        .unwrap();

    let package_name = env
        .call_method(context, "getPackageName", "()Ljava/lang/String;", &[])
        .unwrap()
        .l()
        .unwrap();

    env.get_string(&package_name.into())
        .unwrap()
        .to_string_lossy()
        .to_string()
}
