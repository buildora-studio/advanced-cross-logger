package crosslogger

/*
#cgo LDFLAGS: -L${SRCDIR}/../../target/release -lcross_logger -ldl -lpthread
#include <stdlib.h>

void log_info(const char* message);
void log_warn(const char* message);
void log_error(const char* message);
*/
import "C"
import "unsafe"

func LogInfo(message string) {
	cs := C.CString(message)
	defer C.free(unsafe.Pointer(cs))
	C.log_info(cs)
}

func LogWarn(message string) {
	cs := C.CString(message)
	defer C.free(unsafe.Pointer(cs))
	C.log_warn(cs)
}

func LogError(message string) {
	cs := C.CString(message)
	defer C.free(unsafe.Pointer(cs))
	C.log_error(cs)
}
