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

const METHOD_METRIC_SERVICE_LIST_MONITORED_RESOURCE_DESCRIPTORS: ::grpcio::Method<super::metric_service::ListMonitoredResourceDescriptorsRequest, super::metric_service::ListMonitoredResourceDescriptorsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.MetricService/ListMonitoredResourceDescriptors",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_METRIC_SERVICE_GET_MONITORED_RESOURCE_DESCRIPTOR: ::grpcio::Method<super::metric_service::GetMonitoredResourceDescriptorRequest, super::monitored_resource::MonitoredResourceDescriptor> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.MetricService/GetMonitoredResourceDescriptor",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_METRIC_SERVICE_LIST_METRIC_DESCRIPTORS: ::grpcio::Method<super::metric_service::ListMetricDescriptorsRequest, super::metric_service::ListMetricDescriptorsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.MetricService/ListMetricDescriptors",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_METRIC_SERVICE_GET_METRIC_DESCRIPTOR: ::grpcio::Method<super::metric_service::GetMetricDescriptorRequest, super::metric_api::MetricDescriptor> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.MetricService/GetMetricDescriptor",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_METRIC_SERVICE_CREATE_METRIC_DESCRIPTOR: ::grpcio::Method<super::metric_service::CreateMetricDescriptorRequest, super::metric_api::MetricDescriptor> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.MetricService/CreateMetricDescriptor",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_METRIC_SERVICE_DELETE_METRIC_DESCRIPTOR: ::grpcio::Method<super::metric_service::DeleteMetricDescriptorRequest, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.MetricService/DeleteMetricDescriptor",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_METRIC_SERVICE_LIST_TIME_SERIES: ::grpcio::Method<super::metric_service::ListTimeSeriesRequest, super::metric_service::ListTimeSeriesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.MetricService/ListTimeSeries",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_METRIC_SERVICE_CREATE_TIME_SERIES: ::grpcio::Method<super::metric_service::CreateTimeSeriesRequest, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.MetricService/CreateTimeSeries",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct MetricServiceClient {
    client: ::grpcio::Client,
}

impl MetricServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        MetricServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn list_monitored_resource_descriptors_opt(&self, req: &super::metric_service::ListMonitoredResourceDescriptorsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::metric_service::ListMonitoredResourceDescriptorsResponse> {
        self.client.unary_call(&METHOD_METRIC_SERVICE_LIST_MONITORED_RESOURCE_DESCRIPTORS, req, opt)
    }

    pub fn list_monitored_resource_descriptors(&self, req: &super::metric_service::ListMonitoredResourceDescriptorsRequest) -> ::grpcio::Result<super::metric_service::ListMonitoredResourceDescriptorsResponse> {
        self.list_monitored_resource_descriptors_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_monitored_resource_descriptors_async_opt(&self, req: &super::metric_service::ListMonitoredResourceDescriptorsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::metric_service::ListMonitoredResourceDescriptorsResponse>> {
        self.client.unary_call_async(&METHOD_METRIC_SERVICE_LIST_MONITORED_RESOURCE_DESCRIPTORS, req, opt)
    }

    pub fn list_monitored_resource_descriptors_async(&self, req: &super::metric_service::ListMonitoredResourceDescriptorsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::metric_service::ListMonitoredResourceDescriptorsResponse>> {
        self.list_monitored_resource_descriptors_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_monitored_resource_descriptor_opt(&self, req: &super::metric_service::GetMonitoredResourceDescriptorRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::monitored_resource::MonitoredResourceDescriptor> {
        self.client.unary_call(&METHOD_METRIC_SERVICE_GET_MONITORED_RESOURCE_DESCRIPTOR, req, opt)
    }

    pub fn get_monitored_resource_descriptor(&self, req: &super::metric_service::GetMonitoredResourceDescriptorRequest) -> ::grpcio::Result<super::monitored_resource::MonitoredResourceDescriptor> {
        self.get_monitored_resource_descriptor_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_monitored_resource_descriptor_async_opt(&self, req: &super::metric_service::GetMonitoredResourceDescriptorRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::monitored_resource::MonitoredResourceDescriptor>> {
        self.client.unary_call_async(&METHOD_METRIC_SERVICE_GET_MONITORED_RESOURCE_DESCRIPTOR, req, opt)
    }

    pub fn get_monitored_resource_descriptor_async(&self, req: &super::metric_service::GetMonitoredResourceDescriptorRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::monitored_resource::MonitoredResourceDescriptor>> {
        self.get_monitored_resource_descriptor_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_metric_descriptors_opt(&self, req: &super::metric_service::ListMetricDescriptorsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::metric_service::ListMetricDescriptorsResponse> {
        self.client.unary_call(&METHOD_METRIC_SERVICE_LIST_METRIC_DESCRIPTORS, req, opt)
    }

    pub fn list_metric_descriptors(&self, req: &super::metric_service::ListMetricDescriptorsRequest) -> ::grpcio::Result<super::metric_service::ListMetricDescriptorsResponse> {
        self.list_metric_descriptors_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_metric_descriptors_async_opt(&self, req: &super::metric_service::ListMetricDescriptorsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::metric_service::ListMetricDescriptorsResponse>> {
        self.client.unary_call_async(&METHOD_METRIC_SERVICE_LIST_METRIC_DESCRIPTORS, req, opt)
    }

    pub fn list_metric_descriptors_async(&self, req: &super::metric_service::ListMetricDescriptorsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::metric_service::ListMetricDescriptorsResponse>> {
        self.list_metric_descriptors_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_metric_descriptor_opt(&self, req: &super::metric_service::GetMetricDescriptorRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::metric_api::MetricDescriptor> {
        self.client.unary_call(&METHOD_METRIC_SERVICE_GET_METRIC_DESCRIPTOR, req, opt)
    }

    pub fn get_metric_descriptor(&self, req: &super::metric_service::GetMetricDescriptorRequest) -> ::grpcio::Result<super::metric_api::MetricDescriptor> {
        self.get_metric_descriptor_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_metric_descriptor_async_opt(&self, req: &super::metric_service::GetMetricDescriptorRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::metric_api::MetricDescriptor>> {
        self.client.unary_call_async(&METHOD_METRIC_SERVICE_GET_METRIC_DESCRIPTOR, req, opt)
    }

    pub fn get_metric_descriptor_async(&self, req: &super::metric_service::GetMetricDescriptorRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::metric_api::MetricDescriptor>> {
        self.get_metric_descriptor_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_metric_descriptor_opt(&self, req: &super::metric_service::CreateMetricDescriptorRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::metric_api::MetricDescriptor> {
        self.client.unary_call(&METHOD_METRIC_SERVICE_CREATE_METRIC_DESCRIPTOR, req, opt)
    }

    pub fn create_metric_descriptor(&self, req: &super::metric_service::CreateMetricDescriptorRequest) -> ::grpcio::Result<super::metric_api::MetricDescriptor> {
        self.create_metric_descriptor_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_metric_descriptor_async_opt(&self, req: &super::metric_service::CreateMetricDescriptorRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::metric_api::MetricDescriptor>> {
        self.client.unary_call_async(&METHOD_METRIC_SERVICE_CREATE_METRIC_DESCRIPTOR, req, opt)
    }

    pub fn create_metric_descriptor_async(&self, req: &super::metric_service::CreateMetricDescriptorRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::metric_api::MetricDescriptor>> {
        self.create_metric_descriptor_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_metric_descriptor_opt(&self, req: &super::metric_service::DeleteMetricDescriptorRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_METRIC_SERVICE_DELETE_METRIC_DESCRIPTOR, req, opt)
    }

    pub fn delete_metric_descriptor(&self, req: &super::metric_service::DeleteMetricDescriptorRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.delete_metric_descriptor_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_metric_descriptor_async_opt(&self, req: &super::metric_service::DeleteMetricDescriptorRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_METRIC_SERVICE_DELETE_METRIC_DESCRIPTOR, req, opt)
    }

    pub fn delete_metric_descriptor_async(&self, req: &super::metric_service::DeleteMetricDescriptorRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.delete_metric_descriptor_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_time_series_opt(&self, req: &super::metric_service::ListTimeSeriesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::metric_service::ListTimeSeriesResponse> {
        self.client.unary_call(&METHOD_METRIC_SERVICE_LIST_TIME_SERIES, req, opt)
    }

    pub fn list_time_series(&self, req: &super::metric_service::ListTimeSeriesRequest) -> ::grpcio::Result<super::metric_service::ListTimeSeriesResponse> {
        self.list_time_series_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_time_series_async_opt(&self, req: &super::metric_service::ListTimeSeriesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::metric_service::ListTimeSeriesResponse>> {
        self.client.unary_call_async(&METHOD_METRIC_SERVICE_LIST_TIME_SERIES, req, opt)
    }

    pub fn list_time_series_async(&self, req: &super::metric_service::ListTimeSeriesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::metric_service::ListTimeSeriesResponse>> {
        self.list_time_series_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_time_series_opt(&self, req: &super::metric_service::CreateTimeSeriesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_METRIC_SERVICE_CREATE_TIME_SERIES, req, opt)
    }

    pub fn create_time_series(&self, req: &super::metric_service::CreateTimeSeriesRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.create_time_series_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_time_series_async_opt(&self, req: &super::metric_service::CreateTimeSeriesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_METRIC_SERVICE_CREATE_TIME_SERIES, req, opt)
    }

    pub fn create_time_series_async(&self, req: &super::metric_service::CreateTimeSeriesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.create_time_series_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait MetricService {
    fn list_monitored_resource_descriptors(&mut self, ctx: ::grpcio::RpcContext, _req: super::metric_service::ListMonitoredResourceDescriptorsRequest, sink: ::grpcio::UnarySink<super::metric_service::ListMonitoredResourceDescriptorsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_monitored_resource_descriptor(&mut self, ctx: ::grpcio::RpcContext, _req: super::metric_service::GetMonitoredResourceDescriptorRequest, sink: ::grpcio::UnarySink<super::monitored_resource::MonitoredResourceDescriptor>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_metric_descriptors(&mut self, ctx: ::grpcio::RpcContext, _req: super::metric_service::ListMetricDescriptorsRequest, sink: ::grpcio::UnarySink<super::metric_service::ListMetricDescriptorsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_metric_descriptor(&mut self, ctx: ::grpcio::RpcContext, _req: super::metric_service::GetMetricDescriptorRequest, sink: ::grpcio::UnarySink<super::metric_api::MetricDescriptor>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn create_metric_descriptor(&mut self, ctx: ::grpcio::RpcContext, _req: super::metric_service::CreateMetricDescriptorRequest, sink: ::grpcio::UnarySink<super::metric_api::MetricDescriptor>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_metric_descriptor(&mut self, ctx: ::grpcio::RpcContext, _req: super::metric_service::DeleteMetricDescriptorRequest, sink: ::grpcio::UnarySink<super::empty::Empty>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_time_series(&mut self, ctx: ::grpcio::RpcContext, _req: super::metric_service::ListTimeSeriesRequest, sink: ::grpcio::UnarySink<super::metric_service::ListTimeSeriesResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn create_time_series(&mut self, ctx: ::grpcio::RpcContext, _req: super::metric_service::CreateTimeSeriesRequest, sink: ::grpcio::UnarySink<super::empty::Empty>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_metric_service<S: MetricService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_METRIC_SERVICE_LIST_MONITORED_RESOURCE_DESCRIPTORS, move |ctx, req, resp| {
        instance.list_monitored_resource_descriptors(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_METRIC_SERVICE_GET_MONITORED_RESOURCE_DESCRIPTOR, move |ctx, req, resp| {
        instance.get_monitored_resource_descriptor(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_METRIC_SERVICE_LIST_METRIC_DESCRIPTORS, move |ctx, req, resp| {
        instance.list_metric_descriptors(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_METRIC_SERVICE_GET_METRIC_DESCRIPTOR, move |ctx, req, resp| {
        instance.get_metric_descriptor(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_METRIC_SERVICE_CREATE_METRIC_DESCRIPTOR, move |ctx, req, resp| {
        instance.create_metric_descriptor(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_METRIC_SERVICE_DELETE_METRIC_DESCRIPTOR, move |ctx, req, resp| {
        instance.delete_metric_descriptor(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_METRIC_SERVICE_LIST_TIME_SERIES, move |ctx, req, resp| {
        instance.list_time_series(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_METRIC_SERVICE_CREATE_TIME_SERIES, move |ctx, req, resp| {
        instance.create_time_series(ctx, req, resp)
    });
    builder.build()
}
