use jni::JNIEnv;
use jni::objects::{JClass, JObject, JObjectArray, JString};
use jni::sys::{jboolean, jint, jlong};
use cross_logger_core::{LoggerConfig as CoreConfig, LogLevel as CoreLevel};

fn to_level(value: jint) -> CoreLevel {
    match value {
        -1 => CoreLevel::Off,
        0  => CoreLevel::Silly,
        1  => CoreLevel::Debug,
        3  => CoreLevel::Warn,
        4  => CoreLevel::Error,
        5  => CoreLevel::Fatal,
        _  => CoreLevel::Info,
    }
}

/// Creates a CoreConfig on the heap and returns its pointer as a jlong handle.
/// Called from LoggerConfig constructor in Java.
#[unsafe(no_mangle)]
pub extern "system" fn Java_com_crosslogger_LoggerConfig_nativeCreate(
    mut env: JNIEnv,
    _class: JClass,
    name: JString,
    min_level: jint,
    is_cloud: jboolean,
    obfuscate_keys: JObjectArray,
    obfuscate_depth: jint,
) -> jlong {
    let name: String = env.get_string(&name).unwrap().into();
    let mut config = CoreConfig::new(name, to_level(min_level)).with_cloud(is_cloud != 0);

    let keys_obj = JObject::from(obfuscate_keys);
    if !keys_obj.is_null() {
        let arr = JObjectArray::from(keys_obj);
        let len = env.get_array_length(&arr).unwrap_or(0);
        let keys: Vec<String> = (0..len)
            .map(|i| {
                let obj = env.get_object_array_element(&arr, i).unwrap();
                let jstr = JString::from(obj);
                env.get_string(&jstr).unwrap().into()
            })
            .collect();
        let key_refs: Vec<&str> = keys.iter().map(|s| s.as_str()).collect();
        config = config.with_obfuscation(key_refs, obfuscate_depth as usize);
    }

    Box::into_raw(Box::new(config)) as jlong
}

/// Emits a log entry through the native LoggerConfig handle.
#[unsafe(no_mangle)]
pub extern "system" fn Java_com_crosslogger_LoggerConfig_nativeLog(
    mut env: JNIEnv,
    _object: JObject,
    handle: jlong,
    level: jint,
    message: JString,
) {
    let config = unsafe { &*(handle as *const CoreConfig) };
    let msg: String = env.get_string(&message).unwrap().into();
    config.log(to_level(level), &msg);
}

/// Drops the CoreConfig behind the handle. Called from LoggerConfig.close().
#[unsafe(no_mangle)]
pub extern "system" fn Java_com_crosslogger_LoggerConfig_nativeDestroy(
    _env: JNIEnv,
    _object: JObject,
    handle: jlong,
) {
    unsafe { drop(Box::from_raw(handle as *mut CoreConfig)) };
}
