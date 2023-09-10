type var = string

type ident =
  | Local of var
  | Remote of { idr_path: string list; name: var }

type literal =
  | Pid
  | Integer
  | Float
  | String
  | Atom

type pat =
  (* matching on literal *)
  | Const of literal
  (* match on a whole value binding it to a name *)
  | Bind of ident
  (* matching on lists *)
  | Cons of pat * pat list
  | Nil
  (* matching on tuples *)
  | Tuple of pat list
  (* matching on structs *)
  | Struct of (pat * pat) list
  (* matching on enums *)
  | Enum of (ident * pat list)

type expr =
  (* using a variable *)
  | Ident of ident
  (* assigning to a mutable variable *)
  | Assign of { asg_var: var; asg_body: expr }
  (* binding a new variable name to a value *)
  | Let of { let_var: var; let_body: expr }
  (* calling a function *)
  | Call of { call_name: expr; call_args: expr list }
  (* matching on an expression *)
  | Match of {m_on: expr; m_body: (pat * expr) list }
  (* receiving a message *)
  | Receive of { rcv_pat: (pat * expr) list; rcv_after: (expr * expr) }

type typ_ =
  | Unit
  | Struct of struct_type
  | Enum of enum_type

and field = string

and struct_type = {
  str_name: string;
  str_body: (field * typ_) list
}

and enum_type = {
  enum_name: string;
}

type type_decl =
  | Struct of struct_type
  | Enum of enum_type
