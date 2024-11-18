# Documentation
|Type|description|
|---|---|
|[AstType](#AstType)||

|Value|description|
|---|---|
|[walk](#walk)||

## AstType

```moonbit
:::source,component/moonbit/interface/exports/share/walker/astWalker/top.mbt,3:::pub struct AstType {
  content : UInt64
  start : UInt64
  end : UInt64
}
```


#### mooncakes-io-implementation-mark-Implementations
- ```moonbit
  :::source,component/moonbit/interface/exports/share/walker/astWalker/top.mbt,5:::impl <a href="moonbitlang/core/builtin#Show">Show</a> for <a href="component/moonbit/interface/exports/share/walker/astWalker#AstType">AstType</a> with output(<a href="component/moonbit/interface/exports/share/walker/astWalker#AstType">AstType</a>, <a href="moonbitlang/core/builtin#Logger">Logger</a>) -> Unit
  ```
  > 

#### mooncakes-io-method-mark-Methods
- #### op\_equal
  ```moonbit
  :::source,component/moonbit/interface/exports/share/walker/astWalker/top.mbt,5:::fn <a href="component/moonbit/interface/exports/share/walker/astWalker#AstType">AstType</a>::op_equal(<a href="component/moonbit/interface/exports/share/walker/astWalker#AstType">AstType</a>, <a href="component/moonbit/interface/exports/share/walker/astWalker#AstType">AstType</a>) -> Bool
  ```
  > 
- #### to\_string
  ```moonbit
  :::source,component/moonbit/interface/exports/share/walker/astWalker/top.mbt,5:::fn <a href="component/moonbit/interface/exports/share/walker/astWalker#AstType">AstType</a>::to_string(<a href="component/moonbit/interface/exports/share/walker/astWalker#AstType">AstType</a>) -> String
  ```
  > 

## walk

```moonbit
:::source,component/moonbit/interface/exports/share/walker/astWalker/stub.mbt,3:::fn walk(ast : <a href="component/moonbit/interface/exports/share/walker/astWalker#AstType">AstType</a>) -> <a href="component/moonbit/interface/exports/share/walker/astWalker#AstType">AstType</a>
```

