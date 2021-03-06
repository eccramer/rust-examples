// This file is generated by rust-protobuf 2.4.0. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct OrderForm {
    // message fields
    pub quantity: i32,
    pub product: OilProductType,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl OrderForm {
    pub fn new() -> OrderForm {
        ::std::default::Default::default()
    }

    // int32 quantity = 1;

    pub fn clear_quantity(&mut self) {
        self.quantity = 0;
    }

    // Param is passed by value, moved
    pub fn set_quantity(&mut self, v: i32) {
        self.quantity = v;
    }

    pub fn get_quantity(&self) -> i32 {
        self.quantity
    }

    // .refinery.OilProductType product = 2;

    pub fn clear_product(&mut self) {
        self.product = OilProductType::GASOLINE;
    }

    // Param is passed by value, moved
    pub fn set_product(&mut self, v: OilProductType) {
        self.product = v;
    }

    pub fn get_product(&self) -> OilProductType {
        self.product
    }
}

impl ::protobuf::Message for OrderForm {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.quantity = tmp;
                },
                2 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.product, 2, &mut self.unknown_fields)?
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.quantity != 0 {
            my_size += ::protobuf::rt::value_size(1, self.quantity, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.product != OilProductType::GASOLINE {
            my_size += ::protobuf::rt::enum_size(2, self.product);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.quantity != 0 {
            os.write_int32(1, self.quantity)?;
        }
        if self.product != OilProductType::GASOLINE {
            os.write_enum(2, self.product.value())?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> OrderForm {
        OrderForm::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "quantity",
                    |m: &OrderForm| { &m.quantity },
                    |m: &mut OrderForm| { &mut m.quantity },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<OilProductType>>(
                    "product",
                    |m: &OrderForm| { &m.product },
                    |m: &mut OrderForm| { &mut m.product },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OrderForm>(
                    "OrderForm",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static OrderForm {
        static mut instance: ::protobuf::lazy::Lazy<OrderForm> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OrderForm,
        };
        unsafe {
            instance.get(OrderForm::new)
        }
    }
}

impl ::protobuf::Clear for OrderForm {
    fn clear(&mut self) {
        self.clear_quantity();
        self.clear_product();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OrderForm {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OrderForm {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OrderStatus {
    // message fields
    pub status: OrderResponseType,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl OrderStatus {
    pub fn new() -> OrderStatus {
        ::std::default::Default::default()
    }

    // .refinery.OrderResponseType status = 1;

    pub fn clear_status(&mut self) {
        self.status = OrderResponseType::UNKNOWN;
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: OrderResponseType) {
        self.status = v;
    }

    pub fn get_status(&self) -> OrderResponseType {
        self.status
    }
}

impl ::protobuf::Message for OrderStatus {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.status, 1, &mut self.unknown_fields)?
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.status != OrderResponseType::UNKNOWN {
            my_size += ::protobuf::rt::enum_size(1, self.status);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.status != OrderResponseType::UNKNOWN {
            os.write_enum(1, self.status.value())?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> OrderStatus {
        OrderStatus::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<OrderResponseType>>(
                    "status",
                    |m: &OrderStatus| { &m.status },
                    |m: &mut OrderStatus| { &mut m.status },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OrderStatus>(
                    "OrderStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static OrderStatus {
        static mut instance: ::protobuf::lazy::Lazy<OrderStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OrderStatus,
        };
        unsafe {
            instance.get(OrderStatus::new)
        }
    }
}

impl ::protobuf::Clear for OrderStatus {
    fn clear(&mut self) {
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OrderStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OrderStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OrderRecord {
    // message fields
    pub id: i32,
    pub quantity: i32,
    pub product: OilProductType,
    pub received_time: ::protobuf::SingularPtrField<::protobuf::well_known_types::Timestamp>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl OrderRecord {
    pub fn new() -> OrderRecord {
        ::std::default::Default::default()
    }

    // int32 id = 1;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i32) {
        self.id = v;
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    // int32 quantity = 2;

    pub fn clear_quantity(&mut self) {
        self.quantity = 0;
    }

    // Param is passed by value, moved
    pub fn set_quantity(&mut self, v: i32) {
        self.quantity = v;
    }

    pub fn get_quantity(&self) -> i32 {
        self.quantity
    }

    // .refinery.OilProductType product = 3;

    pub fn clear_product(&mut self) {
        self.product = OilProductType::GASOLINE;
    }

    // Param is passed by value, moved
    pub fn set_product(&mut self, v: OilProductType) {
        self.product = v;
    }

    pub fn get_product(&self) -> OilProductType {
        self.product
    }

    // .google.protobuf.Timestamp received_time = 4;

    pub fn clear_received_time(&mut self) {
        self.received_time.clear();
    }

    pub fn has_received_time(&self) -> bool {
        self.received_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_received_time(&mut self, v: ::protobuf::well_known_types::Timestamp) {
        self.received_time = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_received_time(&mut self) -> &mut ::protobuf::well_known_types::Timestamp {
        if self.received_time.is_none() {
            self.received_time.set_default();
        }
        self.received_time.as_mut().unwrap()
    }

    // Take field
    pub fn take_received_time(&mut self) -> ::protobuf::well_known_types::Timestamp {
        self.received_time.take().unwrap_or_else(|| ::protobuf::well_known_types::Timestamp::new())
    }

    pub fn get_received_time(&self) -> &::protobuf::well_known_types::Timestamp {
        self.received_time.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Timestamp::default_instance())
    }
}

impl ::protobuf::Message for OrderRecord {
    fn is_initialized(&self) -> bool {
        for v in &self.received_time {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.id = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.quantity = tmp;
                },
                3 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.product, 3, &mut self.unknown_fields)?
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.received_time)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.quantity != 0 {
            my_size += ::protobuf::rt::value_size(2, self.quantity, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.product != OilProductType::GASOLINE {
            my_size += ::protobuf::rt::enum_size(3, self.product);
        }
        if let Some(ref v) = self.received_time.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.id != 0 {
            os.write_int32(1, self.id)?;
        }
        if self.quantity != 0 {
            os.write_int32(2, self.quantity)?;
        }
        if self.product != OilProductType::GASOLINE {
            os.write_enum(3, self.product.value())?;
        }
        if let Some(ref v) = self.received_time.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> OrderRecord {
        OrderRecord::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "id",
                    |m: &OrderRecord| { &m.id },
                    |m: &mut OrderRecord| { &mut m.id },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "quantity",
                    |m: &OrderRecord| { &m.quantity },
                    |m: &mut OrderRecord| { &mut m.quantity },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<OilProductType>>(
                    "product",
                    |m: &OrderRecord| { &m.product },
                    |m: &mut OrderRecord| { &mut m.product },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Timestamp>>(
                    "received_time",
                    |m: &OrderRecord| { &m.received_time },
                    |m: &mut OrderRecord| { &mut m.received_time },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OrderRecord>(
                    "OrderRecord",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static OrderRecord {
        static mut instance: ::protobuf::lazy::Lazy<OrderRecord> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OrderRecord,
        };
        unsafe {
            instance.get(OrderRecord::new)
        }
    }
}

impl ::protobuf::Clear for OrderRecord {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_quantity();
        self.clear_product();
        self.clear_received_time();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OrderRecord {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OrderRecord {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OrderRecordList {
    // message fields
    pub order: ::protobuf::RepeatedField<OrderRecord>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl OrderRecordList {
    pub fn new() -> OrderRecordList {
        ::std::default::Default::default()
    }

    // repeated .refinery.OrderRecord order = 1;

    pub fn clear_order(&mut self) {
        self.order.clear();
    }

    // Param is passed by value, moved
    pub fn set_order(&mut self, v: ::protobuf::RepeatedField<OrderRecord>) {
        self.order = v;
    }

    // Mutable pointer to the field.
    pub fn mut_order(&mut self) -> &mut ::protobuf::RepeatedField<OrderRecord> {
        &mut self.order
    }

    // Take field
    pub fn take_order(&mut self) -> ::protobuf::RepeatedField<OrderRecord> {
        ::std::mem::replace(&mut self.order, ::protobuf::RepeatedField::new())
    }

    pub fn get_order(&self) -> &[OrderRecord] {
        &self.order
    }
}

impl ::protobuf::Message for OrderRecordList {
    fn is_initialized(&self) -> bool {
        for v in &self.order {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.order)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.order {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.order {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> OrderRecordList {
        OrderRecordList::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OrderRecord>>(
                    "order",
                    |m: &OrderRecordList| { &m.order },
                    |m: &mut OrderRecordList| { &mut m.order },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OrderRecordList>(
                    "OrderRecordList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static OrderRecordList {
        static mut instance: ::protobuf::lazy::Lazy<OrderRecordList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OrderRecordList,
        };
        unsafe {
            instance.get(OrderRecordList::new)
        }
    }
}

impl ::protobuf::Clear for OrderRecordList {
    fn clear(&mut self) {
        self.clear_order();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OrderRecordList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OrderRecordList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum OilProductType {
    GASOLINE = 0,
    JETFUEL = 1,
    DIESEL = 2,
    ASPHALT = 3,
    HEAVY = 4,
    LUBRICANT = 5,
    OTHER = 6,
}

impl ::protobuf::ProtobufEnum for OilProductType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<OilProductType> {
        match value {
            0 => ::std::option::Option::Some(OilProductType::GASOLINE),
            1 => ::std::option::Option::Some(OilProductType::JETFUEL),
            2 => ::std::option::Option::Some(OilProductType::DIESEL),
            3 => ::std::option::Option::Some(OilProductType::ASPHALT),
            4 => ::std::option::Option::Some(OilProductType::HEAVY),
            5 => ::std::option::Option::Some(OilProductType::LUBRICANT),
            6 => ::std::option::Option::Some(OilProductType::OTHER),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [OilProductType] = &[
            OilProductType::GASOLINE,
            OilProductType::JETFUEL,
            OilProductType::DIESEL,
            OilProductType::ASPHALT,
            OilProductType::HEAVY,
            OilProductType::LUBRICANT,
            OilProductType::OTHER,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("OilProductType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for OilProductType {
}

impl ::std::default::Default for OilProductType {
    fn default() -> Self {
        OilProductType::GASOLINE
    }
}

impl ::protobuf::reflect::ProtobufValue for OilProductType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum OrderResponseType {
    UNKNOWN = 0,
    RECEIVED = 1,
    REJECTED = 2,
}

impl ::protobuf::ProtobufEnum for OrderResponseType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<OrderResponseType> {
        match value {
            0 => ::std::option::Option::Some(OrderResponseType::UNKNOWN),
            1 => ::std::option::Option::Some(OrderResponseType::RECEIVED),
            2 => ::std::option::Option::Some(OrderResponseType::REJECTED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [OrderResponseType] = &[
            OrderResponseType::UNKNOWN,
            OrderResponseType::RECEIVED,
            OrderResponseType::REJECTED,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("OrderResponseType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for OrderResponseType {
}

impl ::std::default::Default for OrderResponseType {
    fn default() -> Self {
        OrderResponseType::UNKNOWN
    }
}

impl ::protobuf::reflect::ProtobufValue for OrderResponseType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0erefinery.proto\x12\x08refinery\x1a\x1fgoogle/protobuf/timestamp.pr\
    oto\x1a\x1bgoogle/protobuf/empty.proto\"[\n\tOrderForm\x12\x1a\n\x08quan\
    tity\x18\x01\x20\x01(\x05R\x08quantity\x122\n\x07product\x18\x02\x20\x01\
    (\x0e2\x18.refinery.OilProductTypeR\x07product\"B\n\x0bOrderStatus\x123\
    \n\x06status\x18\x01\x20\x01(\x0e2\x1b.refinery.OrderResponseTypeR\x06st\
    atus\"\xae\x01\n\x0bOrderRecord\x12\x0e\n\x02id\x18\x01\x20\x01(\x05R\
    \x02id\x12\x1a\n\x08quantity\x18\x02\x20\x01(\x05R\x08quantity\x122\n\
    \x07product\x18\x03\x20\x01(\x0e2\x18.refinery.OilProductTypeR\x07produc\
    t\x12?\n\rreceived_time\x18\x04\x20\x01(\x0b2\x1a.google.protobuf.Timest\
    ampR\x0creceivedTime\">\n\x0fOrderRecordList\x12+\n\x05order\x18\x01\x20\
    \x03(\x0b2\x15.refinery.OrderRecordR\x05order*i\n\x0eOilProductType\x12\
    \x0c\n\x08GASOLINE\x10\0\x12\x0b\n\x07JETFUEL\x10\x01\x12\n\n\x06DIESEL\
    \x10\x02\x12\x0b\n\x07ASPHALT\x10\x03\x12\t\n\x05HEAVY\x10\x04\x12\r\n\t\
    LUBRICANT\x10\x05\x12\t\n\x05OTHER\x10\x06*<\n\x11OrderResponseType\x12\
    \x0b\n\x07UNKNOWN\x10\0\x12\x0c\n\x08RECEIVED\x10\x01\x12\x0c\n\x08REJEC\
    TED\x10\x022\x83\x01\n\x08Refinery\x123\n\x05Order\x12\x13.refinery.Orde\
    rForm\x1a\x15.refinery.OrderStatus\x12B\n\rGetAllRecords\x12\x16.google.\
    protobuf.Empty\x1a\x19.refinery.OrderRecordListb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
