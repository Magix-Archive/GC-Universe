use jni::JNIEnv;
use jni::objects::{JMethodID, JObject, JValue};
use jni::signature::{Primitive, ReturnType};

pub struct Logger<'local> {
    env: JNIEnv<'local>,
    logger: JObject<'local>,
    info_method: JMethodID,
    error_method: JMethodID,
    warn_method: JMethodID,
    debug_method: JMethodID,
    trace_method: JMethodID
}

impl Logger<'_> {
    pub fn new(mut env: JNIEnv) -> Logger {
        // Get the Java logger.
        let class = env.find_class("emu/grasscutter/Grasscutter")
            .expect("failed to find main class");
        let logger = env.get_static_field(&class, "logger", "Lch/qos/logback/classic/Logger;")
            .expect("failed to get logger object")
            .l()
            .unwrap();
        let logger_class = env.get_object_class(&logger)
            .expect("failed to get logger class");

        // Get the logger methods.
        let info_method = env.get_method_id(&logger_class, "info", "(Ljava/lang/String;)V")
            .expect("failed to get info method");
        let error_method = env.get_method_id(&logger_class, "error", "(Ljava/lang/String;)V")
            .expect("failed to get error method");
        let warn_method = env.get_method_id(&logger_class, "warn", "(Ljava/lang/String;)V")
            .expect("failed to get warn method");
        let debug_method = env.get_method_id(&logger_class, "debug", "(Ljava/lang/String;)V")
            .expect("failed to get debug method");
        let trace_method = env.get_method_id(&logger_class, "trace", "(Ljava/lang/String;)V")
            .expect("failed to get trace method");

        Logger {
            env, logger, info_method, error_method, warn_method, debug_method, trace_method
        }
    }

    /// Generic log method.
    /// level: The method ID of the log level.
    /// msg: The message to log.
    fn log<S: Into<String>>(&mut self, level: JMethodID, msg: S) {
        // Try attaching to the JVM.
        let jvm = self.env.get_java_vm()
            .expect("failed to get Java VM");
        let _ = jvm.attach_current_thread()
            .expect("failed to attach to JVM");

        let msg = self.env.new_string(msg.into())
            .expect("failed to create Java string");
        let raw_msg: JValue = (&msg).into();

        unsafe {
            self.env.call_method_unchecked(
                &self.logger, level,
                ReturnType::Primitive(Primitive::Void),
                &[raw_msg.as_jni()]
            ).expect("failed to call log method");
        }
    }

    pub fn info<S: Into<String>>(&mut self, msg: S) {
        self.log(self.info_method, msg);
    }

    pub fn error<S: Into<String>>(&mut self, msg: S) {
        self.log(self.error_method, msg);
    }

    pub fn warn<S: Into<String>>(&mut self, msg: S) {
        self.log(self.warn_method, msg);
    }

    pub fn debug<S: Into<String>>(&mut self, msg: S) {
        self.log(self.debug_method, msg);
    }

    pub fn trace<S: Into<String>>(&mut self, msg: S) {
        self.log(self.trace_method, msg);
    }
}
