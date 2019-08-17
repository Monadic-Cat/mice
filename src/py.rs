//! Python bindings for `mice`. INCOMPLETE.

use cpython::{
    py_fn,
    py_method_def,
    py_module_initializer,
    PyResult,
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
    m.add(py, "roll", py_fn!(py, roll(a: &str)))?;
    m.add(py, "tupl_vec", py_fn!(py, tupl_vec(a: &str)))?;
    m.add(py, "roll_tupls", py_fn!(py, roll_tupls(a: Vec<crate::ExprTuple>)))?;
    Ok(())
});

fn roll(_: Python, input: &str) -> PyResult<i64> {
    Ok(crate::roll(input).unwrap().total())
}

fn tupl_vec(_: Python, input: &str) -> PyResult<Vec<(crate::ExprTuple)>> {
    Ok(crate::tupl_vec(input).unwrap())
}

fn roll_tupls(_: Python, input: Vec<crate::ExprTuple>) -> PyResult<i64> {
    Ok(crate::roll_tupls(&input).unwrap().total())
}
