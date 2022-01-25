// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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

const METHOD_LOGGING_SERVICE_V2_DELETE_LOG: ::grpcio::Method<super::logging::DeleteLogRequest, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.logging.v2.LoggingServiceV2/DeleteLog",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LOGGING_SERVICE_V2_WRITE_LOG_ENTRIES: ::grpcio::Method<super::logging::WriteLogEntriesRequest, super::logging::WriteLogEntriesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.logging.v2.LoggingServiceV2/WriteLogEntries",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LOGGING_SERVICE_V2_LIST_LOG_ENTRIES: ::grpcio::Method<super::logging::ListLogEntriesRequest, super::logging::ListLogEntriesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.logging.v2.LoggingServiceV2/ListLogEntries",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LOGGING_SERVICE_V2_LIST_MONITORED_RESOURCE_DESCRIPTORS: ::grpcio::Method<super::logging::ListMonitoredResourceDescriptorsRequest, super::logging::ListMonitoredResourceDescriptorsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.logging.v2.LoggingServiceV2/ListMonitoredResourceDescriptors",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LOGGING_SERVICE_V2_LIST_LOGS: ::grpcio::Method<super::logging::ListLogsRequest, super::logging::ListLogsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.logging.v2.LoggingServiceV2/ListLogs",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct LoggingServiceV2Client {
    client: ::grpcio::Client,
}

impl LoggingServiceV2Client {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        LoggingServiceV2Client {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn delete_log_opt(&self, req: &super::logging::DeleteLogRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_LOGGING_SERVICE_V2_DELETE_LOG, req, opt)
    }

    pub fn delete_log(&self, req: &super::logging::DeleteLogRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.delete_log_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_log_async_opt(&self, req: &super::logging::DeleteLogRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_LOGGING_SERVICE_V2_DELETE_LOG, req, opt)
    }

    pub fn delete_log_async(&self, req: &super::logging::DeleteLogRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.delete_log_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn write_log_entries_opt(&self, req: &super::logging::WriteLogEntriesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::logging::WriteLogEntriesResponse> {
        self.client.unary_call(&METHOD_LOGGING_SERVICE_V2_WRITE_LOG_ENTRIES, req, opt)
    }

    pub fn write_log_entries(&self, req: &super::logging::WriteLogEntriesRequest) -> ::grpcio::Result<super::logging::WriteLogEntriesResponse> {
        self.write_log_entries_opt(req, ::grpcio::CallOption::default())
    }

    pub fn write_log_entries_async_opt(&self, req: &super::logging::WriteLogEntriesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::logging::WriteLogEntriesResponse>> {
        self.client.unary_call_async(&METHOD_LOGGING_SERVICE_V2_WRITE_LOG_ENTRIES, req, opt)
    }

    pub fn write_log_entries_async(&self, req: &super::logging::WriteLogEntriesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::logging::WriteLogEntriesResponse>> {
        self.write_log_entries_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_log_entries_opt(&self, req: &super::logging::ListLogEntriesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::logging::ListLogEntriesResponse> {
        self.client.unary_call(&METHOD_LOGGING_SERVICE_V2_LIST_LOG_ENTRIES, req, opt)
    }

    pub fn list_log_entries(&self, req: &super::logging::ListLogEntriesRequest) -> ::grpcio::Result<super::logging::ListLogEntriesResponse> {
        self.list_log_entries_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_log_entries_async_opt(&self, req: &super::logging::ListLogEntriesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::logging::ListLogEntriesResponse>> {
        self.client.unary_call_async(&METHOD_LOGGING_SERVICE_V2_LIST_LOG_ENTRIES, req, opt)
    }

    pub fn list_log_entries_async(&self, req: &super::logging::ListLogEntriesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::logging::ListLogEntriesResponse>> {
        self.list_log_entries_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_monitored_resource_descriptors_opt(&self, req: &super::logging::ListMonitoredResourceDescriptorsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::logging::ListMonitoredResourceDescriptorsResponse> {
        self.client.unary_call(&METHOD_LOGGING_SERVICE_V2_LIST_MONITORED_RESOURCE_DESCRIPTORS, req, opt)
    }

    pub fn list_monitored_resource_descriptors(&self, req: &super::logging::ListMonitoredResourceDescriptorsRequest) -> ::grpcio::Result<super::logging::ListMonitoredResourceDescriptorsResponse> {
        self.list_monitored_resource_descriptors_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_monitored_resource_descriptors_async_opt(&self, req: &super::logging::ListMonitoredResourceDescriptorsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::logging::ListMonitoredResourceDescriptorsResponse>> {
        self.client.unary_call_async(&METHOD_LOGGING_SERVICE_V2_LIST_MONITORED_RESOURCE_DESCRIPTORS, req, opt)
    }

    pub fn list_monitored_resource_descriptors_async(&self, req: &super::logging::ListMonitoredResourceDescriptorsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::logging::ListMonitoredResourceDescriptorsResponse>> {
        self.list_monitored_resource_descriptors_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_logs_opt(&self, req: &super::logging::ListLogsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::logging::ListLogsResponse> {
        self.client.unary_call(&METHOD_LOGGING_SERVICE_V2_LIST_LOGS, req, opt)
    }

    pub fn list_logs(&self, req: &super::logging::ListLogsRequest) -> ::grpcio::Result<super::logging::ListLogsResponse> {
        self.list_logs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_logs_async_opt(&self, req: &super::logging::ListLogsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::logging::ListLogsResponse>> {
        self.client.unary_call_async(&METHOD_LOGGING_SERVICE_V2_LIST_LOGS, req, opt)
    }

    pub fn list_logs_async(&self, req: &super::logging::ListLogsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::logging::ListLogsResponse>> {
        self.list_logs_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait LoggingServiceV2 {
    fn delete_log(&mut self, ctx: ::grpcio::RpcContext, _req: super::logging::DeleteLogRequest, sink: ::grpcio::UnarySink<super::empty::Empty>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn write_log_entries(&mut self, ctx: ::grpcio::RpcContext, _req: super::logging::WriteLogEntriesRequest, sink: ::grpcio::UnarySink<super::logging::WriteLogEntriesResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_log_entries(&mut self, ctx: ::grpcio::RpcContext, _req: super::logging::ListLogEntriesRequest, sink: ::grpcio::UnarySink<super::logging::ListLogEntriesResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_monitored_resource_descriptors(&mut self, ctx: ::grpcio::RpcContext, _req: super::logging::ListMonitoredResourceDescriptorsRequest, sink: ::grpcio::UnarySink<super::logging::ListMonitoredResourceDescriptorsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_logs(&mut self, ctx: ::grpcio::RpcContext, _req: super::logging::ListLogsRequest, sink: ::grpcio::UnarySink<super::logging::ListLogsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_logging_service_v2<S: LoggingServiceV2 + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LOGGING_SERVICE_V2_DELETE_LOG, move |ctx, req, resp| {
        instance.delete_log(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LOGGING_SERVICE_V2_WRITE_LOG_ENTRIES, move |ctx, req, resp| {
        instance.write_log_entries(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LOGGING_SERVICE_V2_LIST_LOG_ENTRIES, move |ctx, req, resp| {
        instance.list_log_entries(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LOGGING_SERVICE_V2_LIST_MONITORED_RESOURCE_DESCRIPTORS, move |ctx, req, resp| {
        instance.list_monitored_resource_descriptors(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_LOGGING_SERVICE_V2_LIST_LOGS, move |ctx, req, resp| {
        instance.list_logs(ctx, req, resp)
    });
    builder.build()
}
