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

const METHOD_SERVICE_MONITORING_SERVICE_CREATE_SERVICE: ::grpcio::Method<super::service_service::CreateServiceRequest, super::service::Service> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.ServiceMonitoringService/CreateService",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SERVICE_MONITORING_SERVICE_GET_SERVICE: ::grpcio::Method<super::service_service::GetServiceRequest, super::service::Service> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.ServiceMonitoringService/GetService",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SERVICE_MONITORING_SERVICE_LIST_SERVICES: ::grpcio::Method<super::service_service::ListServicesRequest, super::service_service::ListServicesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.ServiceMonitoringService/ListServices",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SERVICE_MONITORING_SERVICE_UPDATE_SERVICE: ::grpcio::Method<super::service_service::UpdateServiceRequest, super::service::Service> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.ServiceMonitoringService/UpdateService",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SERVICE_MONITORING_SERVICE_DELETE_SERVICE: ::grpcio::Method<super::service_service::DeleteServiceRequest, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.ServiceMonitoringService/DeleteService",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SERVICE_MONITORING_SERVICE_CREATE_SERVICE_LEVEL_OBJECTIVE: ::grpcio::Method<super::service_service::CreateServiceLevelObjectiveRequest, super::service::ServiceLevelObjective> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.ServiceMonitoringService/CreateServiceLevelObjective",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SERVICE_MONITORING_SERVICE_GET_SERVICE_LEVEL_OBJECTIVE: ::grpcio::Method<super::service_service::GetServiceLevelObjectiveRequest, super::service::ServiceLevelObjective> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.ServiceMonitoringService/GetServiceLevelObjective",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SERVICE_MONITORING_SERVICE_LIST_SERVICE_LEVEL_OBJECTIVES: ::grpcio::Method<super::service_service::ListServiceLevelObjectivesRequest, super::service_service::ListServiceLevelObjectivesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.ServiceMonitoringService/ListServiceLevelObjectives",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SERVICE_MONITORING_SERVICE_UPDATE_SERVICE_LEVEL_OBJECTIVE: ::grpcio::Method<super::service_service::UpdateServiceLevelObjectiveRequest, super::service::ServiceLevelObjective> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.ServiceMonitoringService/UpdateServiceLevelObjective",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SERVICE_MONITORING_SERVICE_DELETE_SERVICE_LEVEL_OBJECTIVE: ::grpcio::Method<super::service_service::DeleteServiceLevelObjectiveRequest, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.ServiceMonitoringService/DeleteServiceLevelObjective",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct ServiceMonitoringServiceClient {
    client: ::grpcio::Client,
}

impl ServiceMonitoringServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ServiceMonitoringServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn create_service_opt(&self, req: &super::service_service::CreateServiceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::Service> {
        self.client.unary_call(&METHOD_SERVICE_MONITORING_SERVICE_CREATE_SERVICE, req, opt)
    }

    pub fn create_service(&self, req: &super::service_service::CreateServiceRequest) -> ::grpcio::Result<super::service::Service> {
        self.create_service_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_service_async_opt(&self, req: &super::service_service::CreateServiceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::Service>> {
        self.client.unary_call_async(&METHOD_SERVICE_MONITORING_SERVICE_CREATE_SERVICE, req, opt)
    }

    pub fn create_service_async(&self, req: &super::service_service::CreateServiceRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::Service>> {
        self.create_service_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_service_opt(&self, req: &super::service_service::GetServiceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::Service> {
        self.client.unary_call(&METHOD_SERVICE_MONITORING_SERVICE_GET_SERVICE, req, opt)
    }

    pub fn get_service(&self, req: &super::service_service::GetServiceRequest) -> ::grpcio::Result<super::service::Service> {
        self.get_service_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_service_async_opt(&self, req: &super::service_service::GetServiceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::Service>> {
        self.client.unary_call_async(&METHOD_SERVICE_MONITORING_SERVICE_GET_SERVICE, req, opt)
    }

    pub fn get_service_async(&self, req: &super::service_service::GetServiceRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::Service>> {
        self.get_service_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_services_opt(&self, req: &super::service_service::ListServicesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service_service::ListServicesResponse> {
        self.client.unary_call(&METHOD_SERVICE_MONITORING_SERVICE_LIST_SERVICES, req, opt)
    }

    pub fn list_services(&self, req: &super::service_service::ListServicesRequest) -> ::grpcio::Result<super::service_service::ListServicesResponse> {
        self.list_services_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_services_async_opt(&self, req: &super::service_service::ListServicesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service_service::ListServicesResponse>> {
        self.client.unary_call_async(&METHOD_SERVICE_MONITORING_SERVICE_LIST_SERVICES, req, opt)
    }

    pub fn list_services_async(&self, req: &super::service_service::ListServicesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service_service::ListServicesResponse>> {
        self.list_services_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_service_opt(&self, req: &super::service_service::UpdateServiceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::Service> {
        self.client.unary_call(&METHOD_SERVICE_MONITORING_SERVICE_UPDATE_SERVICE, req, opt)
    }

    pub fn update_service(&self, req: &super::service_service::UpdateServiceRequest) -> ::grpcio::Result<super::service::Service> {
        self.update_service_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_service_async_opt(&self, req: &super::service_service::UpdateServiceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::Service>> {
        self.client.unary_call_async(&METHOD_SERVICE_MONITORING_SERVICE_UPDATE_SERVICE, req, opt)
    }

    pub fn update_service_async(&self, req: &super::service_service::UpdateServiceRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::Service>> {
        self.update_service_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_service_opt(&self, req: &super::service_service::DeleteServiceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_SERVICE_MONITORING_SERVICE_DELETE_SERVICE, req, opt)
    }

    pub fn delete_service(&self, req: &super::service_service::DeleteServiceRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.delete_service_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_service_async_opt(&self, req: &super::service_service::DeleteServiceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_SERVICE_MONITORING_SERVICE_DELETE_SERVICE, req, opt)
    }

    pub fn delete_service_async(&self, req: &super::service_service::DeleteServiceRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.delete_service_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_service_level_objective_opt(&self, req: &super::service_service::CreateServiceLevelObjectiveRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::ServiceLevelObjective> {
        self.client.unary_call(&METHOD_SERVICE_MONITORING_SERVICE_CREATE_SERVICE_LEVEL_OBJECTIVE, req, opt)
    }

    pub fn create_service_level_objective(&self, req: &super::service_service::CreateServiceLevelObjectiveRequest) -> ::grpcio::Result<super::service::ServiceLevelObjective> {
        self.create_service_level_objective_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_service_level_objective_async_opt(&self, req: &super::service_service::CreateServiceLevelObjectiveRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::ServiceLevelObjective>> {
        self.client.unary_call_async(&METHOD_SERVICE_MONITORING_SERVICE_CREATE_SERVICE_LEVEL_OBJECTIVE, req, opt)
    }

    pub fn create_service_level_objective_async(&self, req: &super::service_service::CreateServiceLevelObjectiveRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::ServiceLevelObjective>> {
        self.create_service_level_objective_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_service_level_objective_opt(&self, req: &super::service_service::GetServiceLevelObjectiveRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::ServiceLevelObjective> {
        self.client.unary_call(&METHOD_SERVICE_MONITORING_SERVICE_GET_SERVICE_LEVEL_OBJECTIVE, req, opt)
    }

    pub fn get_service_level_objective(&self, req: &super::service_service::GetServiceLevelObjectiveRequest) -> ::grpcio::Result<super::service::ServiceLevelObjective> {
        self.get_service_level_objective_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_service_level_objective_async_opt(&self, req: &super::service_service::GetServiceLevelObjectiveRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::ServiceLevelObjective>> {
        self.client.unary_call_async(&METHOD_SERVICE_MONITORING_SERVICE_GET_SERVICE_LEVEL_OBJECTIVE, req, opt)
    }

    pub fn get_service_level_objective_async(&self, req: &super::service_service::GetServiceLevelObjectiveRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::ServiceLevelObjective>> {
        self.get_service_level_objective_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_service_level_objectives_opt(&self, req: &super::service_service::ListServiceLevelObjectivesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service_service::ListServiceLevelObjectivesResponse> {
        self.client.unary_call(&METHOD_SERVICE_MONITORING_SERVICE_LIST_SERVICE_LEVEL_OBJECTIVES, req, opt)
    }

    pub fn list_service_level_objectives(&self, req: &super::service_service::ListServiceLevelObjectivesRequest) -> ::grpcio::Result<super::service_service::ListServiceLevelObjectivesResponse> {
        self.list_service_level_objectives_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_service_level_objectives_async_opt(&self, req: &super::service_service::ListServiceLevelObjectivesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service_service::ListServiceLevelObjectivesResponse>> {
        self.client.unary_call_async(&METHOD_SERVICE_MONITORING_SERVICE_LIST_SERVICE_LEVEL_OBJECTIVES, req, opt)
    }

    pub fn list_service_level_objectives_async(&self, req: &super::service_service::ListServiceLevelObjectivesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service_service::ListServiceLevelObjectivesResponse>> {
        self.list_service_level_objectives_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_service_level_objective_opt(&self, req: &super::service_service::UpdateServiceLevelObjectiveRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::ServiceLevelObjective> {
        self.client.unary_call(&METHOD_SERVICE_MONITORING_SERVICE_UPDATE_SERVICE_LEVEL_OBJECTIVE, req, opt)
    }

    pub fn update_service_level_objective(&self, req: &super::service_service::UpdateServiceLevelObjectiveRequest) -> ::grpcio::Result<super::service::ServiceLevelObjective> {
        self.update_service_level_objective_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_service_level_objective_async_opt(&self, req: &super::service_service::UpdateServiceLevelObjectiveRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::ServiceLevelObjective>> {
        self.client.unary_call_async(&METHOD_SERVICE_MONITORING_SERVICE_UPDATE_SERVICE_LEVEL_OBJECTIVE, req, opt)
    }

    pub fn update_service_level_objective_async(&self, req: &super::service_service::UpdateServiceLevelObjectiveRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::ServiceLevelObjective>> {
        self.update_service_level_objective_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_service_level_objective_opt(&self, req: &super::service_service::DeleteServiceLevelObjectiveRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_SERVICE_MONITORING_SERVICE_DELETE_SERVICE_LEVEL_OBJECTIVE, req, opt)
    }

    pub fn delete_service_level_objective(&self, req: &super::service_service::DeleteServiceLevelObjectiveRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.delete_service_level_objective_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_service_level_objective_async_opt(&self, req: &super::service_service::DeleteServiceLevelObjectiveRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_SERVICE_MONITORING_SERVICE_DELETE_SERVICE_LEVEL_OBJECTIVE, req, opt)
    }

    pub fn delete_service_level_objective_async(&self, req: &super::service_service::DeleteServiceLevelObjectiveRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.delete_service_level_objective_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait ServiceMonitoringService {
    fn create_service(&mut self, ctx: ::grpcio::RpcContext, _req: super::service_service::CreateServiceRequest, sink: ::grpcio::UnarySink<super::service::Service>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_service(&mut self, ctx: ::grpcio::RpcContext, _req: super::service_service::GetServiceRequest, sink: ::grpcio::UnarySink<super::service::Service>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_services(&mut self, ctx: ::grpcio::RpcContext, _req: super::service_service::ListServicesRequest, sink: ::grpcio::UnarySink<super::service_service::ListServicesResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_service(&mut self, ctx: ::grpcio::RpcContext, _req: super::service_service::UpdateServiceRequest, sink: ::grpcio::UnarySink<super::service::Service>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_service(&mut self, ctx: ::grpcio::RpcContext, _req: super::service_service::DeleteServiceRequest, sink: ::grpcio::UnarySink<super::empty::Empty>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn create_service_level_objective(&mut self, ctx: ::grpcio::RpcContext, _req: super::service_service::CreateServiceLevelObjectiveRequest, sink: ::grpcio::UnarySink<super::service::ServiceLevelObjective>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_service_level_objective(&mut self, ctx: ::grpcio::RpcContext, _req: super::service_service::GetServiceLevelObjectiveRequest, sink: ::grpcio::UnarySink<super::service::ServiceLevelObjective>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_service_level_objectives(&mut self, ctx: ::grpcio::RpcContext, _req: super::service_service::ListServiceLevelObjectivesRequest, sink: ::grpcio::UnarySink<super::service_service::ListServiceLevelObjectivesResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_service_level_objective(&mut self, ctx: ::grpcio::RpcContext, _req: super::service_service::UpdateServiceLevelObjectiveRequest, sink: ::grpcio::UnarySink<super::service::ServiceLevelObjective>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_service_level_objective(&mut self, ctx: ::grpcio::RpcContext, _req: super::service_service::DeleteServiceLevelObjectiveRequest, sink: ::grpcio::UnarySink<super::empty::Empty>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_service_monitoring_service<S: ServiceMonitoringService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SERVICE_MONITORING_SERVICE_CREATE_SERVICE, move |ctx, req, resp| {
        instance.create_service(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SERVICE_MONITORING_SERVICE_GET_SERVICE, move |ctx, req, resp| {
        instance.get_service(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SERVICE_MONITORING_SERVICE_LIST_SERVICES, move |ctx, req, resp| {
        instance.list_services(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SERVICE_MONITORING_SERVICE_UPDATE_SERVICE, move |ctx, req, resp| {
        instance.update_service(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SERVICE_MONITORING_SERVICE_DELETE_SERVICE, move |ctx, req, resp| {
        instance.delete_service(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SERVICE_MONITORING_SERVICE_CREATE_SERVICE_LEVEL_OBJECTIVE, move |ctx, req, resp| {
        instance.create_service_level_objective(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SERVICE_MONITORING_SERVICE_GET_SERVICE_LEVEL_OBJECTIVE, move |ctx, req, resp| {
        instance.get_service_level_objective(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SERVICE_MONITORING_SERVICE_LIST_SERVICE_LEVEL_OBJECTIVES, move |ctx, req, resp| {
        instance.list_service_level_objectives(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SERVICE_MONITORING_SERVICE_UPDATE_SERVICE_LEVEL_OBJECTIVE, move |ctx, req, resp| {
        instance.update_service_level_objective(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_SERVICE_MONITORING_SERVICE_DELETE_SERVICE_LEVEL_OBJECTIVE, move |ctx, req, resp| {
        instance.delete_service_level_objective(ctx, req, resp)
    });
    builder.build()
}
