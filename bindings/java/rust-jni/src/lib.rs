use jni::JNIEnv;
use jni::objects::{JClass, JString};
use cross_logger_core::{print_log, LogLevel};

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_crosslogger_CrossLogger_logInfo(
    mut env: JNIEnv,
    _class: JClass,
    message: JString,
) {
    let msg: String = env.get_string(&message).unwrap().into();
    print_log(LogLevel::Info, &msg);
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_crosslogger_CrossLogger_logWarn(
    mut env: JNIEnv,
    _class: JClass,
    message: JString,
) {
    let msg: String = env.get_string(&message).unwrap().into();
    print_log(LogLevel::Warn, &msg);
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_crosslogger_CrossLogger_logError(
    mut env: JNIEnv,
    _class: JClass,
    message: JString,
) {
    let msg: String = env.get_string(&message).unwrap().into();
    print_log(LogLevel::Error, &msg);
}
