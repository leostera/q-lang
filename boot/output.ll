; ModuleID = 'boot'
source_filename = "boot"

@format_string = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1

declare i32 @printf(ptr, ...)

define i32 @main() {
entry:
  %printf_call = call i32 (ptr, ...) @printf(ptr @format_string, i32 211200)
  ret i32 0
}
