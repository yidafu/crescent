use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub enum LuaValue {
    Nil,
    Boolean(bool),
    Integer(i64),
    Number(f64),
    String(String),
}

impl PartialEq for LuaValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl PartialOrd for LuaValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            LuaValue::Nil => match other {
                LuaValue::Nil => Some(Ordering::Equal),
                _ => None,
            },
            LuaValue::Boolean(a_val) => match other {
                LuaValue::Boolean(b_val) => Some(a_val.cmp(b_val)),
                _ => None,
            },
            LuaValue::Integer(a_int) => match other {
                LuaValue::Integer(b_int) => Some(a_int.cmp(b_int)),
                LuaValue::Number(b_float) => {
                    let a_float = a_int.clone() as f64;

                    a_float.clone().partial_cmp(b_float)
                }
                _ => None,
            },
            LuaValue::Number(a_float) => match other {
                LuaValue::Integer(b_int) => {
                    let b_float = b_int.clone() as f64;
                    a_float.clone().partial_cmp(&b_float)
                }
                LuaValue::Number(b_float) => a_float.clone().partial_cmp(b_float),
                _ => None,
            },
            LuaValue::String(a_str) => match other {
                LuaValue::String(b_str) => Some(a_str.cmp(b_str)),
                _ => None,
            },
        }
    }
}

impl TryInto<i64> for LuaValue {
    type Error = &'static str;

    fn try_into(self) -> Result<i64, Self::Error> {
        match self {
            LuaValue::Integer(val) => Ok(val),
            LuaValue::Number(v) => Ok(v.round() as i64),
            LuaValue::String(v) => {
                let res = v.parse::<i64>();
                if res.is_ok() {
                    Ok(res.unwrap())
                } else {
                    let f_res = v.parse::<f64>();
                    if f_res.is_ok() {
                        Ok(f_res.unwrap().round() as i64)
                    } else {
                        Err("could not convert string to int")
                    }
                }
            }
            _ => Err("Lua Value must be Integer/Number/String"),
        }
    }
}

impl TryInto<f64> for LuaValue {
    type Error = &'static str;

    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            LuaValue::Integer(v) => Ok(v as f64),
            LuaValue::Number(v) => Ok(v),
            LuaValue::String(v) => {
                let res = v.parse::<f64>();
                if res.is_ok() {
                    Ok(res.unwrap())
                } else {
                    Err("could not convert String to Number")
                }
            }
            _ => Err("Lua Value must be Integer/Number/String"),
        }
    }
}
