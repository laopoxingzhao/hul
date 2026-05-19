/// 值模块 - 定义解释器中使用的值类型和环境
///
/// 该模块包含：
/// - Value 枚举：表示解释器中的所有值类型
/// - ValueRef 类型：可变的引用计数值
/// - Environment 结构体：管理变量绑定和作用域
use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

/// 值枚举 - 表示解释器中所有可能的值类型
///
/// # 变体
/// - `Number(f64)`: 数值类型，用于存储浮点数
/// - `String(String)`: 字符串类型，用于存储文本
/// - `Boolean(bool)`: 布尔类型，用于存储真/假值
/// - `Nil`: 空值/无值，表示未初始化或不返回任何值
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// 数值类型 - 使用 f64 存储浮点数
    Number(f64),
    /// 字符串类型 - 存储文本数据
    String(String),
    /// 布尔类型 - 存储 true 或 false
    Boolean(bool),
    /// 空值 - 表示无值或 null
    Nil,
}

/// 为 Value 实现 Display trait，使其可以转换为字符串显示
impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // 数值直接转换为字符串
            Value::Number(n) => write!(f, "{}", n),
            // 字符串直接输出
            Value::String(s) => write!(f, "{}", s),
            // 布尔值转换为 "true" 或 "false"
            Value::Boolean(b) => write!(f, "{}", b),
            // 空值显示为 "nil"
            Value::Nil => write!(f, "nil"),
        }
    }
}

/// ValueRef 类型别名 - 引用计数的可变值引用
///
/// 使用 Rc<RefCell<Value>> 实现：
/// - Rc: 引用计数，允许多个所有者
/// - RefCell: 提供内部可变性，允许在不可变引用时修改值
pub type ValueRef = Rc<RefCell<Value>>;

/// 创建新的值引用
///
/// # 参数
/// - `value`: 要包装的 Value 值
///
/// # 返回
/// 返回一个新的 ValueRef，可用于在解释器中共享和修改值
///
/// # 示例
/// ```
/// let value_ref = new_value_ref(Value::Number(42.0));
/// ```
pub fn new_value_ref(value: Value) -> ValueRef {
    Rc::new(RefCell::new(value))
}

/// 判断值是否为"真"
///
/// 在条件判断中使用的真值判断逻辑：
/// - Nil 返回 false
/// - Boolean(false) 返回 false
/// - 其他所有值（包括 0 和空字符串）返回 true
///
/// # 参数
/// - `value`: 要判断的 Value 引用
///
/// # 返回
/// 返回 true 表示"真"，false 表示"假"
pub fn is_truthy(value: &Value) -> bool {
    match value {
        // nil 在条件判断中视为假
        Value::Nil => false,
        // 布尔值直接使用其真值
        Value::Boolean(b) => *b,
        // 其他所有值（数字、字符串）都视为真
        _ => true,
    }
}

/// 环境结构体 - 管理变量绑定和作用域链
///
/// 用于实现词法作用域，支持：
/// - 变量定义 (define)
/// - 变量赋值 (assign)
/// - 变量查找 (get)
/// - 嵌套作用域 (parent)
///
/// # 字段
/// - `bindings`: 存储当前作用域中的变量绑定 (变量名 -> 值引用)
/// - `parent`: 指向父作用域的引用，用于实现作用域链查找
#[derive(Debug, Clone)]
pub struct Environment {
    /// 当前作用域的变量绑定表
    bindings: HashMap<String, ValueRef>,
    /// 指向父作用域的引用，用于实现嵌套作用域
    parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    /// 创建新的顶级环境（无父作用域）
    ///
    /// # 返回
    /// 返回一个新的空环境，通常用于程序的全局作用域
    pub fn new() -> Self {
        Environment {
            bindings: HashMap::new(),
            parent: None,
        }
    }

    /// 创建带有父作用域的新环境
    ///
    /// 用于实现函数调用或块语句的作用域嵌套
    ///
    /// # 参数
    /// - `parent`: 父环境的引用
    ///
    /// # 返回
    /// 返回一个新环境，其 parent 指向传入的父环境
    pub fn new_with_parent(parent: Rc<RefCell<Environment>>) -> Self {
        Environment {
            bindings: HashMap::new(),
            parent: Some(parent),
        }
    }

    /// 定义新变量
    ///
    /// 在当前作用域中创建新的变量绑定
    ///
    /// # 参数
    /// - `name`: 变量名称
    /// - `value`: 变量的值引用
    ///
    /// # 注意
    /// 如果变量已存在，此操作会覆盖原有值
    pub fn define(&mut self, name: String, value: ValueRef) {
        self.bindings.insert(name, value);
    }

    /// 赋值现有变量
    ///
    /// 尝试为已存在的变量赋新值
    /// 如果当前作用域中没有该变量，会递归查找父作用域
    ///
    /// # 参数
    /// - `name`: 变量名称
    /// - `value`: 新的值引用
    ///
    /// # 返回
    /// - `Ok(())`: 赋值成功
    /// - `Err(String)`: 变量未定义
    pub fn assign(&mut self, name: &str, value: ValueRef) -> Result<(), String> {
        // 如果当前作用域中存在该变量，直接更新
        if self.bindings.contains_key(name) {
            self.bindings.insert(name.to_string(), value);
            Ok(())
        } else if let Some(parent) = &self.parent {
            // 否则尝试在父作用域中赋值
            parent.borrow_mut().assign(name, value)
        } else {
            // 未找到变量，返回错误
            Err(format!("Undefined variable '{}'", name))
        }
    }

    /// 获取变量的值
    ///
    /// 在当前作用域中查找变量，如果未找到则递归查找父作用域
    ///
    /// # 参数
    /// - `name`: 变量名称
    ///
    /// # 返回
    /// - `Ok(ValueRef)`: 找到的变量值引用
    /// - `Err(String)`: 变量未定义
    pub fn get(&self, name: &str) -> Result<ValueRef, String> {
        if let Some(val) = self.bindings.get(name) {
            // 在当前作用域中找到变量
            Ok(val.clone())
        } else if let Some(parent) = &self.parent {
            // 未找到，递归在父作用域中查找
            parent.borrow().get(name)
        } else {
            // 到达最外层作用域仍未找到
            Err(format!("Undefined variable '{}'", name))
        }
    }
}
