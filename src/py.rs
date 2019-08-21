//! Python bindings for `mice`. INCOMPLETE.
//! Only support Python 3.

use cpython::{
    py_class, py_class_impl, py_coerce_item, py_fn, py_method_def, py_module_initializer, PyResult,
    Python,
};

py_module_initializer!(libmice, initmice, PyInit_libmice, |py, m| {
    m.add(
        py,
        "__doc__",
        "
This module is implemented in Rust.
Things are still not good here.
Very little of mice's functionality is exposed.
",
    )?;
    m.add(py, "version", format!("{}", env!("CARGO_PKG_VERSION")))?;
    m.add(py, "roll", py_fn!(py, roll(a: &str)))?;
    m.add(py, "tupl_vec", py_fn!(py, tupl_vec(a: &str)))?;
    m.add(
        py,
        "roll_tupls",
        py_fn!(py, roll_tupls(a: Vec<crate::ExprTuple>)),
    )?;
    Ok(())
});

// Documentation comments placed inside this break it.
py_class!(class ExpressionResult |py| {
    data res: crate::ExpressionResult;
    def __format__(&self, format_spec: &str) -> PyResult<String> {
        if format_spec == "?" {
            Ok(format!("{:?}", self.res(py)))
        } else if format_spec == "#?" {
            Ok(format!("{:#?}", self.res(py)))
        } else {
            Ok(format!("{}", self.res(py)))
        }
    }
    // For now, this is the same as `__str__`, but when more verbose
    // descriptions are available, use them here and not there.
    def __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self.res(py)))
    }
    // Nice looking representation of the result of a roll.
    def __str__(&self) -> PyResult<String> {
        Ok(format!("{}", self.res(py)))
    }
    def total(&self) -> PyResult<i64> {
        Ok(self.res(py).total())
    }
});

fn roll(py: Python, input: &str) -> PyResult<ExpressionResult> {
    ExpressionResult::create_instance(py, crate::roll(input).unwrap())
}

fn tupl_vec(_: Python, input: &str) -> PyResult<Vec<(crate::ExprTuple)>> {
    Ok(crate::tupl_vec(input).unwrap())
}

fn roll_tupls(py: Python, input: Vec<crate::ExprTuple>) -> PyResult<ExpressionResult> {
    ExpressionResult::create_instance(py, crate::roll_tupls(&input).unwrap())
}
