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

const METHOD_GROUP_SERVICE_LIST_GROUPS: ::grpcio::Method<super::group_service::ListGroupsRequest, super::group_service::ListGroupsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.GroupService/ListGroups",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_GROUP_SERVICE_GET_GROUP: ::grpcio::Method<super::group_service::GetGroupRequest, super::group::Group> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.GroupService/GetGroup",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_GROUP_SERVICE_CREATE_GROUP: ::grpcio::Method<super::group_service::CreateGroupRequest, super::group::Group> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.GroupService/CreateGroup",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_GROUP_SERVICE_UPDATE_GROUP: ::grpcio::Method<super::group_service::UpdateGroupRequest, super::group::Group> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.GroupService/UpdateGroup",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_GROUP_SERVICE_DELETE_GROUP: ::grpcio::Method<super::group_service::DeleteGroupRequest, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.GroupService/DeleteGroup",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_GROUP_SERVICE_LIST_GROUP_MEMBERS: ::grpcio::Method<super::group_service::ListGroupMembersRequest, super::group_service::ListGroupMembersResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.GroupService/ListGroupMembers",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct GroupServiceClient {
    client: ::grpcio::Client,
}

impl GroupServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        GroupServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn list_groups_opt(&self, req: &super::group_service::ListGroupsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::group_service::ListGroupsResponse> {
        self.client.unary_call(&METHOD_GROUP_SERVICE_LIST_GROUPS, req, opt)
    }

    pub fn list_groups(&self, req: &super::group_service::ListGroupsRequest) -> ::grpcio::Result<super::group_service::ListGroupsResponse> {
        self.list_groups_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_groups_async_opt(&self, req: &super::group_service::ListGroupsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::group_service::ListGroupsResponse>> {
        self.client.unary_call_async(&METHOD_GROUP_SERVICE_LIST_GROUPS, req, opt)
    }

    pub fn list_groups_async(&self, req: &super::group_service::ListGroupsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::group_service::ListGroupsResponse>> {
        self.list_groups_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_group_opt(&self, req: &super::group_service::GetGroupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::group::Group> {
        self.client.unary_call(&METHOD_GROUP_SERVICE_GET_GROUP, req, opt)
    }

    pub fn get_group(&self, req: &super::group_service::GetGroupRequest) -> ::grpcio::Result<super::group::Group> {
        self.get_group_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_group_async_opt(&self, req: &super::group_service::GetGroupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::group::Group>> {
        self.client.unary_call_async(&METHOD_GROUP_SERVICE_GET_GROUP, req, opt)
    }

    pub fn get_group_async(&self, req: &super::group_service::GetGroupRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::group::Group>> {
        self.get_group_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_group_opt(&self, req: &super::group_service::CreateGroupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::group::Group> {
        self.client.unary_call(&METHOD_GROUP_SERVICE_CREATE_GROUP, req, opt)
    }

    pub fn create_group(&self, req: &super::group_service::CreateGroupRequest) -> ::grpcio::Result<super::group::Group> {
        self.create_group_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_group_async_opt(&self, req: &super::group_service::CreateGroupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::group::Group>> {
        self.client.unary_call_async(&METHOD_GROUP_SERVICE_CREATE_GROUP, req, opt)
    }

    pub fn create_group_async(&self, req: &super::group_service::CreateGroupRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::group::Group>> {
        self.create_group_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_group_opt(&self, req: &super::group_service::UpdateGroupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::group::Group> {
        self.client.unary_call(&METHOD_GROUP_SERVICE_UPDATE_GROUP, req, opt)
    }

    pub fn update_group(&self, req: &super::group_service::UpdateGroupRequest) -> ::grpcio::Result<super::group::Group> {
        self.update_group_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_group_async_opt(&self, req: &super::group_service::UpdateGroupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::group::Group>> {
        self.client.unary_call_async(&METHOD_GROUP_SERVICE_UPDATE_GROUP, req, opt)
    }

    pub fn update_group_async(&self, req: &super::group_service::UpdateGroupRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::group::Group>> {
        self.update_group_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_group_opt(&self, req: &super::group_service::DeleteGroupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_GROUP_SERVICE_DELETE_GROUP, req, opt)
    }

    pub fn delete_group(&self, req: &super::group_service::DeleteGroupRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.delete_group_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_group_async_opt(&self, req: &super::group_service::DeleteGroupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_GROUP_SERVICE_DELETE_GROUP, req, opt)
    }

    pub fn delete_group_async(&self, req: &super::group_service::DeleteGroupRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.delete_group_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_group_members_opt(&self, req: &super::group_service::ListGroupMembersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::group_service::ListGroupMembersResponse> {
        self.client.unary_call(&METHOD_GROUP_SERVICE_LIST_GROUP_MEMBERS, req, opt)
    }

    pub fn list_group_members(&self, req: &super::group_service::ListGroupMembersRequest) -> ::grpcio::Result<super::group_service::ListGroupMembersResponse> {
        self.list_group_members_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_group_members_async_opt(&self, req: &super::group_service::ListGroupMembersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::group_service::ListGroupMembersResponse>> {
        self.client.unary_call_async(&METHOD_GROUP_SERVICE_LIST_GROUP_MEMBERS, req, opt)
    }

    pub fn list_group_members_async(&self, req: &super::group_service::ListGroupMembersRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::group_service::ListGroupMembersResponse>> {
        self.list_group_members_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait GroupService {
    fn list_groups(&mut self, ctx: ::grpcio::RpcContext, _req: super::group_service::ListGroupsRequest, sink: ::grpcio::UnarySink<super::group_service::ListGroupsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_group(&mut self, ctx: ::grpcio::RpcContext, _req: super::group_service::GetGroupRequest, sink: ::grpcio::UnarySink<super::group::Group>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn create_group(&mut self, ctx: ::grpcio::RpcContext, _req: super::group_service::CreateGroupRequest, sink: ::grpcio::UnarySink<super::group::Group>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_group(&mut self, ctx: ::grpcio::RpcContext, _req: super::group_service::UpdateGroupRequest, sink: ::grpcio::UnarySink<super::group::Group>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_group(&mut self, ctx: ::grpcio::RpcContext, _req: super::group_service::DeleteGroupRequest, sink: ::grpcio::UnarySink<super::empty::Empty>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_group_members(&mut self, ctx: ::grpcio::RpcContext, _req: super::group_service::ListGroupMembersRequest, sink: ::grpcio::UnarySink<super::group_service::ListGroupMembersResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_group_service<S: GroupService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GROUP_SERVICE_LIST_GROUPS, move |ctx, req, resp| {
        instance.list_groups(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GROUP_SERVICE_GET_GROUP, move |ctx, req, resp| {
        instance.get_group(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GROUP_SERVICE_CREATE_GROUP, move |ctx, req, resp| {
        instance.create_group(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GROUP_SERVICE_UPDATE_GROUP, move |ctx, req, resp| {
        instance.update_group(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GROUP_SERVICE_DELETE_GROUP, move |ctx, req, resp| {
        instance.delete_group(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_GROUP_SERVICE_LIST_GROUP_MEMBERS, move |ctx, req, resp| {
        instance.list_group_members(ctx, req, resp)
    });
    builder.build()
}
