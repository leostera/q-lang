type label = string

type typ_cstr =
  | Int
  | String
  | Unit
  | Arrow
  | Array
  | 

and type_desc =
  | App of { app_con: typ_cstr; type_desc list }
  | Function of { fn_arg: type_desc; fn_ret: type_desc }
  | Struct of { fields: (label * type_desc) list }
