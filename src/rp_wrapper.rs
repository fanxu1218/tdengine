use std::collections::{HashMap};
use libc;

use td_rp::Value;
use td_rlua::{self, LuaPush, lua_State};


pub struct LuaWrapperValue(pub Value);

impl LuaPush for LuaWrapperValue {
    fn push_to_lua(self, lua: *mut lua_State) -> i32 {
        match self.0 {
            Value::Nil              => ().push_to_lua(lua),
            Value::U8(val)          => val.push_to_lua(lua),
            Value::I8(val)          => val.push_to_lua(lua),
            Value::U16(val)         => val.push_to_lua(lua),
            Value::I16(val)         => val.push_to_lua(lua),
            Value::U32(val)         => val.push_to_lua(lua),
            Value::I32(val)         => val.push_to_lua(lua),
            Value::Float(val)       => val.push_to_lua(lua),
            Value::Str(val)         => val.push_to_lua(lua),
            Value::Raw(val)         => {
                unsafe { td_rlua::lua_pushlstring(lua, val.as_ptr() as *const libc::c_char, val.len()) };
                1
            },
            Value::Map(mut val)     => {
                let mut wrapper_val : HashMap<String, LuaWrapperValue> = HashMap::new();
                for (k, v) in val.drain() {
                    wrapper_val.insert(k, LuaWrapperValue(v));
                }
                wrapper_val.push_to_lua(lua)
            },
            Value::AU8(mut val) | Value::AI8(mut val) | Value::AU16(mut val) | Value::AI16(mut val) | Value::AU32(mut val) | Value::AI32(mut val)
            | Value::AFloat(mut val) | Value::AStr(mut val) | Value::ARaw(mut val) | Value::AMap(mut val) => {
                let mut wrapper_val : Vec<LuaWrapperValue> = vec![];
                for v in val.drain(..) {
                    wrapper_val.push(LuaWrapperValue(v));
                }
                wrapper_val.push_to_lua(lua)
            },
        }
    }
}

// impl LuaRead for LuaWrapperValue {
//     fn lua_read_at_position(lua: *mut lua_State, index: i32) -> Option<LuaWrapperValue> {
//         None
//         // let args = unsafe { td_rlua::lua_gettop(lua) - index + 1 };
//         // if args <= 0 {
//         //     return None;
//         // }
//         // let mut cmd = Cmd::new();
//         // for i in 0 .. args {
//         //     let mut val : Option<String> = LuaRead::lua_read_at_position(lua, i + index);
//         //     if val.is_none() {
//         //         let bval : Option<bool> = LuaRead::lua_read_at_position(lua, i + index);
//         //         if let Some(b) = bval {
//         //             if b {
//         //                 val = Some("1".to_string());
//         //             } else {
//         //                 val = Some("0".to_string());
//         //             }
//         //         }
//         //     }

//         //     if val.is_none() {
//         //         return None;
//         //     }
//         //     cmd.arg(val.unwrap());
//         // }
//         // Some(RedisWrapperCmd(cmd))
//     }
// }


pub struct LuaWrapperVecValue(pub Vec<Value>);
impl LuaPush for LuaWrapperVecValue {
    fn push_to_lua(mut self, lua: *mut lua_State) -> i32 {
        let mut index = 0;
        for v in self.0.drain(..) {
            index = LuaWrapperValue(v).push_to_lua(lua);
        }
        index
    }
}


// impl LuaRead for LuaWrapperVecValue {
//     fn lua_read_at_position(lua: *mut lua_State, index: i32) -> Option<LuaWrapperVecValue> {
//         let args = unsafe { td_rlua::lua_gettop(lua) - index + 1 };
//         if args <= 0 {
//             return None;
//         }
//         let list = vec![];
//         for i in 0 .. args {
//             let mut val : Option<LuaWrapperValue> = LuaRead::lua_read_at_position(lua, i + index);
//             if val.is_none() {
//                 return None;
//             }
//             list.push(val.unwrap().0);
//         }
//         Some(LuaWrapperVecValue(list))
//     }
// }


pub struct LuaWrapperTableValue(pub Vec<Value>);
impl LuaPush for LuaWrapperTableValue {
    fn push_to_lua(mut self, lua: *mut lua_State) -> i32 {
        unsafe {
            td_rlua::lua_newtable(lua);
            for (i, v) in self.0.drain(..).enumerate() {
                td_rlua::lua_pushnumber(lua, (i + 1) as f64);
                LuaWrapperValue(v).push_to_lua(lua);
                td_rlua::lua_settable(lua, -3);
            }
        }
        1
    }
}