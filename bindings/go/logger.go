package crosslogger

/*
#cgo pkg-config: cross_logger
#include <stdlib.h>

void* logger_config_create(const char* name, int min_level, int is_cloud,
    const char** obfuscate_keys, size_t obfuscate_keys_count, size_t obfuscate_depth);
void  logger_config_log(void* handle, int level, const char* message);
void  logger_config_destroy(void* handle);
*/
import "C"
import (
	"runtime"
	"unsafe"
)

// Log level constants. Pass these to LoggerConfig.Log().
const (
	LogLevelOff   = -1
	LogLevelSilly = 0
	LogLevelDebug = 1
	LogLevelInfo  = 2
	LogLevelWarn  = 3
	LogLevelError = 4
	LogLevelFatal = 5
)

// LoggerConfig is a named logger instance backed by a native Rust LoggerConfig.
// Call Close() or use it in a defer to release native memory.
type LoggerConfig struct {
	handle unsafe.Pointer
}

type loggerOptions struct {
	isCloud        bool
	obfuscateKeys  []string
	obfuscateDepth int
}

// Option is a functional option for NewLoggerConfig.
type Option func(*loggerOptions)

// WithCloud enables cloud-compatible output format.
func WithCloud(v bool) Option {
	return func(o *loggerOptions) { o.isCloud = v }
}

// WithObfuscation redacts matching keys up to the given nesting depth in JSON messages.
func WithObfuscation(keys []string, depth int) Option {
	return func(o *loggerOptions) {
		o.obfuscateKeys = keys
		o.obfuscateDepth = depth
	}
}

// NewLoggerConfig creates a logger with the given name and minimum level.
// Optional settings are passed as Option values.
func NewLoggerConfig(name string, minLevel int, opts ...Option) *LoggerConfig {
	o := &loggerOptions{}
	for _, opt := range opts {
		opt(o)
	}

	cName := C.CString(name)
	defer C.free(unsafe.Pointer(cName))

	isCloud := 0
	if o.isCloud {
		isCloud = 1
	}

	keysArr, freeKeys := toCStringArray(o.obfuscateKeys)
	defer freeKeys()

	handle := C.logger_config_create(
		cName,
		C.int(minLevel),
		C.int(isCloud),
		keysArr,
		C.size_t(len(o.obfuscateKeys)),
		C.size_t(o.obfuscateDepth),
	)

	l := &LoggerConfig{handle: unsafe.Pointer(handle)}
	runtime.SetFinalizer(l, (*LoggerConfig).Close)
	return l
}

// Log emits a log entry if level >= minLevel.
// If message is a JSON string it is embedded as a nested object in the output.
func (l *LoggerConfig) Log(level int, message string) {
	cMsg := C.CString(message)
	defer C.free(unsafe.Pointer(cMsg))
	C.logger_config_log(l.handle, C.int(level), cMsg)
}

// Close releases the native Rust LoggerConfig. Safe to call multiple times.
func (l *LoggerConfig) Close() {
	if l.handle != nil {
		C.logger_config_destroy(l.handle)
		l.handle = nil
	}
}

// toCStringArray converts a Go string slice to a C-allocated array of C strings.
// The returned cleanup function must be called to free all allocated memory.
func toCStringArray(strs []string) (**C.char, func()) {
	if len(strs) == 0 {
		return nil, func() {}
	}

	ptrSize := unsafe.Sizeof((*C.char)(nil))
	arr := (**C.char)(C.malloc(C.size_t(len(strs)) * C.size_t(ptrSize)))

	for i, s := range strs {
		elem := (**C.char)(unsafe.Pointer(uintptr(unsafe.Pointer(arr)) + uintptr(i)*ptrSize))
		*elem = C.CString(s)
	}

	return arr, func() {
		for i := range strs {
			elem := (**C.char)(unsafe.Pointer(uintptr(unsafe.Pointer(arr)) + uintptr(i)*ptrSize))
			C.free(unsafe.Pointer(*elem))
		}
		C.free(unsafe.Pointer(arr))
	}
}
