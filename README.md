# ReSob
Playing with Wasm binary (WIP)

## Code Structures
```
/src
    / parser - parser(decoder) codes
        -- module - module parser
        / sections - parser code for each section
            -- function_section
            -- type_section
            -- code_section
            / instructions - parser code for instructions; used to decode code section
```

## links
- https://zenn.dev/skanehira/books/writing-wasm-runtime-in-rust
- https://pengowray.github.io/wasm-ops/
- https://www.w3.org/TR/wasm-core-1/
