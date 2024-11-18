# Documentation
|Trait|description|
|---|---|
|[Any](#Any)||

|Type|description|
|---|---|
|[Cleanup](#Cleanup)||

|Value|description|
|---|---|
|[float\_array2ptr](#float_array2ptr)||
|[store64](#store64)||
|[malloc](#malloc)||
|[f32\_to\_i32](#f32_to_i32)||
|[loadf32](#loadf32)||
|[load8](#load8)||
|[ptr2float\_array](#ptr2float_array)||
|[ptr2int\_array](#ptr2int_array)||
|[bytes2ptr](#bytes2ptr)||
|[storef64](#storef64)||
|[uint64\_array2ptr](#uint64_array2ptr)||
|[load32](#load32)||
|[extend8](#extend8)||
|[load64](#load64)||
|[load16](#load16)||
|[store8](#store8)||
|[ptr2double\_array](#ptr2double_array)||
|[free](#free)||
|[ptr2int64\_array](#ptr2int64_array)||
|[double\_array2ptr](#double_array2ptr)||
|[ptr2str](#ptr2str)||
|[f32\_to\_i64](#f32_to_i64)||
|[ptr2bytes](#ptr2bytes)||
|[load16\_u](#load16_u)||
|[str2ptr](#str2ptr)||
|[ptr2uint\_array](#ptr2uint_array)||
|[int64\_array2ptr](#int64_array2ptr)||
|[int\_array2ptr](#int_array2ptr)||
|[copy](#copy)||
|[storef32](#storef32)||
|[loadf64](#loadf64)||
|[store32](#store32)||
|[uint\_array2ptr](#uint_array2ptr)||
|[load8\_u](#load8_u)||
|[ptr2uint64\_array](#ptr2uint64_array)||
|[store16](#store16)||
|[extend16](#extend16)||

## Any

```moonbit
:::source,component/moonbit/ffi/top.mbt,153:::pub trait Any {
}
```


## Cleanup

```moonbit
:::source,component/moonbit/ffi/top.mbt,154:::pub struct Cleanup {
  address : Int
  size : Int
  align : Int
}
```


## float\_array2ptr

```moonbit
:::source,component/moonbit/ffi/top.mbt,108:::fn float_array2ptr(array : <a href="moonbitlang/core/array#FixedArray">FixedArray</a>[float]) -> Int
```


## store64

```moonbit
:::source,component/moonbit/ffi/top.mbt,33:::fn store64(offset : Int, value : Int64) -> Unit
```


## malloc

```moonbit
:::source,component/moonbit/ffi/top.mbt,60:::fn malloc(size : Int) -> Int
```


## f32\_to\_i32

```moonbit
:::source,component/moonbit/ffi/top.mbt,51:::fn f32_to_i32(value : float) -> Int
```


## loadf32

```moonbit
:::source,component/moonbit/ffi/top.mbt,42:::fn loadf32(offset : Int) -> float
```


## load8

```moonbit
:::source,component/moonbit/ffi/top.mbt,15:::fn load8(offset : Int) -> Int
```


## ptr2float\_array

```moonbit
:::source,component/moonbit/ffi/top.mbt,120:::fn ptr2float_array(ptr : Int) -> <a href="moonbitlang/core/array#FixedArray">FixedArray</a>[float]
```


## ptr2int\_array

```moonbit
:::source,component/moonbit/ffi/top.mbt,117:::fn ptr2int_array(ptr : Int) -> <a href="moonbitlang/core/array#FixedArray">FixedArray</a>[Int]
```


## bytes2ptr

```moonbit
:::source,component/moonbit/ffi/top.mbt,90:::fn bytes2ptr(bytes : Bytes) -> Int
```


## storef64

```moonbit
:::source,component/moonbit/ffi/top.mbt,45:::fn storef64(offset : Int, value : Double) -> Unit
```


## uint64\_array2ptr

```moonbit
:::source,component/moonbit/ffi/top.mbt,99:::fn uint64_array2ptr(array : <a href="moonbitlang/core/array#FixedArray">FixedArray</a>[UInt64]) -> Int
```


## load32

```moonbit
:::source,component/moonbit/ffi/top.mbt,30:::fn load32(offset : Int) -> Int
```


## extend8

```moonbit
:::source,component/moonbit/ffi/top.mbt,6:::fn extend8(value : Int) -> Int
```


## load64

```moonbit
:::source,component/moonbit/ffi/top.mbt,36:::fn load64(offset : Int) -> Int64
```


## load16

```moonbit
:::source,component/moonbit/ffi/top.mbt,21:::fn load16(offset : Int) -> Int
```


## store8

```moonbit
:::source,component/moonbit/ffi/top.mbt,9:::fn store8(offset : Int, value : Int) -> Unit
```


## ptr2double\_array

```moonbit
:::source,component/moonbit/ffi/top.mbt,142:::fn ptr2double_array(ptr : Int) -> <a href="moonbitlang/core/array#FixedArray">FixedArray</a>[Double]
```


## free

```moonbit
:::source,component/moonbit/ffi/top.mbt,69:::fn free(position : Int) -> Unit
```


## ptr2int64\_array

```moonbit
:::source,component/moonbit/ffi/top.mbt,134:::fn ptr2int64_array(ptr : Int) -> <a href="moonbitlang/core/array#FixedArray">FixedArray</a>[Int64]
```


## double\_array2ptr

```moonbit
:::source,component/moonbit/ffi/top.mbt,111:::fn double_array2ptr(array : <a href="moonbitlang/core/array#FixedArray">FixedArray</a>[Double]) -> Int
```


## ptr2str

```moonbit
:::source,component/moonbit/ffi/top.mbt,87:::fn ptr2str(ptr : Int) -> String
```


## f32\_to\_i64

```moonbit
:::source,component/moonbit/ffi/top.mbt,54:::fn f32_to_i64(value : float) -> Int64
```


## ptr2bytes

```moonbit
:::source,component/moonbit/ffi/top.mbt,93:::fn ptr2bytes(ptr : Int) -> Bytes
```


## load16\_u

```moonbit
:::source,component/moonbit/ffi/top.mbt,24:::fn load16_u(offset : Int) -> Int
```


## str2ptr

```moonbit
:::source,component/moonbit/ffi/top.mbt,84:::fn str2ptr(str : String) -> Int
```


## ptr2uint\_array

```moonbit
:::source,component/moonbit/ffi/top.mbt,114:::fn ptr2uint_array(ptr : Int) -> <a href="moonbitlang/core/array#FixedArray">FixedArray</a>[UInt]
```


## int64\_array2ptr

```moonbit
:::source,component/moonbit/ffi/top.mbt,105:::fn int64_array2ptr(array : <a href="moonbitlang/core/array#FixedArray">FixedArray</a>[Int64]) -> Int
```


## int\_array2ptr

```moonbit
:::source,component/moonbit/ffi/top.mbt,102:::fn int_array2ptr(array : <a href="moonbitlang/core/array#FixedArray">FixedArray</a>[Int]) -> Int
```


## copy

```moonbit
:::source,component/moonbit/ffi/top.mbt,72:::fn copy(dest : Int, src : Int) -> Unit
```


## storef32

```moonbit
:::source,component/moonbit/ffi/top.mbt,39:::fn storef32(offset : Int, value : float) -> Unit
```


## loadf64

```moonbit
:::source,component/moonbit/ffi/top.mbt,48:::fn loadf64(offset : Int) -> Double
```


## store32

```moonbit
:::source,component/moonbit/ffi/top.mbt,27:::fn store32(offset : Int, value : Int) -> Unit
```


## uint\_array2ptr

```moonbit
:::source,component/moonbit/ffi/top.mbt,96:::fn uint_array2ptr(array : <a href="moonbitlang/core/array#FixedArray">FixedArray</a>[UInt]) -> Int
```


## load8\_u

```moonbit
:::source,component/moonbit/ffi/top.mbt,12:::fn load8_u(offset : Int) -> Int
```


## ptr2uint64\_array

```moonbit
:::source,component/moonbit/ffi/top.mbt,126:::fn ptr2uint64_array(ptr : Int) -> <a href="moonbitlang/core/array#FixedArray">FixedArray</a>[UInt64]
```


## store16

```moonbit
:::source,component/moonbit/ffi/top.mbt,18:::fn store16(offset : Int, value : Int) -> Unit
```


## extend16

```moonbit
:::source,component/moonbit/ffi/top.mbt,3:::fn extend16(value : Int) -> Int
```

