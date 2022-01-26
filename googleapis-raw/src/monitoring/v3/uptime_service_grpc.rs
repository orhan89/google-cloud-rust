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

const METHOD_UPTIME_CHECK_SERVICE_LIST_UPTIME_CHECK_CONFIGS: ::grpcio::Method<super::uptime_service::ListUptimeCheckConfigsRequest, super::uptime_service::ListUptimeCheckConfigsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.UptimeCheckService/ListUptimeCheckConfigs",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_UPTIME_CHECK_SERVICE_GET_UPTIME_CHECK_CONFIG: ::grpcio::Method<super::uptime_service::GetUptimeCheckConfigRequest, super::uptime::UptimeCheckConfig> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.UptimeCheckService/GetUptimeCheckConfig",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_UPTIME_CHECK_SERVICE_CREATE_UPTIME_CHECK_CONFIG: ::grpcio::Method<super::uptime_service::CreateUptimeCheckConfigRequest, super::uptime::UptimeCheckConfig> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.UptimeCheckService/CreateUptimeCheckConfig",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_UPTIME_CHECK_SERVICE_UPDATE_UPTIME_CHECK_CONFIG: ::grpcio::Method<super::uptime_service::UpdateUptimeCheckConfigRequest, super::uptime::UptimeCheckConfig> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.UptimeCheckService/UpdateUptimeCheckConfig",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_UPTIME_CHECK_SERVICE_DELETE_UPTIME_CHECK_CONFIG: ::grpcio::Method<super::uptime_service::DeleteUptimeCheckConfigRequest, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.UptimeCheckService/DeleteUptimeCheckConfig",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_UPTIME_CHECK_SERVICE_LIST_UPTIME_CHECK_IPS: ::grpcio::Method<super::uptime_service::ListUptimeCheckIpsRequest, super::uptime_service::ListUptimeCheckIpsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.UptimeCheckService/ListUptimeCheckIps",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct UptimeCheckServiceClient {
    client: ::grpcio::Client,
}

impl UptimeCheckServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        UptimeCheckServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn list_uptime_check_configs_opt(&self, req: &super::uptime_service::ListUptimeCheckConfigsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::uptime_service::ListUptimeCheckConfigsResponse> {
        self.client.unary_call(&METHOD_UPTIME_CHECK_SERVICE_LIST_UPTIME_CHECK_CONFIGS, req, opt)
    }

    pub fn list_uptime_check_configs(&self, req: &super::uptime_service::ListUptimeCheckConfigsRequest) -> ::grpcio::Result<super::uptime_service::ListUptimeCheckConfigsResponse> {
        self.list_uptime_check_configs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_uptime_check_configs_async_opt(&self, req: &super::uptime_service::ListUptimeCheckConfigsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::uptime_service::ListUptimeCheckConfigsResponse>> {
        self.client.unary_call_async(&METHOD_UPTIME_CHECK_SERVICE_LIST_UPTIME_CHECK_CONFIGS, req, opt)
    }

    pub fn list_uptime_check_configs_async(&self, req: &super::uptime_service::ListUptimeCheckConfigsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::uptime_service::ListUptimeCheckConfigsResponse>> {
        self.list_uptime_check_configs_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_uptime_check_config_opt(&self, req: &super::uptime_service::GetUptimeCheckConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::uptime::UptimeCheckConfig> {
        self.client.unary_call(&METHOD_UPTIME_CHECK_SERVICE_GET_UPTIME_CHECK_CONFIG, req, opt)
    }

    pub fn get_uptime_check_config(&self, req: &super::uptime_service::GetUptimeCheckConfigRequest) -> ::grpcio::Result<super::uptime::UptimeCheckConfig> {
        self.get_uptime_check_config_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_uptime_check_config_async_opt(&self, req: &super::uptime_service::GetUptimeCheckConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::uptime::UptimeCheckConfig>> {
        self.client.unary_call_async(&METHOD_UPTIME_CHECK_SERVICE_GET_UPTIME_CHECK_CONFIG, req, opt)
    }

    pub fn get_uptime_check_config_async(&self, req: &super::uptime_service::GetUptimeCheckConfigRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::uptime::UptimeCheckConfig>> {
        self.get_uptime_check_config_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_uptime_check_config_opt(&self, req: &super::uptime_service::CreateUptimeCheckConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::uptime::UptimeCheckConfig> {
        self.client.unary_call(&METHOD_UPTIME_CHECK_SERVICE_CREATE_UPTIME_CHECK_CONFIG, req, opt)
    }

    pub fn create_uptime_check_config(&self, req: &super::uptime_service::CreateUptimeCheckConfigRequest) -> ::grpcio::Result<super::uptime::UptimeCheckConfig> {
        self.create_uptime_check_config_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_uptime_check_config_async_opt(&self, req: &super::uptime_service::CreateUptimeCheckConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::uptime::UptimeCheckConfig>> {
        self.client.unary_call_async(&METHOD_UPTIME_CHECK_SERVICE_CREATE_UPTIME_CHECK_CONFIG, req, opt)
    }

    pub fn create_uptime_check_config_async(&self, req: &super::uptime_service::CreateUptimeCheckConfigRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::uptime::UptimeCheckConfig>> {
        self.create_uptime_check_config_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_uptime_check_config_opt(&self, req: &super::uptime_service::UpdateUptimeCheckConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::uptime::UptimeCheckConfig> {
        self.client.unary_call(&METHOD_UPTIME_CHECK_SERVICE_UPDATE_UPTIME_CHECK_CONFIG, req, opt)
    }

    pub fn update_uptime_check_config(&self, req: &super::uptime_service::UpdateUptimeCheckConfigRequest) -> ::grpcio::Result<super::uptime::UptimeCheckConfig> {
        self.update_uptime_check_config_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_uptime_check_config_async_opt(&self, req: &super::uptime_service::UpdateUptimeCheckConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::uptime::UptimeCheckConfig>> {
        self.client.unary_call_async(&METHOD_UPTIME_CHECK_SERVICE_UPDATE_UPTIME_CHECK_CONFIG, req, opt)
    }

    pub fn update_uptime_check_config_async(&self, req: &super::uptime_service::UpdateUptimeCheckConfigRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::uptime::UptimeCheckConfig>> {
        self.update_uptime_check_config_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_uptime_check_config_opt(&self, req: &super::uptime_service::DeleteUptimeCheckConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_UPTIME_CHECK_SERVICE_DELETE_UPTIME_CHECK_CONFIG, req, opt)
    }

    pub fn delete_uptime_check_config(&self, req: &super::uptime_service::DeleteUptimeCheckConfigRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.delete_uptime_check_config_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_uptime_check_config_async_opt(&self, req: &super::uptime_service::DeleteUptimeCheckConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_UPTIME_CHECK_SERVICE_DELETE_UPTIME_CHECK_CONFIG, req, opt)
    }

    pub fn delete_uptime_check_config_async(&self, req: &super::uptime_service::DeleteUptimeCheckConfigRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.delete_uptime_check_config_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_uptime_check_ips_opt(&self, req: &super::uptime_service::ListUptimeCheckIpsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::uptime_service::ListUptimeCheckIpsResponse> {
        self.client.unary_call(&METHOD_UPTIME_CHECK_SERVICE_LIST_UPTIME_CHECK_IPS, req, opt)
    }

    pub fn list_uptime_check_ips(&self, req: &super::uptime_service::ListUptimeCheckIpsRequest) -> ::grpcio::Result<super::uptime_service::ListUptimeCheckIpsResponse> {
        self.list_uptime_check_ips_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_uptime_check_ips_async_opt(&self, req: &super::uptime_service::ListUptimeCheckIpsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::uptime_service::ListUptimeCheckIpsResponse>> {
        self.client.unary_call_async(&METHOD_UPTIME_CHECK_SERVICE_LIST_UPTIME_CHECK_IPS, req, opt)
    }

    pub fn list_uptime_check_ips_async(&self, req: &super::uptime_service::ListUptimeCheckIpsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::uptime_service::ListUptimeCheckIpsResponse>> {
        self.list_uptime_check_ips_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait UptimeCheckService {
    fn list_uptime_check_configs(&mut self, ctx: ::grpcio::RpcContext, _req: super::uptime_service::ListUptimeCheckConfigsRequest, sink: ::grpcio::UnarySink<super::uptime_service::ListUptimeCheckConfigsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_uptime_check_config(&mut self, ctx: ::grpcio::RpcContext, _req: super::uptime_service::GetUptimeCheckConfigRequest, sink: ::grpcio::UnarySink<super::uptime::UptimeCheckConfig>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn create_uptime_check_config(&mut self, ctx: ::grpcio::RpcContext, _req: super::uptime_service::CreateUptimeCheckConfigRequest, sink: ::grpcio::UnarySink<super::uptime::UptimeCheckConfig>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_uptime_check_config(&mut self, ctx: ::grpcio::RpcContext, _req: super::uptime_service::UpdateUptimeCheckConfigRequest, sink: ::grpcio::UnarySink<super::uptime::UptimeCheckConfig>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_uptime_check_config(&mut self, ctx: ::grpcio::RpcContext, _req: super::uptime_service::DeleteUptimeCheckConfigRequest, sink: ::grpcio::UnarySink<super::empty::Empty>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_uptime_check_ips(&mut self, ctx: ::grpcio::RpcContext, _req: super::uptime_service::ListUptimeCheckIpsRequest, sink: ::grpcio::UnarySink<super::uptime_service::ListUptimeCheckIpsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_uptime_check_service<S: UptimeCheckService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_UPTIME_CHECK_SERVICE_LIST_UPTIME_CHECK_CONFIGS, move |ctx, req, resp| {
        instance.list_uptime_check_configs(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_UPTIME_CHECK_SERVICE_GET_UPTIME_CHECK_CONFIG, move |ctx, req, resp| {
        instance.get_uptime_check_config(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_UPTIME_CHECK_SERVICE_CREATE_UPTIME_CHECK_CONFIG, move |ctx, req, resp| {
        instance.create_uptime_check_config(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_UPTIME_CHECK_SERVICE_UPDATE_UPTIME_CHECK_CONFIG, move |ctx, req, resp| {
        instance.update_uptime_check_config(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_UPTIME_CHECK_SERVICE_DELETE_UPTIME_CHECK_CONFIG, move |ctx, req, resp| {
        instance.delete_uptime_check_config(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_UPTIME_CHECK_SERVICE_LIST_UPTIME_CHECK_IPS, move |ctx, req, resp| {
        instance.list_uptime_check_ips(ctx, req, resp)
    });
    builder.build()
}
