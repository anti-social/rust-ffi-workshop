#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::c_long;
use std::ptr;

mod py {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[repr(transparent)]
struct SyncWrapper<T>(T);
unsafe impl<T> Sync for SyncWrapper<T> {}

static mut PY_MODULE: py::PyModuleDef = py::PyModuleDef {
    // PyModuleDef_HEAD_INIT.
    m_base: py::PyModuleDef_Base {
        ob_base: py::PyObject {
            __bindgen_anon_1: py::_object__bindgen_ty_1 { ob_refcnt: 1 },
            ob_type: ptr::null_mut(),
        },
        m_init: None,
        m_index: 0,
        m_copy: ptr::null_mut(),
    },
    m_name: c"add".as_ptr(),
    m_doc: ptr::null(),
    // -1: module keeps state in globals, doesn't support sub-interpreters.
    m_size: -1,
    m_methods: PY_METHODS.0.as_ptr().cast_mut(),
    m_slots: ptr::null_mut(),
    m_traverse: None,
    m_clear: None,
    m_free: None,
};

static PY_METHODS: SyncWrapper<[py::PyMethodDef; 2]> = SyncWrapper([
    py::PyMethodDef {
        ml_name: c"add".as_ptr(),
        ml_meth: Some(add),
        ml_flags: py::METH_VARARGS as i32,
        ml_doc: ptr::null(),
    },
    py::PyMethodDef {
        ml_name: ptr::null(),
        ml_meth: None,
        ml_flags: 0,
        ml_doc: ptr::null(),
    },
]);

#[unsafe(no_mangle)]
extern "C" fn add(selfobj: *mut py::PyObject, args: *mut py::PyObject) -> *mut py::PyObject {
    let mut a: c_long = 0;
    let mut b: c_long = 0;
    if unsafe { py::PyArg_ParseTuple(args, c"ll".as_ptr(), &mut a, &mut b) } == 0 {
        return ptr::null_mut();
    }
    return unsafe { py::PyLong_FromLong(a + b) };
}

#[unsafe(no_mangle)]
pub extern "C" fn PyInit_add() -> *mut py::PyObject {
    unsafe { py::PyModule_Create2(&raw mut PY_MODULE, py::PYTHON_API_VERSION as i32) }
}
