meta:
  id: position_abs
  endian: le
seq:
  - id: index_offset
    type: u4
types:
  index:
    seq:
     - id: entry
       type: strz
       encoding: UTF-8
instances:
  index:
    position_abs: index_offset
    type: index
