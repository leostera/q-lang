type label = string

type type_desc =
  | Bool of bool
  | Function of { fn_arg: type_desc; fn_ret: type_desc }
  | Struct of { fields: (label * type_desc) list }


