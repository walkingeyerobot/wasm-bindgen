(module $reference_test.wasm
  (type (;0;) (func))
  (import "./reference_test_bg.js" "__wbindgen_init_externref_table" (func (;0;) (type 0)))
  (func $nop (;1;) (type 0))
  (table (;0;) 128 externref)
  (memory (;0;) 17)
  (export "memory" (memory 0))
  (export "nop" (func $nop))
  (export "__wbindgen_export_0" (table 0))
  (export "__wbindgen_start" (func 0))
  (@custom "target_features" (after code) "\04+\0amultivalue+\0fmutable-globals+\0freference-types+\08sign-ext")
)

