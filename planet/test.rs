#![feature(prelude_import)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
pub mod isostasy {}
pub mod surface {}
pub mod tectonics {
    pub mod generate_cells {
        use terrain_gen_core::stage::Stage;
        use terrain_gen_core::mesh::Mesh;
        use terrain_gen_core::data::{Fields, PlanetData};
        use terrain_gen_macros::godot_stage_export;
        use core;
        use rand::{Rng, SeedableRng};
        use rand::rngs::StdRng;
        use glam::Vec3;
        pub struct CellGenerator {
            seed: i64,
            cell_count: u32,
            jitter: f32,
        }
        #[class(base = Resource, init, tool, rename = CellGenerator)]
        struct __GodotCellGenerator {
            #[export]
            seed: i32,
            #[export]
            cell_count: u32,
            #[export]
            jitter: f32,
            base: ::godot::obj::Base<::godot::classes::Resource>,
        }
        impl ::godot::obj::GodotClass for __GodotCellGenerator {
            type Base = ::godot::classes::Resource;
            fn class_id() -> ::godot::meta::ClassId {
                use ::godot::meta::ClassId;
                static CLASS_ID: std::sync::OnceLock<ClassId> = std::sync::OnceLock::new();
                let id: &'static ClassId = CLASS_ID
                    .get_or_init(|| ClassId::__alloc_next_unicode("CellGenerator"));
                *id
            }
        }
        unsafe impl ::godot::obj::Bounds for __GodotCellGenerator {
            type Memory = <<Self as ::godot::obj::GodotClass>::Base as ::godot::obj::Bounds>::Memory;
            type DynMemory = <<Self as ::godot::obj::GodotClass>::Base as ::godot::obj::Bounds>::DynMemory;
            type Declarer = ::godot::obj::bounds::DeclUser;
            type Exportable = <<Self as ::godot::obj::GodotClass>::Base as ::godot::obj::Bounds>::Exportable;
        }
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        struct __godot___GodotCellGenerator_Funcs {}
        impl ::godot::obj::cap::GodotDefault for __GodotCellGenerator {
            fn __godot_user_init(
                base: ::godot::obj::Base<
                    <__GodotCellGenerator as ::godot::obj::GodotClass>::Base,
                >,
            ) -> Self {
                Self {
                    seed: ::std::default::Default::default(),
                    cell_count: ::std::default::Default::default(),
                    jitter: ::std::default::Default::default(),
                    base: base,
                }
            }
        }
        impl ::godot::obj::WithBaseField for __GodotCellGenerator {
            fn to_gd(&self) -> ::godot::obj::Gd<__GodotCellGenerator> {
                let base = <__GodotCellGenerator as ::godot::obj::WithBaseField>::base_field(
                    self,
                );
                base.__constructed_gd().cast()
            }
            fn base_field(
                &self,
            ) -> &::godot::obj::Base<
                <__GodotCellGenerator as ::godot::obj::GodotClass>::Base,
            > {
                &self.base
            }
        }
        impl __GodotCellGenerator {
            #[doc(hidden)]
            pub fn __godot_get_seed(&self) -> <i32 as ::godot::meta::GodotConvert>::Via {
                <i32 as ::godot::register::property::Var>::var_get(&self.seed)
            }
            #[deprecated = "Auto-generated Rust getters/setters for `#[var]` are being phased out until v0.6.\n\
                    If you need them, opt in with #[var(pub)]."]
            #[allow(dead_code)]
            pub fn get_seed(&self) -> <i32 as ::godot::meta::GodotConvert>::Via {
                self.__godot_get_seed()
            }
            #[doc(hidden)]
            pub fn __godot_set_seed(
                &mut self,
                seed: <i32 as ::godot::meta::GodotConvert>::Via,
            ) {
                <i32 as ::godot::register::property::Var>::var_set(&mut self.seed, seed)
            }
            #[deprecated = "Auto-generated Rust getters/setters for `#[var]` are being phased out until v0.6.\n\
                    If you need them, opt in with #[var(pub)]."]
            #[allow(dead_code)]
            pub fn set_seed(&mut self, seed: <i32 as ::godot::meta::GodotConvert>::Via) {
                self.__godot_set_seed(seed)
            }
            #[doc(hidden)]
            pub fn __godot_get_cell_count(
                &self,
            ) -> <u32 as ::godot::meta::GodotConvert>::Via {
                <u32 as ::godot::register::property::Var>::var_get(&self.cell_count)
            }
            #[deprecated = "Auto-generated Rust getters/setters for `#[var]` are being phased out until v0.6.\n\
                    If you need them, opt in with #[var(pub)]."]
            #[allow(dead_code)]
            pub fn get_cell_count(&self) -> <u32 as ::godot::meta::GodotConvert>::Via {
                self.__godot_get_cell_count()
            }
            #[doc(hidden)]
            pub fn __godot_set_cell_count(
                &mut self,
                cell_count: <u32 as ::godot::meta::GodotConvert>::Via,
            ) {
                <u32 as ::godot::register::property::Var>::var_set(
                    &mut self.cell_count,
                    cell_count,
                )
            }
            #[deprecated = "Auto-generated Rust getters/setters for `#[var]` are being phased out until v0.6.\n\
                    If you need them, opt in with #[var(pub)]."]
            #[allow(dead_code)]
            pub fn set_cell_count(
                &mut self,
                cell_count: <u32 as ::godot::meta::GodotConvert>::Via,
            ) {
                self.__godot_set_cell_count(cell_count)
            }
            #[doc(hidden)]
            pub fn __godot_get_jitter(
                &self,
            ) -> <f32 as ::godot::meta::GodotConvert>::Via {
                <f32 as ::godot::register::property::Var>::var_get(&self.jitter)
            }
            #[deprecated = "Auto-generated Rust getters/setters for `#[var]` are being phased out until v0.6.\n\
                    If you need them, opt in with #[var(pub)]."]
            #[allow(dead_code)]
            pub fn get_jitter(&self) -> <f32 as ::godot::meta::GodotConvert>::Via {
                self.__godot_get_jitter()
            }
            #[doc(hidden)]
            pub fn __godot_set_jitter(
                &mut self,
                jitter: <f32 as ::godot::meta::GodotConvert>::Via,
            ) {
                <f32 as ::godot::register::property::Var>::var_set(
                    &mut self.jitter,
                    jitter,
                )
            }
            #[deprecated = "Auto-generated Rust getters/setters for `#[var]` are being phased out until v0.6.\n\
                    If you need them, opt in with #[var(pub)]."]
            #[allow(dead_code)]
            pub fn set_jitter(
                &mut self,
                jitter: <f32 as ::godot::meta::GodotConvert>::Via,
            ) {
                self.__godot_set_jitter(jitter)
            }
        }
        impl __godot___GodotCellGenerator_Funcs {
            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            pub const __godot_get_seed: &str = "get_seed";
            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            pub const __godot_set_seed: &str = "set_seed";
            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            pub const __godot_get_cell_count: &str = "get_cell_count";
            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            pub const __godot_set_cell_count: &str = "set_cell_count";
            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            pub const __godot_get_jitter: &str = "get_jitter";
            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            pub const __godot_set_jitter: &str = "set_jitter";
        }
        impl ::godot::obj::cap::ImplementsGodotExports for __GodotCellGenerator {
            fn __register_exports() {
                {}
                {}
                {
                    {
                        use ::godot::obj::GodotClass;
                        use ::godot::register::private::method::ClassMethodInfo;
                        use ::godot::builtin::{StringName, Variant};
                        use ::godot::sys;
                        type CallParams = ();
                        type CallRet = <i32 as ::godot::meta::GodotConvert>::Via;
                        let method_name = StringName::from("get_seed");
                        unsafe extern "C" fn varcall_fn(
                            _method_data: *mut std::ffi::c_void,
                            instance_ptr: sys::GDExtensionClassInstancePtr,
                            args_ptr: *const sys::GDExtensionConstVariantPtr,
                            arg_count: sys::GDExtensionInt,
                            ret: sys::GDExtensionVariantPtr,
                            err: *mut sys::GDExtensionCallError,
                        ) {
                            let call_ctx = ::godot::private::CallContext::func(
                                "__GodotCellGenerator",
                                "get_seed",
                            );
                            ::godot::private::handle_fallible_varcall(
                                &call_ctx,
                                &mut *err,
                                || {
                                    let defaults = ::alloc::vec::Vec::new();
                                    ::godot::private::Signature::<
                                        CallParams,
                                        CallRet,
                                    >::in_varcall(
                                        instance_ptr,
                                        &call_ctx,
                                        args_ptr,
                                        arg_count,
                                        &defaults,
                                        ret,
                                        err,
                                        |instance_ptr, params| {
                                            let () = params;
                                            let storage = unsafe {
                                                ::godot::private::as_storage::<
                                                    __GodotCellGenerator,
                                                >(instance_ptr)
                                            };
                                            let __gdext_self = ::godot::private::Storage::get(storage);
                                            __gdext_self.__godot_get_seed()
                                        },
                                    )
                                },
                            );
                        }
                        unsafe extern "C" fn ptrcall_fn(
                            _method_data: *mut std::ffi::c_void,
                            instance_ptr: sys::GDExtensionClassInstancePtr,
                            args_ptr: *const sys::GDExtensionConstTypePtr,
                            ret: sys::GDExtensionTypePtr,
                        ) {
                            let call_ctx = ::godot::private::CallContext::func(
                                "__GodotCellGenerator",
                                "get_seed",
                            );
                            ::godot::private::handle_fallible_ptrcall(
                                &call_ctx,
                                || ::godot::private::Signature::<
                                    CallParams,
                                    CallRet,
                                >::in_ptrcall(
                                    instance_ptr,
                                    &call_ctx,
                                    args_ptr,
                                    ret,
                                    |instance_ptr, params| {
                                        let () = params;
                                        let storage = unsafe {
                                            ::godot::private::as_storage::<
                                                __GodotCellGenerator,
                                            >(instance_ptr)
                                        };
                                        let __gdext_self = ::godot::private::Storage::get(storage);
                                        __gdext_self.__godot_get_seed()
                                    },
                                    sys::PtrcallType::Standard,
                                ),
                            );
                        }
                        let method_info = unsafe {
                            ClassMethodInfo::from_signature::<
                                __GodotCellGenerator,
                                CallParams,
                                CallRet,
                            >(
                                method_name,
                                Some(varcall_fn),
                                Some(ptrcall_fn),
                                ::godot::register::info::MethodFlags::NORMAL
                                    | ::godot::register::info::MethodFlags::CONST,
                                &[],
                                ::alloc::vec::Vec::new(),
                            )
                        };
                        {
                            if false {
                                format_args!(
                                    "   Register fn:   {0}::{1}",
                                    "__GodotCellGenerator",
                                    "get_seed",
                                );
                            }
                        };
                        method_info.register_extension_class_method();
                    };
                }
                {
                    {
                        use ::godot::obj::GodotClass;
                        use ::godot::register::private::method::ClassMethodInfo;
                        use ::godot::builtin::{StringName, Variant};
                        use ::godot::sys;
                        type CallParams = (<i32 as ::godot::meta::GodotConvert>::Via,);
                        type CallRet = ();
                        let method_name = StringName::from("set_seed");
                        unsafe extern "C" fn varcall_fn(
                            _method_data: *mut std::ffi::c_void,
                            instance_ptr: sys::GDExtensionClassInstancePtr,
                            args_ptr: *const sys::GDExtensionConstVariantPtr,
                            arg_count: sys::GDExtensionInt,
                            ret: sys::GDExtensionVariantPtr,
                            err: *mut sys::GDExtensionCallError,
                        ) {
                            let call_ctx = ::godot::private::CallContext::func(
                                "__GodotCellGenerator",
                                "set_seed",
                            );
                            ::godot::private::handle_fallible_varcall(
                                &call_ctx,
                                &mut *err,
                                || {
                                    let defaults = ::alloc::vec::Vec::new();
                                    ::godot::private::Signature::<
                                        CallParams,
                                        CallRet,
                                    >::in_varcall(
                                        instance_ptr,
                                        &call_ctx,
                                        args_ptr,
                                        arg_count,
                                        &defaults,
                                        ret,
                                        err,
                                        |instance_ptr, params| {
                                            let (seed,) = params;
                                            let storage = unsafe {
                                                ::godot::private::as_storage::<
                                                    __GodotCellGenerator,
                                                >(instance_ptr)
                                            };
                                            let mut __gdext_self = ::godot::private::Storage::get_mut(
                                                storage,
                                            );
                                            __gdext_self.__godot_set_seed(seed)
                                        },
                                    )
                                },
                            );
                        }
                        unsafe extern "C" fn ptrcall_fn(
                            _method_data: *mut std::ffi::c_void,
                            instance_ptr: sys::GDExtensionClassInstancePtr,
                            args_ptr: *const sys::GDExtensionConstTypePtr,
                            ret: sys::GDExtensionTypePtr,
                        ) {
                            let call_ctx = ::godot::private::CallContext::func(
                                "__GodotCellGenerator",
                                "set_seed",
                            );
                            ::godot::private::handle_fallible_ptrcall(
                                &call_ctx,
                                || ::godot::private::Signature::<
                                    CallParams,
                                    CallRet,
                                >::in_ptrcall(
                                    instance_ptr,
                                    &call_ctx,
                                    args_ptr,
                                    ret,
                                    |instance_ptr, params| {
                                        let (seed,) = params;
                                        let storage = unsafe {
                                            ::godot::private::as_storage::<
                                                __GodotCellGenerator,
                                            >(instance_ptr)
                                        };
                                        let mut __gdext_self = ::godot::private::Storage::get_mut(
                                            storage,
                                        );
                                        __gdext_self.__godot_set_seed(seed)
                                    },
                                    sys::PtrcallType::Standard,
                                ),
                            );
                        }
                        let method_info = unsafe {
                            ClassMethodInfo::from_signature::<
                                __GodotCellGenerator,
                                CallParams,
                                CallRet,
                            >(
                                method_name,
                                Some(varcall_fn),
                                Some(ptrcall_fn),
                                ::godot::register::info::MethodFlags::NORMAL,
                                &["seed"],
                                ::alloc::vec::Vec::new(),
                            )
                        };
                        {
                            if false {
                                format_args!(
                                    "   Register fn:   {0}::{1}",
                                    "__GodotCellGenerator",
                                    "set_seed",
                                );
                            }
                        };
                        method_info.register_extension_class_method();
                    };
                }
                {
                    type FieldType = i32;
                    ::godot::register::private::register_export::<
                        __GodotCellGenerator,
                        FieldType,
                    >(
                        "seed",
                        __godot___GodotCellGenerator_Funcs::__godot_get_seed,
                        __godot___GodotCellGenerator_Funcs::__godot_set_seed,
                        None,
                        None,
                    );
                }
                {}
                {}
                {
                    {
                        use ::godot::obj::GodotClass;
                        use ::godot::register::private::method::ClassMethodInfo;
                        use ::godot::builtin::{StringName, Variant};
                        use ::godot::sys;
                        type CallParams = ();
                        type CallRet = <u32 as ::godot::meta::GodotConvert>::Via;
                        let method_name = StringName::from("get_cell_count");
                        unsafe extern "C" fn varcall_fn(
                            _method_data: *mut std::ffi::c_void,
                            instance_ptr: sys::GDExtensionClassInstancePtr,
                            args_ptr: *const sys::GDExtensionConstVariantPtr,
                            arg_count: sys::GDExtensionInt,
                            ret: sys::GDExtensionVariantPtr,
                            err: *mut sys::GDExtensionCallError,
                        ) {
                            let call_ctx = ::godot::private::CallContext::func(
                                "__GodotCellGenerator",
                                "get_cell_count",
                            );
                            ::godot::private::handle_fallible_varcall(
                                &call_ctx,
                                &mut *err,
                                || {
                                    let defaults = ::alloc::vec::Vec::new();
                                    ::godot::private::Signature::<
                                        CallParams,
                                        CallRet,
                                    >::in_varcall(
                                        instance_ptr,
                                        &call_ctx,
                                        args_ptr,
                                        arg_count,
                                        &defaults,
                                        ret,
                                        err,
                                        |instance_ptr, params| {
                                            let () = params;
                                            let storage = unsafe {
                                                ::godot::private::as_storage::<
                                                    __GodotCellGenerator,
                                                >(instance_ptr)
                                            };
                                            let __gdext_self = ::godot::private::Storage::get(storage);
                                            __gdext_self.__godot_get_cell_count()
                                        },
                                    )
                                },
                            );
                        }
                        unsafe extern "C" fn ptrcall_fn(
                            _method_data: *mut std::ffi::c_void,
                            instance_ptr: sys::GDExtensionClassInstancePtr,
                            args_ptr: *const sys::GDExtensionConstTypePtr,
                            ret: sys::GDExtensionTypePtr,
                        ) {
                            let call_ctx = ::godot::private::CallContext::func(
                                "__GodotCellGenerator",
                                "get_cell_count",
                            );
                            ::godot::private::handle_fallible_ptrcall(
                                &call_ctx,
                                || ::godot::private::Signature::<
                                    CallParams,
                                    CallRet,
                                >::in_ptrcall(
                                    instance_ptr,
                                    &call_ctx,
                                    args_ptr,
                                    ret,
                                    |instance_ptr, params| {
                                        let () = params;
                                        let storage = unsafe {
                                            ::godot::private::as_storage::<
                                                __GodotCellGenerator,
                                            >(instance_ptr)
                                        };
                                        let __gdext_self = ::godot::private::Storage::get(storage);
                                        __gdext_self.__godot_get_cell_count()
                                    },
                                    sys::PtrcallType::Standard,
                                ),
                            );
                        }
                        let method_info = unsafe {
                            ClassMethodInfo::from_signature::<
                                __GodotCellGenerator,
                                CallParams,
                                CallRet,
                            >(
                                method_name,
                                Some(varcall_fn),
                                Some(ptrcall_fn),
                                ::godot::register::info::MethodFlags::NORMAL
                                    | ::godot::register::info::MethodFlags::CONST,
                                &[],
                                ::alloc::vec::Vec::new(),
                            )
                        };
                        {
                            if false {
                                format_args!(
                                    "   Register fn:   {0}::{1}",
                                    "__GodotCellGenerator",
                                    "get_cell_count",
                                );
                            }
                        };
                        method_info.register_extension_class_method();
                    };
                }
                {
                    {
                        use ::godot::obj::GodotClass;
                        use ::godot::register::private::method::ClassMethodInfo;
                        use ::godot::builtin::{StringName, Variant};
                        use ::godot::sys;
                        type CallParams = (<u32 as ::godot::meta::GodotConvert>::Via,);
                        type CallRet = ();
                        let method_name = StringName::from("set_cell_count");
                        unsafe extern "C" fn varcall_fn(
                            _method_data: *mut std::ffi::c_void,
                            instance_ptr: sys::GDExtensionClassInstancePtr,
                            args_ptr: *const sys::GDExtensionConstVariantPtr,
                            arg_count: sys::GDExtensionInt,
                            ret: sys::GDExtensionVariantPtr,
                            err: *mut sys::GDExtensionCallError,
                        ) {
                            let call_ctx = ::godot::private::CallContext::func(
                                "__GodotCellGenerator",
                                "set_cell_count",
                            );
                            ::godot::private::handle_fallible_varcall(
                                &call_ctx,
                                &mut *err,
                                || {
                                    let defaults = ::alloc::vec::Vec::new();
                                    ::godot::private::Signature::<
                                        CallParams,
                                        CallRet,
                                    >::in_varcall(
                                        instance_ptr,
                                        &call_ctx,
                                        args_ptr,
                                        arg_count,
                                        &defaults,
                                        ret,
                                        err,
                                        |instance_ptr, params| {
                                            let (cell_count,) = params;
                                            let storage = unsafe {
                                                ::godot::private::as_storage::<
                                                    __GodotCellGenerator,
                                                >(instance_ptr)
                                            };
                                            let mut __gdext_self = ::godot::private::Storage::get_mut(
                                                storage,
                                            );
                                            __gdext_self.__godot_set_cell_count(cell_count)
                                        },
                                    )
                                },
                            );
                        }
                        unsafe extern "C" fn ptrcall_fn(
                            _method_data: *mut std::ffi::c_void,
                            instance_ptr: sys::GDExtensionClassInstancePtr,
                            args_ptr: *const sys::GDExtensionConstTypePtr,
                            ret: sys::GDExtensionTypePtr,
                        ) {
                            let call_ctx = ::godot::private::CallContext::func(
                                "__GodotCellGenerator",
                                "set_cell_count",
                            );
                            ::godot::private::handle_fallible_ptrcall(
                                &call_ctx,
                                || ::godot::private::Signature::<
                                    CallParams,
                                    CallRet,
                                >::in_ptrcall(
                                    instance_ptr,
                                    &call_ctx,
                                    args_ptr,
                                    ret,
                                    |instance_ptr, params| {
                                        let (cell_count,) = params;
                                        let storage = unsafe {
                                            ::godot::private::as_storage::<
                                                __GodotCellGenerator,
                                            >(instance_ptr)
                                        };
                                        let mut __gdext_self = ::godot::private::Storage::get_mut(
                                            storage,
                                        );
                                        __gdext_self.__godot_set_cell_count(cell_count)
                                    },
                                    sys::PtrcallType::Standard,
                                ),
                            );
                        }
                        let method_info = unsafe {
                            ClassMethodInfo::from_signature::<
                                __GodotCellGenerator,
                                CallParams,
                                CallRet,
                            >(
                                method_name,
                                Some(varcall_fn),
                                Some(ptrcall_fn),
                                ::godot::register::info::MethodFlags::NORMAL,
                                &["cell_count"],
                                ::alloc::vec::Vec::new(),
                            )
                        };
                        {
                            if false {
                                format_args!(
                                    "   Register fn:   {0}::{1}",
                                    "__GodotCellGenerator",
                                    "set_cell_count",
                                );
                            }
                        };
                        method_info.register_extension_class_method();
                    };
                }
                {
                    type FieldType = u32;
                    ::godot::register::private::register_export::<
                        __GodotCellGenerator,
                        FieldType,
                    >(
                        "cell_count",
                        __godot___GodotCellGenerator_Funcs::__godot_get_cell_count,
                        __godot___GodotCellGenerator_Funcs::__godot_set_cell_count,
                        None,
                        None,
                    );
                }
                {}
                {}
                {
                    {
                        use ::godot::obj::GodotClass;
                        use ::godot::register::private::method::ClassMethodInfo;
                        use ::godot::builtin::{StringName, Variant};
                        use ::godot::sys;
                        type CallParams = ();
                        type CallRet = <f32 as ::godot::meta::GodotConvert>::Via;
                        let method_name = StringName::from("get_jitter");
                        unsafe extern "C" fn varcall_fn(
                            _method_data: *mut std::ffi::c_void,
                            instance_ptr: sys::GDExtensionClassInstancePtr,
                            args_ptr: *const sys::GDExtensionConstVariantPtr,
                            arg_count: sys::GDExtensionInt,
                            ret: sys::GDExtensionVariantPtr,
                            err: *mut sys::GDExtensionCallError,
                        ) {
                            let call_ctx = ::godot::private::CallContext::func(
                                "__GodotCellGenerator",
                                "get_jitter",
                            );
                            ::godot::private::handle_fallible_varcall(
                                &call_ctx,
                                &mut *err,
                                || {
                                    let defaults = ::alloc::vec::Vec::new();
                                    ::godot::private::Signature::<
                                        CallParams,
                                        CallRet,
                                    >::in_varcall(
                                        instance_ptr,
                                        &call_ctx,
                                        args_ptr,
                                        arg_count,
                                        &defaults,
                                        ret,
                                        err,
                                        |instance_ptr, params| {
                                            let () = params;
                                            let storage = unsafe {
                                                ::godot::private::as_storage::<
                                                    __GodotCellGenerator,
                                                >(instance_ptr)
                                            };
                                            let __gdext_self = ::godot::private::Storage::get(storage);
                                            __gdext_self.__godot_get_jitter()
                                        },
                                    )
                                },
                            );
                        }
                        unsafe extern "C" fn ptrcall_fn(
                            _method_data: *mut std::ffi::c_void,
                            instance_ptr: sys::GDExtensionClassInstancePtr,
                            args_ptr: *const sys::GDExtensionConstTypePtr,
                            ret: sys::GDExtensionTypePtr,
                        ) {
                            let call_ctx = ::godot::private::CallContext::func(
                                "__GodotCellGenerator",
                                "get_jitter",
                            );
                            ::godot::private::handle_fallible_ptrcall(
                                &call_ctx,
                                || ::godot::private::Signature::<
                                    CallParams,
                                    CallRet,
                                >::in_ptrcall(
                                    instance_ptr,
                                    &call_ctx,
                                    args_ptr,
                                    ret,
                                    |instance_ptr, params| {
                                        let () = params;
                                        let storage = unsafe {
                                            ::godot::private::as_storage::<
                                                __GodotCellGenerator,
                                            >(instance_ptr)
                                        };
                                        let __gdext_self = ::godot::private::Storage::get(storage);
                                        __gdext_self.__godot_get_jitter()
                                    },
                                    sys::PtrcallType::Standard,
                                ),
                            );
                        }
                        let method_info = unsafe {
                            ClassMethodInfo::from_signature::<
                                __GodotCellGenerator,
                                CallParams,
                                CallRet,
                            >(
                                method_name,
                                Some(varcall_fn),
                                Some(ptrcall_fn),
                                ::godot::register::info::MethodFlags::NORMAL
                                    | ::godot::register::info::MethodFlags::CONST,
                                &[],
                                ::alloc::vec::Vec::new(),
                            )
                        };
                        {
                            if false {
                                format_args!(
                                    "   Register fn:   {0}::{1}",
                                    "__GodotCellGenerator",
                                    "get_jitter",
                                );
                            }
                        };
                        method_info.register_extension_class_method();
                    };
                }
                {
                    {
                        use ::godot::obj::GodotClass;
                        use ::godot::register::private::method::ClassMethodInfo;
                        use ::godot::builtin::{StringName, Variant};
                        use ::godot::sys;
                        type CallParams = (<f32 as ::godot::meta::GodotConvert>::Via,);
                        type CallRet = ();
                        let method_name = StringName::from("set_jitter");
                        unsafe extern "C" fn varcall_fn(
                            _method_data: *mut std::ffi::c_void,
                            instance_ptr: sys::GDExtensionClassInstancePtr,
                            args_ptr: *const sys::GDExtensionConstVariantPtr,
                            arg_count: sys::GDExtensionInt,
                            ret: sys::GDExtensionVariantPtr,
                            err: *mut sys::GDExtensionCallError,
                        ) {
                            let call_ctx = ::godot::private::CallContext::func(
                                "__GodotCellGenerator",
                                "set_jitter",
                            );
                            ::godot::private::handle_fallible_varcall(
                                &call_ctx,
                                &mut *err,
                                || {
                                    let defaults = ::alloc::vec::Vec::new();
                                    ::godot::private::Signature::<
                                        CallParams,
                                        CallRet,
                                    >::in_varcall(
                                        instance_ptr,
                                        &call_ctx,
                                        args_ptr,
                                        arg_count,
                                        &defaults,
                                        ret,
                                        err,
                                        |instance_ptr, params| {
                                            let (jitter,) = params;
                                            let storage = unsafe {
                                                ::godot::private::as_storage::<
                                                    __GodotCellGenerator,
                                                >(instance_ptr)
                                            };
                                            let mut __gdext_self = ::godot::private::Storage::get_mut(
                                                storage,
                                            );
                                            __gdext_self.__godot_set_jitter(jitter)
                                        },
                                    )
                                },
                            );
                        }
                        unsafe extern "C" fn ptrcall_fn(
                            _method_data: *mut std::ffi::c_void,
                            instance_ptr: sys::GDExtensionClassInstancePtr,
                            args_ptr: *const sys::GDExtensionConstTypePtr,
                            ret: sys::GDExtensionTypePtr,
                        ) {
                            let call_ctx = ::godot::private::CallContext::func(
                                "__GodotCellGenerator",
                                "set_jitter",
                            );
                            ::godot::private::handle_fallible_ptrcall(
                                &call_ctx,
                                || ::godot::private::Signature::<
                                    CallParams,
                                    CallRet,
                                >::in_ptrcall(
                                    instance_ptr,
                                    &call_ctx,
                                    args_ptr,
                                    ret,
                                    |instance_ptr, params| {
                                        let (jitter,) = params;
                                        let storage = unsafe {
                                            ::godot::private::as_storage::<
                                                __GodotCellGenerator,
                                            >(instance_ptr)
                                        };
                                        let mut __gdext_self = ::godot::private::Storage::get_mut(
                                            storage,
                                        );
                                        __gdext_self.__godot_set_jitter(jitter)
                                    },
                                    sys::PtrcallType::Standard,
                                ),
                            );
                        }
                        let method_info = unsafe {
                            ClassMethodInfo::from_signature::<
                                __GodotCellGenerator,
                                CallParams,
                                CallRet,
                            >(
                                method_name,
                                Some(varcall_fn),
                                Some(ptrcall_fn),
                                ::godot::register::info::MethodFlags::NORMAL,
                                &["jitter"],
                                ::alloc::vec::Vec::new(),
                            )
                        };
                        {
                            if false {
                                format_args!(
                                    "   Register fn:   {0}::{1}",
                                    "__GodotCellGenerator",
                                    "set_jitter",
                                );
                            }
                        };
                        method_info.register_extension_class_method();
                    };
                }
                {
                    type FieldType = f32;
                    ::godot::register::private::register_export::<
                        __GodotCellGenerator,
                        FieldType,
                    >(
                        "jitter",
                        __godot___GodotCellGenerator_Funcs::__godot_get_jitter,
                        __godot___GodotCellGenerator_Funcs::__godot_set_jitter,
                        None,
                        None,
                    );
                }
            }
        }
        impl ::godot::obj::UserClass for __GodotCellGenerator {
            #[doc(hidden)]
            fn __config() -> ::godot::private::ClassConfig {
                ::godot::private::ClassConfig {
                    is_tool: true,
                }
            }
            #[doc(hidden)]
            fn __before_ready(&mut self) {
                ::godot::register::private::auto_register_rpcs::<
                    __GodotCellGenerator,
                >(self);
            }
        }
        const _: () = {
            #[allow(non_upper_case_globals)]
            #[used]
            #[unsafe(link_section = ".init_array")]
            static __init: extern "C" fn() = {
                #[unsafe(link_section = ".text.startup")]
                extern "C" fn __inner_init() {
                    {
                        ::godot::private::__GODOT_SHARD_REGISTRY
                            .lock()
                            .unwrap()
                            .push(
                                ::godot::private::ClassShard::new::<
                                    __GodotCellGenerator,
                                >(
                                    ::godot::private::ShardItem::Struct(
                                        ::godot::private::Struct::new::<__GodotCellGenerator>()
                                            .with_generated::<__GodotCellGenerator>()
                                            .with_instantiable()
                                            .with_tool(),
                                    ),
                                ),
                            );
                    }
                }
                __inner_init
            };
        };
        unsafe impl ::godot::obj::Inherits<::godot::classes::Resource>
        for __GodotCellGenerator {}
        unsafe impl ::godot::obj::Inherits<::godot::classes::RefCounted>
        for __GodotCellGenerator {}
        unsafe impl ::godot::obj::Inherits<::godot::classes::Object>
        for __GodotCellGenerator {}
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    terrain_gen_core::stage::StageConverter(|res| {
                        res.try_cast::<__GodotCellGenerator>()
                            .ok()
                            .map(|g| {
                                let bound = g.bind();
                                let plain = CellGenerator {
                                    seed: bound.seed.clone().into(),
                                    cell_count: bound.cell_count.clone().into(),
                                    jitter: bound.jitter.clone().into(),
                                };
                                Box::new(plain) as Box<dyn terrain_gen_core::stage::Stage>
                            })
                    })
                },
                next: ::inventory::__private::UnsafeCell::new(
                    ::inventory::__private::Option::None,
                ),
            };
            #[link_section = ".text.startup"]
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = ".init_array"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
        struct TriangulationResult {
            pub triangles: Vec<u32>,
            pub halfedges: Vec<u32>,
        }
        impl CellGenerator {
            fn make_rng(&self) -> Box<dyn Rng> {
                Box::new(StdRng::seed_from_u64(self.seed as u64))
            }
            fn generate_fibbonacy_points(&self, mut rng: Box<dyn Rng>) -> Vec<Vec3> {
                let n = self.cell_count as usize;
                let jitter = self.jitter as f64;
                if n == 1 {
                    return ::alloc::boxed::box_assume_init_into_vec_unsafe(
                        ::alloc::intrinsics::write_box_via_move(
                            ::alloc::boxed::Box::new_uninit(),
                            [Vec3::new(0.0, 1.0, 0.0)],
                        ),
                    );
                }
                let mut points: Vec<Vec3> = Vec::with_capacity(n);
                const PI: f64 = core::f64::consts::PI;
                let s = 3.6 / (n as f64).sqrt();
                let dlong = PI * (3. - f64::sqrt(5.));
                let dz = 2.0 / (n as f64);
                let mut long: f64 = 0.0;
                let mut z = 1. - dz / 2.;
                for _ in 0..n {
                    let r = (1. - z * z).sqrt();
                    let mut lat = z.asin();
                    let mut lon = long;
                    let rand_lat = rng.next_u32() as f64 / u32::MAX as f64;
                    let rand_long = rng.next_u32() as f64 / u32::MAX as f64;
                    lat
                        += jitter * rand_lat
                            * (lat - f64::asin(f64::max(-1., z - dz * 2. * PI * r / s)));
                    lon += jitter * rand_long * s / r;
                    points
                        .push(Vec3 {
                            x: (lat.cos() * lon.cos()) as f32,
                            y: (lat.cos() * lon.sin()) as f32,
                            z: (lat.sin()) as f32,
                        });
                    z -= dz;
                    long += dlong;
                }
                points
            }
            fn triangulate(&self, points: &mut Vec<Vec3>) -> TriangulationResult {
                use delaunator::{EMPTY, Point, Triangulation, triangulate};
                fn stereographic_projection(points: &Vec<Vec3>) -> Vec<Point> {
                    points
                        .iter()
                        .map(|point| {
                            Point {
                                x: (point.x / (1. - point.z)) as f64,
                                y: (point.y / (1. - point.z)) as f64,
                            }
                        })
                        .collect()
                }
                fn add_pole(pole_id: usize, triangulation: &mut Triangulation) {
                    let n = triangulation.triangles.len();
                    fn next(i: usize) -> usize {
                        if (i % 3) == 2 { i - 2 } else { i + 1 }
                    }
                    let mut num_unpaired: usize = 0;
                    let mut first_unpaired: usize = 0;
                    let mut point_to_side = ::alloc::vec::from_elem(0, n);
                    for s in 0..n {
                        if triangulation.halfedges[s] == EMPTY {
                            num_unpaired += 1;
                            point_to_side[triangulation.triangles[s]] = s;
                            first_unpaired = s;
                        }
                    }
                    let total_capacity = n + 3 * num_unpaired;
                    triangulation.triangles.resize(total_capacity, 0);
                    triangulation.halfedges.resize(total_capacity, 0);
                    let mut s = first_unpaired;
                    for i in 0..num_unpaired {
                        let ns = n + 3 * i;
                        triangulation.halfedges[s] = ns;
                        triangulation.halfedges[ns] = s;
                        triangulation.triangles[ns] = triangulation.triangles[next(s)];
                        triangulation.triangles[ns + 1] = triangulation.triangles[s];
                        triangulation.triangles[ns + 2] = pole_id;
                        let k = n + (3 * i + 4) % (3 * num_unpaired);
                        triangulation.halfedges[ns + 2] = k;
                        triangulation.halfedges[k] = ns + 2;
                        s = point_to_side[triangulation.triangles[next(s)]];
                    }
                }
                let flat = stereographic_projection(&points);
                let mut result = triangulate(&flat);
                add_pole(points.len(), &mut result);
                points.push(Vec3::new(0.0, 0.0, 1.0));
                TriangulationResult {
                    triangles: result.triangles.iter().map(|t| *t as u32).collect(),
                    halfedges: result.halfedges.iter().map(|h| *h as u32).collect(),
                }
            }
        }
        impl Stage for CellGenerator {
            fn name(&self) -> &'static str {
                "Cell generator"
            }
            fn provides(&self) -> Fields {
                Fields::MESH | Fields::CELL_POSITION
            }
            fn run(&self, planet: &mut PlanetData) -> anyhow::Result<()> {
                let rng = self.make_rng();
                let mut points = self.generate_fibbonacy_points(rng);
                let triangulation = self.triangulate(&mut points);
                let mesh = Mesh::from_delaunator(
                        self.cell_count,
                        triangulation.triangles,
                        triangulation.halfedges,
                    )
                    .map_err(|e| ::anyhow::__private::must_use({
                        use ::anyhow::__private::kind::*;
                        let error = match e {
                            error => (&error).anyhow_kind().new(error),
                        };
                        error
                    }))?;
                planet.mesh = mesh;
                planet.cells.position = points;
                Ok(())
            }
        }
    }
}
#![feature(prelude_import)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
pub mod isostasy {}
pub mod surface {}
pub mod tectonics {
    pub mod generate_cells {
        use terrain_gen_core::stage::Stage;
        use terrain_gen_core::mesh::Mesh;
        use terrain_gen_core::data::{Fields, PlanetData};
        use terrain_gen_macros::godot_stage_export;
        use core;
        use rand::{Rng, SeedableRng};
        use rand::rngs::StdRng;
        use glam::Vec3;
        pub struct CellGenerator {
            seed: i64,
            cell_count: u32,
            jitter: f32,
        }
        #[class(base = Resource, init, tool, rename = CellGenerator)]
        struct __GodotCellGenerator {
            #[export]
            seed: i32,
            #[export]
            cell_count: u32,
            #[export]
            jitter: f32,
            base: ::godot::obj::Base<::godot::classes::Resource>,
        }
        impl ::godot::obj::GodotClass for __GodotCellGenerator {
            type Base = ::godot::classes::Resource;
            fn class_id() -> ::godot::meta::ClassId {
                use ::godot::meta::ClassId;
                static CLASS_ID: std::sync::OnceLock<ClassId> = std::sync::OnceLock::new();
                let id: &'static ClassId = CLASS_ID
                    .get_or_init(|| ClassId::__alloc_next_unicode("CellGenerator"));
                *id
            }
        }
        unsafe impl ::godot::obj::Bounds for __GodotCellGenerator {
            type Memory = <<Self as ::godot::obj::GodotClass>::Base as ::godot::obj::Bounds>::Memory;
            type DynMemory = <<Self as ::godot::obj::GodotClass>::Base as ::godot::obj::Bounds>::DynMemory;
            type Declarer = ::godot::obj::bounds::DeclUser;
            type Exportable = <<Self as ::godot::obj::GodotClass>::Base as ::godot::obj::Bounds>::Exportable;
        }
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        struct __godot___GodotCellGenerator_Funcs {}
        impl ::godot::obj::cap::GodotDefault for __GodotCellGenerator {
            fn __godot_user_init(
                base: ::godot::obj::Base<
                    <__GodotCellGenerator as ::godot::obj::GodotClass>::Base,
                >,
            ) -> Self {
                Self {
                    seed: ::std::default::Default::default(),
                    cell_count: ::std::default::Default::default(),
                    jitter: ::std::default::Default::default(),
                    base: base,
                }
            }
        }
        impl ::godot::obj::WithBaseField for __GodotCellGenerator {
            fn to_gd(&self) -> ::godot::obj::Gd<__GodotCellGenerator> {
                let base = <__GodotCellGenerator as ::godot::obj::WithBaseField>::base_field(
                    self,
                );
                base.__constructed_gd().cast()
            }
            fn base_field(
                &self,
            ) -> &::godot::obj::Base<
                <__GodotCellGenerator as ::godot::obj::GodotClass>::Base,
            > {
                &self.base
            }
        }
        impl __GodotCellGenerator {
            #[doc(hidden)]
            pub fn __godot_get_seed(&self) -> <i32 as ::godot::meta::GodotConvert>::Via {
                <i32 as ::godot::register::property::Var>::var_get(&self.seed)
            }
            #[deprecated = "Auto-generated Rust getters/setters for `#[var]` are being phased out until v0.6.\n\
                    If you need them, opt in with #[var(pub)]."]
            #[allow(dead_code)]
            pub fn get_seed(&self) -> <i32 as ::godot::meta::GodotConvert>::Via {
                self.__godot_get_seed()
            }
            #[doc(hidden)]
            pub fn __godot_set_seed(
                &mut self,
                seed: <i32 as ::godot::meta::GodotConvert>::Via,
            ) {
                <i32 as ::godot::register::property::Var>::var_set(&mut self.seed, seed)
            }
            #[deprecated = "Auto-generated Rust getters/setters for `#[var]` are being phased out until v0.6.\n\
                    If you need them, opt in with #[var(pub)]."]
            #[allow(dead_code)]
            pub fn set_seed(&mut self, seed: <i32 as ::godot::meta::GodotConvert>::Via) {
                self.__godot_set_seed(seed)
            }
            #[doc(hidden)]
            pub fn __godot_get_cell_count(
                &self,
            ) -> <u32 as ::godot::meta::GodotConvert>::Via {
                <u32 as ::godot::register::property::Var>::var_get(&self.cell_count)
            }
            #[deprecated = "Auto-generated Rust getters/setters for `#[var]` are being phased out until v0.6.\n\
                    If you need them, opt in with #[var(pub)]."]
            #[allow(dead_code)]
            pub fn get_cell_count(&self) -> <u32 as ::godot::meta::GodotConvert>::Via {
                self.__godot_get_cell_count()
            }
            #[doc(hidden)]
            pub fn __godot_set_cell_count(
                &mut self,
                cell_count: <u32 as ::godot::meta::GodotConvert>::Via,
            ) {
                <u32 as ::godot::register::property::Var>::var_set(
                    &mut self.cell_count,
                    cell_count,
                )
            }
            #[deprecated = "Auto-generated Rust getters/setters for `#[var]` are being phased out until v0.6.\n\
                    If you need them, opt in with #[var(pub)]."]
            #[allow(dead_code)]
            pub fn set_cell_count(
                &mut self,
                cell_count: <u32 as ::godot::meta::GodotConvert>::Via,
            ) {
                self.__godot_set_cell_count(cell_count)
            }
            #[doc(hidden)]
            pub fn __godot_get_jitter(
                &self,
            ) -> <f32 as ::godot::meta::GodotConvert>::Via {
                <f32 as ::godot::register::property::Var>::var_get(&self.jitter)
            }
            #[deprecated = "Auto-generated Rust getters/setters for `#[var]` are being phased out until v0.6.\n\
                    If you need them, opt in with #[var(pub)]."]
            #[allow(dead_code)]
            pub fn get_jitter(&self) -> <f32 as ::godot::meta::GodotConvert>::Via {
                self.__godot_get_jitter()
            }
            #[doc(hidden)]
            pub fn __godot_set_jitter(
                &mut self,
                jitter: <f32 as ::godot::meta::GodotConvert>::Via,
            ) {
                <f32 as ::godot::register::property::Var>::var_set(
                    &mut self.jitter,
                    jitter,
                )
            }
            #[deprecated = "Auto-generated Rust getters/setters for `#[var]` are being phased out until v0.6.\n\
                    If you need them, opt in with #[var(pub)]."]
            #[allow(dead_code)]
            pub fn set_jitter(
                &mut self,
                jitter: <f32 as ::godot::meta::GodotConvert>::Via,
            ) {
                self.__godot_set_jitter(jitter)
            }
        }
        impl __godot___GodotCellGenerator_Funcs {
            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            pub const __godot_get_seed: &str = "get_seed";
            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            pub const __godot_set_seed: &str = "set_seed";
            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            pub const __godot_get_cell_count: &str = "get_cell_count";
            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            pub const __godot_set_cell_count: &str = "set_cell_count";
            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            pub const __godot_get_jitter: &str = "get_jitter";
            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            pub const __godot_set_jitter: &str = "set_jitter";
        }
        impl ::godot::obj::cap::ImplementsGodotExports for __GodotCellGenerator {
            fn __register_exports() {
                {}
                {}
                {
                    {
                        use ::godot::obj::GodotClass;
                        use ::godot::register::private::method::ClassMethodInfo;
                        use ::godot::builtin::{StringName, Variant};
                        use ::godot::sys;
                        type CallParams = ();
                        type CallRet = <i32 as ::godot::meta::GodotConvert>::Via;
                        let method_name = StringName::from("get_seed");
                        unsafe extern "C" fn varcall_fn(
                            _method_data: *mut std::ffi::c_void,
                            instance_ptr: sys::GDExtensionClassInstancePtr,
                            args_ptr: *const sys::GDExtensionConstVariantPtr,
                            arg_count: sys::GDExtensionInt,
                            ret: sys::GDExtensionVariantPtr,
                            err: *mut sys::GDExtensionCallError,
                        ) {
                            let call_ctx = ::godot::private::CallContext::func(
                                "__GodotCellGenerator",
                                "get_seed",
                            );
                            ::godot::private::handle_fallible_varcall(
                                &call_ctx,
                                &mut *err,
                                || {
                                    let defaults = ::alloc::vec::Vec::new();
                                    ::godot::private::Signature::<
                                        CallParams,
                                        CallRet,
                                    >::in_varcall(
                                        instance_ptr,
                                        &call_ctx,
                                        args_ptr,
                                        arg_count,
                                        &defaults,
                                        ret,
                                        err,
                                        |instance_ptr, params| {
                                            let () = params;
                                            let storage = unsafe {
                                                ::godot::private::as_storage::<
                                                    __GodotCellGenerator,
                                                >(instance_ptr)
                                            };
                                            let __gdext_self = ::godot::private::Storage::get(storage);
                                            __gdext_self.__godot_get_seed()
                                        },
                                    )
                                },
                            );
                        }
                        unsafe extern "C" fn ptrcall_fn(
                            _method_data: *mut std::ffi::c_void,
                            instance_ptr: sys::GDExtensionClassInstancePtr,
                            args_ptr: *const sys::GDExtensionConstTypePtr,
                            ret: sys::GDExtensionTypePtr,
                        ) {
                            let call_ctx = ::godot::private::CallContext::func(
                                "__GodotCellGenerator",
                                "get_seed",
                            );
                            ::godot::private::handle_fallible_ptrcall(
                                &call_ctx,
                                || ::godot::private::Signature::<
                                    CallParams,
                                    CallRet,
                                >::in_ptrcall(
                                    instance_ptr,
                                    &call_ctx,
                                    args_ptr,
                                    ret,
                                    |instance_ptr, params| {
                                        let () = params;
                                        let storage = unsafe {
                                            ::godot::private::as_storage::<
                                                __GodotCellGenerator,
                                            >(instance_ptr)
                                        };
                                        let __gdext_self = ::godot::private::Storage::get(storage);
                                        __gdext_self.__godot_get_seed()
                                    },
                                    sys::PtrcallType::Standard,
                                ),
                            );
                        }
                        let method_info = unsafe {
                            ClassMethodInfo::from_signature::<
                                __GodotCellGenerator,
                                CallParams,
                                CallRet,
                            >(
                                method_name,
                                Some(varcall_fn),
                                Some(ptrcall_fn),
                                ::godot::register::info::MethodFlags::NORMAL
                                    | ::godot::register::info::MethodFlags::CONST,
                                &[],
                                ::alloc::vec::Vec::new(),
                            )
                        };
                        {
                            if false {
                                format_args!(
                                    "   Register fn:   {0}::{1}",
                                    "__GodotCellGenerator",
                                    "get_seed",
                                );
                            }
                        };
                        method_info.register_extension_class_method();
                    };
                }
                {
                    {
                        use ::godot::obj::GodotClass;
                        use ::godot::register::private::method::ClassMethodInfo;
                        use ::godot::builtin::{StringName, Variant};
                        use ::godot::sys;
                        type CallParams = (<i32 as ::godot::meta::GodotConvert>::Via,);
                        type CallRet = ();
                        let method_name = StringName::from("set_seed");
                        unsafe extern "C" fn varcall_fn(
                            _method_data: *mut std::ffi::c_void,
                            instance_ptr: sys::GDExtensionClassInstancePtr,
                            args_ptr: *const sys::GDExtensionConstVariantPtr,
                            arg_count: sys::GDExtensionInt,
                            ret: sys::GDExtensionVariantPtr,
                            err: *mut sys::GDExtensionCallError,
                        ) {
                            let call_ctx = ::godot::private::CallContext::func(
                                "__GodotCellGenerator",
                                "set_seed",
                            );
                            ::godot::private::handle_fallible_varcall(
                                &call_ctx,
                                &mut *err,
                                || {
                                    let defaults = ::alloc::vec::Vec::new();
                                    ::godot::private::Signature::<
                                        CallParams,
                                        CallRet,
                                    >::in_varcall(
                                        instance_ptr,
                                        &call_ctx,
                                        args_ptr,
                                        arg_count,
                                        &defaults,
                                        ret,
                                        err,
                                        |instance_ptr, params| {
                                            let (seed,) = params;
                                            let storage = unsafe {
                                                ::godot::private::as_storage::<
                                                    __GodotCellGenerator,
                                                >(instance_ptr)
                                            };
                                            let mut __gdext_self = ::godot::private::Storage::get_mut(
                                                storage,
                                            );
                                            __gdext_self.__godot_set_seed(seed)
                                        },
                                    )
                                },
                            );
                        }
                        unsafe extern "C" fn ptrcall_fn(
                            _method_data: *mut std::ffi::c_void,
                            instance_ptr: sys::GDExtensionClassInstancePtr,
                            args_ptr: *const sys::GDExtensionConstTypePtr,
                            ret: sys::GDExtensionTypePtr,
                        ) {
                            let call_ctx = ::godot::private::CallContext::func(
                                "__GodotCellGenerator",
                                "set_seed",
                            );
                            ::godot::private::handle_fallible_ptrcall(
                                &call_ctx,
                                || ::godot::private::Signature::<
                                    CallParams,
                                    CallRet,
                                >::in_ptrcall(
                                    instance_ptr,
                                    &call_ctx,
                                    args_ptr,
                                    ret,
                                    |instance_ptr, params| {
                                        let (seed,) = params;
                                        let storage = unsafe {
                                            ::godot::private::as_storage::<
                                                __GodotCellGenerator,
                                            >(instance_ptr)
                                        };
                                        let mut __gdext_self = ::godot::private::Storage::get_mut(
                                            storage,
                                        );
                                        __gdext_self.__godot_set_seed(seed)
                                    },
                                    sys::PtrcallType::Standard,
                                ),
                            );
                        }
                        let method_info = unsafe {
                            ClassMethodInfo::from_signature::<
                                __GodotCellGenerator,
                                CallParams,
                                CallRet,
                            >(
                                method_name,
                                Some(varcall_fn),
                                Some(ptrcall_fn),
                                ::godot::register::info::MethodFlags::NORMAL,
                                &["seed"],
                                ::alloc::vec::Vec::new(),
                            )
                        };
                        {
                            if false {
                                format_args!(
                                    "   Register fn:   {0}::{1}",
                                    "__GodotCellGenerator",
                                    "set_seed",
                                );
                            }
                        };
                        method_info.register_extension_class_method();
                    };
                }
                {
                    type FieldType = i32;
                    ::godot::register::private::register_export::<
                        __GodotCellGenerator,
                        FieldType,
                    >(
                        "seed",
                        __godot___GodotCellGenerator_Funcs::__godot_get_seed,
                        __godot___GodotCellGenerator_Funcs::__godot_set_seed,
                        None,
                        None,
                    );
                }
                {}
                {}
                {
                    {
                        use ::godot::obj::GodotClass;
                        use ::godot::register::private::method::ClassMethodInfo;
                        use ::godot::builtin::{StringName, Variant};
                        use ::godot::sys;
                        type CallParams = ();
                        type CallRet = <u32 as ::godot::meta::GodotConvert>::Via;
                        let method_name = StringName::from("get_cell_count");
                        unsafe extern "C" fn varcall_fn(
                            _method_data: *mut std::ffi::c_void,
                            instance_ptr: sys::GDExtensionClassInstancePtr,
                            args_ptr: *const sys::GDExtensionConstVariantPtr,
                            arg_count: sys::GDExtensionInt,
                            ret: sys::GDExtensionVariantPtr,
                            err: *mut sys::GDExtensionCallError,
                        ) {
                            let call_ctx = ::godot::private::CallContext::func(
                                "__GodotCellGenerator",
                                "get_cell_count",
                            );
                            ::godot::private::handle_fallible_varcall(
                                &call_ctx,
                                &mut *err,
                                || {
                                    let defaults = ::alloc::vec::Vec::new();
                                    ::godot::private::Signature::<
                                        CallParams,
                                        CallRet,
                                    >::in_varcall(
                                        instance_ptr,
                                        &call_ctx,
                                        args_ptr,
                                        arg_count,
                                        &defaults,
                                        ret,
                                        err,
                                        |instance_ptr, params| {
                                            let () = params;
                                            let storage = unsafe {
                                                ::godot::private::as_storage::<
                                                    __GodotCellGenerator,
                                                >(instance_ptr)
                                            };
                                            let __gdext_self = ::godot::private::Storage::get(storage);
                                            __gdext_self.__godot_get_cell_count()
                                        },
                                    )
                                },
                            );
                        }
                        unsafe extern "C" fn ptrcall_fn(
                            _method_data: *mut std::ffi::c_void,
                            instance_ptr: sys::GDExtensionClassInstancePtr,
                            args_ptr: *const sys::GDExtensionConstTypePtr,
                            ret: sys::GDExtensionTypePtr,
                        ) {
                            let call_ctx = ::godot::private::CallContext::func(
                                "__GodotCellGenerator",
                                "get_cell_count",
                            );
                            ::godot::private::handle_fallible_ptrcall(
                                &call_ctx,
                                || ::godot::private::Signature::<
                                    CallParams,
                                    CallRet,
                                >::in_ptrcall(
                                    instance_ptr,
                                    &call_ctx,
                                    args_ptr,
                                    ret,
                                    |instance_ptr, params| {
                                        let () = params;
                                        let storage = unsafe {
                                            ::godot::private::as_storage::<
                                                __GodotCellGenerator,
                                            >(instance_ptr)
                                        };
                                        let __gdext_self = ::godot::private::Storage::get(storage);
                                        __gdext_self.__godot_get_cell_count()
                                    },
                                    sys::PtrcallType::Standard,
                                ),
                            );
                        }
                        let method_info = unsafe {
                            ClassMethodInfo::from_signature::<
                                __GodotCellGenerator,
                                CallParams,
                                CallRet,
                            >(
                                method_name,
                                Some(varcall_fn),
                                Some(ptrcall_fn),
                                ::godot::register::info::MethodFlags::NORMAL
                                    | ::godot::register::info::MethodFlags::CONST,
                                &[],
                                ::alloc::vec::Vec::new(),
                            )
                        };
                        {
                            if false {
                                format_args!(
                                    "   Register fn:   {0}::{1}",
                                    "__GodotCellGenerator",
                                    "get_cell_count",
                                );
                            }
                        };
                        method_info.register_extension_class_method();
                    };
                }
                {
                    {
                        use ::godot::obj::GodotClass;
                        use ::godot::register::private::method::ClassMethodInfo;
                        use ::godot::builtin::{StringName, Variant};
                        use ::godot::sys;
                        type CallParams = (<u32 as ::godot::meta::GodotConvert>::Via,);
                        type CallRet = ();
                        let method_name = StringName::from("set_cell_count");
                        unsafe extern "C" fn varcall_fn(
                            _method_data: *mut std::ffi::c_void,
                            instance_ptr: sys::GDExtensionClassInstancePtr,
                            args_ptr: *const sys::GDExtensionConstVariantPtr,
                            arg_count: sys::GDExtensionInt,
                            ret: sys::GDExtensionVariantPtr,
                            err: *mut sys::GDExtensionCallError,
                        ) {
                            let call_ctx = ::godot::private::CallContext::func(
                                "__GodotCellGenerator",
                                "set_cell_count",
                            );
                            ::godot::private::handle_fallible_varcall(
                                &call_ctx,
                                &mut *err,
                                || {
                                    let defaults = ::alloc::vec::Vec::new();
                                    ::godot::private::Signature::<
                                        CallParams,
                                        CallRet,
                                    >::in_varcall(
                                        instance_ptr,
                                        &call_ctx,
                                        args_ptr,
                                        arg_count,
                                        &defaults,
                                        ret,
                                        err,
                                        |instance_ptr, params| {
                                            let (cell_count,) = params;
                                            let storage = unsafe {
                                                ::godot::private::as_storage::<
                                                    __GodotCellGenerator,
                                                >(instance_ptr)
                                            };
                                            let mut __gdext_self = ::godot::private::Storage::get_mut(
                                                storage,
                                            );
                                            __gdext_self.__godot_set_cell_count(cell_count)
                                        },
                                    )
                                },
                            );
                        }
                        unsafe extern "C" fn ptrcall_fn(
                            _method_data: *mut std::ffi::c_void,
                            instance_ptr: sys::GDExtensionClassInstancePtr,
                            args_ptr: *const sys::GDExtensionConstTypePtr,
                            ret: sys::GDExtensionTypePtr,
                        ) {
                            let call_ctx = ::godot::private::CallContext::func(
                                "__GodotCellGenerator",
                                "set_cell_count",
                            );
                            ::godot::private::handle_fallible_ptrcall(
                                &call_ctx,
                                || ::godot::private::Signature::<
                                    CallParams,
                                    CallRet,
                                >::in_ptrcall(
                                    instance_ptr,
                                    &call_ctx,
                                    args_ptr,
                                    ret,
                                    |instance_ptr, params| {
                                        let (cell_count,) = params;
                                        let storage = unsafe {
                                            ::godot::private::as_storage::<
                                                __GodotCellGenerator,
                                            >(instance_ptr)
                                        };
                                        let mut __gdext_self = ::godot::private::Storage::get_mut(
                                            storage,
                                        );
                                        __gdext_self.__godot_set_cell_count(cell_count)
                                    },
                                    sys::PtrcallType::Standard,
                                ),
                            );
                        }
                        let method_info = unsafe {
                            ClassMethodInfo::from_signature::<
                                __GodotCellGenerator,
                                CallParams,
                                CallRet,
                            >(
                                method_name,
                                Some(varcall_fn),
                                Some(ptrcall_fn),
                                ::godot::register::info::MethodFlags::NORMAL,
                                &["cell_count"],
                                ::alloc::vec::Vec::new(),
                            )
                        };
                        {
                            if false {
                                format_args!(
                                    "   Register fn:   {0}::{1}",
                                    "__GodotCellGenerator",
                                    "set_cell_count",
                                );
                            }
                        };
                        method_info.register_extension_class_method();
                    };
                }
                {
                    type FieldType = u32;
                    ::godot::register::private::register_export::<
                        __GodotCellGenerator,
                        FieldType,
                    >(
                        "cell_count",
                        __godot___GodotCellGenerator_Funcs::__godot_get_cell_count,
                        __godot___GodotCellGenerator_Funcs::__godot_set_cell_count,
                        None,
                        None,
                    );
                }
                {}
                {}
                {
                    {
                        use ::godot::obj::GodotClass;
                        use ::godot::register::private::method::ClassMethodInfo;
                        use ::godot::builtin::{StringName, Variant};
                        use ::godot::sys;
                        type CallParams = ();
                        type CallRet = <f32 as ::godot::meta::GodotConvert>::Via;
                        let method_name = StringName::from("get_jitter");
                        unsafe extern "C" fn varcall_fn(
                            _method_data: *mut std::ffi::c_void,
                            instance_ptr: sys::GDExtensionClassInstancePtr,
                            args_ptr: *const sys::GDExtensionConstVariantPtr,
                            arg_count: sys::GDExtensionInt,
                            ret: sys::GDExtensionVariantPtr,
                            err: *mut sys::GDExtensionCallError,
                        ) {
                            let call_ctx = ::godot::private::CallContext::func(
                                "__GodotCellGenerator",
                                "get_jitter",
                            );
                            ::godot::private::handle_fallible_varcall(
                                &call_ctx,
                                &mut *err,
                                || {
                                    let defaults = ::alloc::vec::Vec::new();
                                    ::godot::private::Signature::<
                                        CallParams,
                                        CallRet,
                                    >::in_varcall(
                                        instance_ptr,
                                        &call_ctx,
                                        args_ptr,
                                        arg_count,
                                        &defaults,
                                        ret,
                                        err,
                                        |instance_ptr, params| {
                                            let () = params;
                                            let storage = unsafe {
                                                ::godot::private::as_storage::<
                                                    __GodotCellGenerator,
                                                >(instance_ptr)
                                            };
                                            let __gdext_self = ::godot::private::Storage::get(storage);
                                            __gdext_self.__godot_get_jitter()
                                        },
                                    )
                                },
                            );
                        }
                        unsafe extern "C" fn ptrcall_fn(
                            _method_data: *mut std::ffi::c_void,
                            instance_ptr: sys::GDExtensionClassInstancePtr,
                            args_ptr: *const sys::GDExtensionConstTypePtr,
                            ret: sys::GDExtensionTypePtr,
                        ) {
                            let call_ctx = ::godot::private::CallContext::func(
                                "__GodotCellGenerator",
                                "get_jitter",
                            );
                            ::godot::private::handle_fallible_ptrcall(
                                &call_ctx,
                                || ::godot::private::Signature::<
                                    CallParams,
                                    CallRet,
                                >::in_ptrcall(
                                    instance_ptr,
                                    &call_ctx,
                                    args_ptr,
                                    ret,
                                    |instance_ptr, params| {
                                        let () = params;
                                        let storage = unsafe {
                                            ::godot::private::as_storage::<
                                                __GodotCellGenerator,
                                            >(instance_ptr)
                                        };
                                        let __gdext_self = ::godot::private::Storage::get(storage);
                                        __gdext_self.__godot_get_jitter()
                                    },
                                    sys::PtrcallType::Standard,
                                ),
                            );
                        }
                        let method_info = unsafe {
                            ClassMethodInfo::from_signature::<
                                __GodotCellGenerator,
                                CallParams,
                                CallRet,
                            >(
                                method_name,
                                Some(varcall_fn),
                                Some(ptrcall_fn),
                                ::godot::register::info::MethodFlags::NORMAL
                                    | ::godot::register::info::MethodFlags::CONST,
                                &[],
                                ::alloc::vec::Vec::new(),
                            )
                        };
                        {
                            if false {
                                format_args!(
                                    "   Register fn:   {0}::{1}",
                                    "__GodotCellGenerator",
                                    "get_jitter",
                                );
                            }
                        };
                        method_info.register_extension_class_method();
                    };
                }
                {
                    {
                        use ::godot::obj::GodotClass;
                        use ::godot::register::private::method::ClassMethodInfo;
                        use ::godot::builtin::{StringName, Variant};
                        use ::godot::sys;
                        type CallParams = (<f32 as ::godot::meta::GodotConvert>::Via,);
                        type CallRet = ();
                        let method_name = StringName::from("set_jitter");
                        unsafe extern "C" fn varcall_fn(
                            _method_data: *mut std::ffi::c_void,
                            instance_ptr: sys::GDExtensionClassInstancePtr,
                            args_ptr: *const sys::GDExtensionConstVariantPtr,
                            arg_count: sys::GDExtensionInt,
                            ret: sys::GDExtensionVariantPtr,
                            err: *mut sys::GDExtensionCallError,
                        ) {
                            let call_ctx = ::godot::private::CallContext::func(
                                "__GodotCellGenerator",
                                "set_jitter",
                            );
                            ::godot::private::handle_fallible_varcall(
                                &call_ctx,
                                &mut *err,
                                || {
                                    let defaults = ::alloc::vec::Vec::new();
                                    ::godot::private::Signature::<
                                        CallParams,
                                        CallRet,
                                    >::in_varcall(
                                        instance_ptr,
                                        &call_ctx,
                                        args_ptr,
                                        arg_count,
                                        &defaults,
                                        ret,
                                        err,
                                        |instance_ptr, params| {
                                            let (jitter,) = params;
                                            let storage = unsafe {
                                                ::godot::private::as_storage::<
                                                    __GodotCellGenerator,
                                                >(instance_ptr)
                                            };
                                            let mut __gdext_self = ::godot::private::Storage::get_mut(
                                                storage,
                                            );
                                            __gdext_self.__godot_set_jitter(jitter)
                                        },
                                    )
                                },
                            );
                        }
                        unsafe extern "C" fn ptrcall_fn(
                            _method_data: *mut std::ffi::c_void,
                            instance_ptr: sys::GDExtensionClassInstancePtr,
                            args_ptr: *const sys::GDExtensionConstTypePtr,
                            ret: sys::GDExtensionTypePtr,
                        ) {
                            let call_ctx = ::godot::private::CallContext::func(
                                "__GodotCellGenerator",
                                "set_jitter",
                            );
                            ::godot::private::handle_fallible_ptrcall(
                                &call_ctx,
                                || ::godot::private::Signature::<
                                    CallParams,
                                    CallRet,
                                >::in_ptrcall(
                                    instance_ptr,
                                    &call_ctx,
                                    args_ptr,
                                    ret,
                                    |instance_ptr, params| {
                                        let (jitter,) = params;
                                        let storage = unsafe {
                                            ::godot::private::as_storage::<
                                                __GodotCellGenerator,
                                            >(instance_ptr)
                                        };
                                        let mut __gdext_self = ::godot::private::Storage::get_mut(
                                            storage,
                                        );
                                        __gdext_self.__godot_set_jitter(jitter)
                                    },
                                    sys::PtrcallType::Standard,
                                ),
                            );
                        }
                        let method_info = unsafe {
                            ClassMethodInfo::from_signature::<
                                __GodotCellGenerator,
                                CallParams,
                                CallRet,
                            >(
                                method_name,
                                Some(varcall_fn),
                                Some(ptrcall_fn),
                                ::godot::register::info::MethodFlags::NORMAL,
                                &["jitter"],
                                ::alloc::vec::Vec::new(),
                            )
                        };
                        {
                            if false {
                                format_args!(
                                    "   Register fn:   {0}::{1}",
                                    "__GodotCellGenerator",
                                    "set_jitter",
                                );
                            }
                        };
                        method_info.register_extension_class_method();
                    };
                }
                {
                    type FieldType = f32;
                    ::godot::register::private::register_export::<
                        __GodotCellGenerator,
                        FieldType,
                    >(
                        "jitter",
                        __godot___GodotCellGenerator_Funcs::__godot_get_jitter,
                        __godot___GodotCellGenerator_Funcs::__godot_set_jitter,
                        None,
                        None,
                    );
                }
            }
        }
        impl ::godot::obj::UserClass for __GodotCellGenerator {
            #[doc(hidden)]
            fn __config() -> ::godot::private::ClassConfig {
                ::godot::private::ClassConfig {
                    is_tool: true,
                }
            }
            #[doc(hidden)]
            fn __before_ready(&mut self) {
                ::godot::register::private::auto_register_rpcs::<
                    __GodotCellGenerator,
                >(self);
            }
        }
        const _: () = {
            #[allow(non_upper_case_globals)]
            #[used]
            #[unsafe(link_section = ".init_array")]
            static __init: extern "C" fn() = {
                #[unsafe(link_section = ".text.startup")]
                extern "C" fn __inner_init() {
                    {
                        ::godot::private::__GODOT_SHARD_REGISTRY
                            .lock()
                            .unwrap()
                            .push(
                                ::godot::private::ClassShard::new::<
                                    __GodotCellGenerator,
                                >(
                                    ::godot::private::ShardItem::Struct(
                                        ::godot::private::Struct::new::<__GodotCellGenerator>()
                                            .with_generated::<__GodotCellGenerator>()
                                            .with_instantiable()
                                            .with_tool(),
                                    ),
                                ),
                            );
                    }
                }
                __inner_init
            };
        };
        unsafe impl ::godot::obj::Inherits<::godot::classes::Resource>
        for __GodotCellGenerator {}
        unsafe impl ::godot::obj::Inherits<::godot::classes::RefCounted>
        for __GodotCellGenerator {}
        unsafe impl ::godot::obj::Inherits<::godot::classes::Object>
        for __GodotCellGenerator {}
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    terrain_gen_core::stage::StageConverter(|res| {
                        res.try_cast::<__GodotCellGenerator>()
                            .ok()
                            .map(|g| {
                                let bound = g.bind();
                                let plain = CellGenerator {
                                    seed: bound.seed.clone().into(),
                                    cell_count: bound.cell_count.clone().into(),
                                    jitter: bound.jitter.clone().into(),
                                };
                                Box::new(plain) as Box<dyn terrain_gen_core::stage::Stage>
                            })
                    })
                },
                next: ::inventory::__private::UnsafeCell::new(
                    ::inventory::__private::Option::None,
                ),
            };
            #[link_section = ".text.startup"]
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = ".init_array"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
        struct TriangulationResult {
            pub triangles: Vec<u32>,
            pub halfedges: Vec<u32>,
        }
        impl CellGenerator {
            fn make_rng(&self) -> Box<dyn Rng> {
                Box::new(StdRng::seed_from_u64(self.seed as u64))
            }
            fn generate_fibbonacy_points(&self, mut rng: Box<dyn Rng>) -> Vec<Vec3> {
                let n = self.cell_count as usize;
                let jitter = self.jitter as f64;
                if n == 1 {
                    return ::alloc::boxed::box_assume_init_into_vec_unsafe(
                        ::alloc::intrinsics::write_box_via_move(
                            ::alloc::boxed::Box::new_uninit(),
                            [Vec3::new(0.0, 1.0, 0.0)],
                        ),
                    );
                }
                let mut points: Vec<Vec3> = Vec::with_capacity(n);
                const PI: f64 = core::f64::consts::PI;
                let s = 3.6 / (n as f64).sqrt();
                let dlong = PI * (3. - f64::sqrt(5.));
                let dz = 2.0 / (n as f64);
                let mut long: f64 = 0.0;
                let mut z = 1. - dz / 2.;
                for _ in 0..n {
                    let r = (1. - z * z).sqrt();
                    let mut lat = z.asin();
                    let mut lon = long;
                    let rand_lat = rng.next_u32() as f64 / u32::MAX as f64;
                    let rand_long = rng.next_u32() as f64 / u32::MAX as f64;
                    lat
                        += jitter * rand_lat
                            * (lat - f64::asin(f64::max(-1., z - dz * 2. * PI * r / s)));
                    lon += jitter * rand_long * s / r;
                    points
                        .push(Vec3 {
                            x: (lat.cos() * lon.cos()) as f32,
                            y: (lat.cos() * lon.sin()) as f32,
                            z: (lat.sin()) as f32,
                        });
                    z -= dz;
                    long += dlong;
                }
                points
            }
            fn triangulate(&self, points: &mut Vec<Vec3>) -> TriangulationResult {
                use delaunator::{EMPTY, Point, Triangulation, triangulate};
                fn stereographic_projection(points: &Vec<Vec3>) -> Vec<Point> {
                    points
                        .iter()
                        .map(|point| {
                            Point {
                                x: (point.x / (1. - point.z)) as f64,
                                y: (point.y / (1. - point.z)) as f64,
                            }
                        })
                        .collect()
                }
                fn add_pole(pole_id: usize, triangulation: &mut Triangulation) {
                    let n = triangulation.triangles.len();
                    fn next(i: usize) -> usize {
                        if (i % 3) == 2 { i - 2 } else { i + 1 }
                    }
                    let mut num_unpaired: usize = 0;
                    let mut first_unpaired: usize = 0;
                    let mut point_to_side = ::alloc::vec::from_elem(0, n);
                    for s in 0..n {
                        if triangulation.halfedges[s] == EMPTY {
                            num_unpaired += 1;
                            point_to_side[triangulation.triangles[s]] = s;
                            first_unpaired = s;
                        }
                    }
                    let total_capacity = n + 3 * num_unpaired;
                    triangulation.triangles.resize(total_capacity, 0);
                    triangulation.halfedges.resize(total_capacity, 0);
                    let mut s = first_unpaired;
                    for i in 0..num_unpaired {
                        let ns = n + 3 * i;
                        triangulation.halfedges[s] = ns;
                        triangulation.halfedges[ns] = s;
                        triangulation.triangles[ns] = triangulation.triangles[next(s)];
                        triangulation.triangles[ns + 1] = triangulation.triangles[s];
                        triangulation.triangles[ns + 2] = pole_id;
                        let k = n + (3 * i + 4) % (3 * num_unpaired);
                        triangulation.halfedges[ns + 2] = k;
                        triangulation.halfedges[k] = ns + 2;
                        s = point_to_side[triangulation.triangles[next(s)]];
                    }
                }
                let flat = stereographic_projection(&points);
                let mut result = triangulate(&flat);
                add_pole(points.len(), &mut result);
                points.push(Vec3::new(0.0, 0.0, 1.0));
                TriangulationResult {
                    triangles: result.triangles.iter().map(|t| *t as u32).collect(),
                    halfedges: result.halfedges.iter().map(|h| *h as u32).collect(),
                }
            }
        }
        impl Stage for CellGenerator {
            fn name(&self) -> &'static str {
                "Cell generator"
            }
            fn provides(&self) -> Fields {
                Fields::MESH | Fields::CELL_POSITION
            }
            fn run(&self, planet: &mut PlanetData) -> anyhow::Result<()> {
                let rng = self.make_rng();
                let mut points = self.generate_fibbonacy_points(rng);
                let triangulation = self.triangulate(&mut points);
                let mesh = Mesh::from_delaunator(
                        self.cell_count,
                        triangulation.triangles,
                        triangulation.halfedges,
                    )
                    .map_err(|e| ::anyhow::__private::must_use({
                        use ::anyhow::__private::kind::*;
                        let error = match e {
                            error => (&error).anyhow_kind().new(error),
                        };
                        error
                    }))?;
                planet.mesh = mesh;
                planet.cells.position = points;
                Ok(())
            }
        }
    }
}
