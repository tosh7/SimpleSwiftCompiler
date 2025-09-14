; ModuleID = 'swift_module'
source_filename = "swift_source"

declare i32 @printf(i8*, ...)

@.str = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1

define i32 @main() {
entry:
  %1 = add i32 42, 1
  %2 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str, i32 0, i32 0), i32 %1)
  ret i32 0
}
