# Archive

## Archive Layout

```txt
[ "ANGI" ]
[ manifest_entry_count ]
[ manifest_length_in_byte ]
[
   entry
   [ name_length ]
   [ name ]
   [ offset_from_:blob-start ]
   [ byte_of_blob ]
]
:blob-start
[
  Blob
  [ blob ]
  [ blob ]
]
[Total byte | u32]
```
