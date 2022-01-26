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

const METHOD_ALERT_POLICY_SERVICE_LIST_ALERT_POLICIES: ::grpcio::Method<super::alert_service::ListAlertPoliciesRequest, super::alert_service::ListAlertPoliciesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.AlertPolicyService/ListAlertPolicies",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ALERT_POLICY_SERVICE_GET_ALERT_POLICY: ::grpcio::Method<super::alert_service::GetAlertPolicyRequest, super::alert::AlertPolicy> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.AlertPolicyService/GetAlertPolicy",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ALERT_POLICY_SERVICE_CREATE_ALERT_POLICY: ::grpcio::Method<super::alert_service::CreateAlertPolicyRequest, super::alert::AlertPolicy> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.AlertPolicyService/CreateAlertPolicy",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ALERT_POLICY_SERVICE_DELETE_ALERT_POLICY: ::grpcio::Method<super::alert_service::DeleteAlertPolicyRequest, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.AlertPolicyService/DeleteAlertPolicy",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ALERT_POLICY_SERVICE_UPDATE_ALERT_POLICY: ::grpcio::Method<super::alert_service::UpdateAlertPolicyRequest, super::alert::AlertPolicy> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.AlertPolicyService/UpdateAlertPolicy",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct AlertPolicyServiceClient {
    client: ::grpcio::Client,
}

impl AlertPolicyServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        AlertPolicyServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn list_alert_policies_opt(&self, req: &super::alert_service::ListAlertPoliciesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::alert_service::ListAlertPoliciesResponse> {
        self.client.unary_call(&METHOD_ALERT_POLICY_SERVICE_LIST_ALERT_POLICIES, req, opt)
    }

    pub fn list_alert_policies(&self, req: &super::alert_service::ListAlertPoliciesRequest) -> ::grpcio::Result<super::alert_service::ListAlertPoliciesResponse> {
        self.list_alert_policies_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_alert_policies_async_opt(&self, req: &super::alert_service::ListAlertPoliciesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::alert_service::ListAlertPoliciesResponse>> {
        self.client.unary_call_async(&METHOD_ALERT_POLICY_SERVICE_LIST_ALERT_POLICIES, req, opt)
    }

    pub fn list_alert_policies_async(&self, req: &super::alert_service::ListAlertPoliciesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::alert_service::ListAlertPoliciesResponse>> {
        self.list_alert_policies_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_alert_policy_opt(&self, req: &super::alert_service::GetAlertPolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::alert::AlertPolicy> {
        self.client.unary_call(&METHOD_ALERT_POLICY_SERVICE_GET_ALERT_POLICY, req, opt)
    }

    pub fn get_alert_policy(&self, req: &super::alert_service::GetAlertPolicyRequest) -> ::grpcio::Result<super::alert::AlertPolicy> {
        self.get_alert_policy_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_alert_policy_async_opt(&self, req: &super::alert_service::GetAlertPolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::alert::AlertPolicy>> {
        self.client.unary_call_async(&METHOD_ALERT_POLICY_SERVICE_GET_ALERT_POLICY, req, opt)
    }

    pub fn get_alert_policy_async(&self, req: &super::alert_service::GetAlertPolicyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::alert::AlertPolicy>> {
        self.get_alert_policy_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_alert_policy_opt(&self, req: &super::alert_service::CreateAlertPolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::alert::AlertPolicy> {
        self.client.unary_call(&METHOD_ALERT_POLICY_SERVICE_CREATE_ALERT_POLICY, req, opt)
    }

    pub fn create_alert_policy(&self, req: &super::alert_service::CreateAlertPolicyRequest) -> ::grpcio::Result<super::alert::AlertPolicy> {
        self.create_alert_policy_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_alert_policy_async_opt(&self, req: &super::alert_service::CreateAlertPolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::alert::AlertPolicy>> {
        self.client.unary_call_async(&METHOD_ALERT_POLICY_SERVICE_CREATE_ALERT_POLICY, req, opt)
    }

    pub fn create_alert_policy_async(&self, req: &super::alert_service::CreateAlertPolicyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::alert::AlertPolicy>> {
        self.create_alert_policy_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_alert_policy_opt(&self, req: &super::alert_service::DeleteAlertPolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_ALERT_POLICY_SERVICE_DELETE_ALERT_POLICY, req, opt)
    }

    pub fn delete_alert_policy(&self, req: &super::alert_service::DeleteAlertPolicyRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.delete_alert_policy_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_alert_policy_async_opt(&self, req: &super::alert_service::DeleteAlertPolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_ALERT_POLICY_SERVICE_DELETE_ALERT_POLICY, req, opt)
    }

    pub fn delete_alert_policy_async(&self, req: &super::alert_service::DeleteAlertPolicyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.delete_alert_policy_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_alert_policy_opt(&self, req: &super::alert_service::UpdateAlertPolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::alert::AlertPolicy> {
        self.client.unary_call(&METHOD_ALERT_POLICY_SERVICE_UPDATE_ALERT_POLICY, req, opt)
    }

    pub fn update_alert_policy(&self, req: &super::alert_service::UpdateAlertPolicyRequest) -> ::grpcio::Result<super::alert::AlertPolicy> {
        self.update_alert_policy_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_alert_policy_async_opt(&self, req: &super::alert_service::UpdateAlertPolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::alert::AlertPolicy>> {
        self.client.unary_call_async(&METHOD_ALERT_POLICY_SERVICE_UPDATE_ALERT_POLICY, req, opt)
    }

    pub fn update_alert_policy_async(&self, req: &super::alert_service::UpdateAlertPolicyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::alert::AlertPolicy>> {
        self.update_alert_policy_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait AlertPolicyService {
    fn list_alert_policies(&mut self, ctx: ::grpcio::RpcContext, _req: super::alert_service::ListAlertPoliciesRequest, sink: ::grpcio::UnarySink<super::alert_service::ListAlertPoliciesResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_alert_policy(&mut self, ctx: ::grpcio::RpcContext, _req: super::alert_service::GetAlertPolicyRequest, sink: ::grpcio::UnarySink<super::alert::AlertPolicy>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn create_alert_policy(&mut self, ctx: ::grpcio::RpcContext, _req: super::alert_service::CreateAlertPolicyRequest, sink: ::grpcio::UnarySink<super::alert::AlertPolicy>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_alert_policy(&mut self, ctx: ::grpcio::RpcContext, _req: super::alert_service::DeleteAlertPolicyRequest, sink: ::grpcio::UnarySink<super::empty::Empty>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_alert_policy(&mut self, ctx: ::grpcio::RpcContext, _req: super::alert_service::UpdateAlertPolicyRequest, sink: ::grpcio::UnarySink<super::alert::AlertPolicy>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_alert_policy_service<S: AlertPolicyService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ALERT_POLICY_SERVICE_LIST_ALERT_POLICIES, move |ctx, req, resp| {
        instance.list_alert_policies(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ALERT_POLICY_SERVICE_GET_ALERT_POLICY, move |ctx, req, resp| {
        instance.get_alert_policy(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ALERT_POLICY_SERVICE_CREATE_ALERT_POLICY, move |ctx, req, resp| {
        instance.create_alert_policy(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ALERT_POLICY_SERVICE_DELETE_ALERT_POLICY, move |ctx, req, resp| {
        instance.delete_alert_policy(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_ALERT_POLICY_SERVICE_UPDATE_ALERT_POLICY, move |ctx, req, resp| {
        instance.update_alert_policy(ctx, req, resp)
    });
    builder.build()
}
