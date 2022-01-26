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

const METHOD_NOTIFICATION_CHANNEL_SERVICE_LIST_NOTIFICATION_CHANNEL_DESCRIPTORS: ::grpcio::Method<super::notification_service::ListNotificationChannelDescriptorsRequest, super::notification_service::ListNotificationChannelDescriptorsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.NotificationChannelService/ListNotificationChannelDescriptors",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_NOTIFICATION_CHANNEL_SERVICE_GET_NOTIFICATION_CHANNEL_DESCRIPTOR: ::grpcio::Method<super::notification_service::GetNotificationChannelDescriptorRequest, super::notification::NotificationChannelDescriptor> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.NotificationChannelService/GetNotificationChannelDescriptor",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_NOTIFICATION_CHANNEL_SERVICE_LIST_NOTIFICATION_CHANNELS: ::grpcio::Method<super::notification_service::ListNotificationChannelsRequest, super::notification_service::ListNotificationChannelsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.NotificationChannelService/ListNotificationChannels",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_NOTIFICATION_CHANNEL_SERVICE_GET_NOTIFICATION_CHANNEL: ::grpcio::Method<super::notification_service::GetNotificationChannelRequest, super::notification::NotificationChannel> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.NotificationChannelService/GetNotificationChannel",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_NOTIFICATION_CHANNEL_SERVICE_CREATE_NOTIFICATION_CHANNEL: ::grpcio::Method<super::notification_service::CreateNotificationChannelRequest, super::notification::NotificationChannel> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.NotificationChannelService/CreateNotificationChannel",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_NOTIFICATION_CHANNEL_SERVICE_UPDATE_NOTIFICATION_CHANNEL: ::grpcio::Method<super::notification_service::UpdateNotificationChannelRequest, super::notification::NotificationChannel> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.NotificationChannelService/UpdateNotificationChannel",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_NOTIFICATION_CHANNEL_SERVICE_DELETE_NOTIFICATION_CHANNEL: ::grpcio::Method<super::notification_service::DeleteNotificationChannelRequest, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.NotificationChannelService/DeleteNotificationChannel",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_NOTIFICATION_CHANNEL_SERVICE_SEND_NOTIFICATION_CHANNEL_VERIFICATION_CODE: ::grpcio::Method<super::notification_service::SendNotificationChannelVerificationCodeRequest, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.NotificationChannelService/SendNotificationChannelVerificationCode",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_NOTIFICATION_CHANNEL_SERVICE_GET_NOTIFICATION_CHANNEL_VERIFICATION_CODE: ::grpcio::Method<super::notification_service::GetNotificationChannelVerificationCodeRequest, super::notification_service::GetNotificationChannelVerificationCodeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.NotificationChannelService/GetNotificationChannelVerificationCode",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_NOTIFICATION_CHANNEL_SERVICE_VERIFY_NOTIFICATION_CHANNEL: ::grpcio::Method<super::notification_service::VerifyNotificationChannelRequest, super::notification::NotificationChannel> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.monitoring.v3.NotificationChannelService/VerifyNotificationChannel",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct NotificationChannelServiceClient {
    client: ::grpcio::Client,
}

impl NotificationChannelServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        NotificationChannelServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn list_notification_channel_descriptors_opt(&self, req: &super::notification_service::ListNotificationChannelDescriptorsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::notification_service::ListNotificationChannelDescriptorsResponse> {
        self.client.unary_call(&METHOD_NOTIFICATION_CHANNEL_SERVICE_LIST_NOTIFICATION_CHANNEL_DESCRIPTORS, req, opt)
    }

    pub fn list_notification_channel_descriptors(&self, req: &super::notification_service::ListNotificationChannelDescriptorsRequest) -> ::grpcio::Result<super::notification_service::ListNotificationChannelDescriptorsResponse> {
        self.list_notification_channel_descriptors_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_notification_channel_descriptors_async_opt(&self, req: &super::notification_service::ListNotificationChannelDescriptorsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::notification_service::ListNotificationChannelDescriptorsResponse>> {
        self.client.unary_call_async(&METHOD_NOTIFICATION_CHANNEL_SERVICE_LIST_NOTIFICATION_CHANNEL_DESCRIPTORS, req, opt)
    }

    pub fn list_notification_channel_descriptors_async(&self, req: &super::notification_service::ListNotificationChannelDescriptorsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::notification_service::ListNotificationChannelDescriptorsResponse>> {
        self.list_notification_channel_descriptors_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_notification_channel_descriptor_opt(&self, req: &super::notification_service::GetNotificationChannelDescriptorRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::notification::NotificationChannelDescriptor> {
        self.client.unary_call(&METHOD_NOTIFICATION_CHANNEL_SERVICE_GET_NOTIFICATION_CHANNEL_DESCRIPTOR, req, opt)
    }

    pub fn get_notification_channel_descriptor(&self, req: &super::notification_service::GetNotificationChannelDescriptorRequest) -> ::grpcio::Result<super::notification::NotificationChannelDescriptor> {
        self.get_notification_channel_descriptor_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_notification_channel_descriptor_async_opt(&self, req: &super::notification_service::GetNotificationChannelDescriptorRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::notification::NotificationChannelDescriptor>> {
        self.client.unary_call_async(&METHOD_NOTIFICATION_CHANNEL_SERVICE_GET_NOTIFICATION_CHANNEL_DESCRIPTOR, req, opt)
    }

    pub fn get_notification_channel_descriptor_async(&self, req: &super::notification_service::GetNotificationChannelDescriptorRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::notification::NotificationChannelDescriptor>> {
        self.get_notification_channel_descriptor_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_notification_channels_opt(&self, req: &super::notification_service::ListNotificationChannelsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::notification_service::ListNotificationChannelsResponse> {
        self.client.unary_call(&METHOD_NOTIFICATION_CHANNEL_SERVICE_LIST_NOTIFICATION_CHANNELS, req, opt)
    }

    pub fn list_notification_channels(&self, req: &super::notification_service::ListNotificationChannelsRequest) -> ::grpcio::Result<super::notification_service::ListNotificationChannelsResponse> {
        self.list_notification_channels_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_notification_channels_async_opt(&self, req: &super::notification_service::ListNotificationChannelsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::notification_service::ListNotificationChannelsResponse>> {
        self.client.unary_call_async(&METHOD_NOTIFICATION_CHANNEL_SERVICE_LIST_NOTIFICATION_CHANNELS, req, opt)
    }

    pub fn list_notification_channels_async(&self, req: &super::notification_service::ListNotificationChannelsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::notification_service::ListNotificationChannelsResponse>> {
        self.list_notification_channels_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_notification_channel_opt(&self, req: &super::notification_service::GetNotificationChannelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::notification::NotificationChannel> {
        self.client.unary_call(&METHOD_NOTIFICATION_CHANNEL_SERVICE_GET_NOTIFICATION_CHANNEL, req, opt)
    }

    pub fn get_notification_channel(&self, req: &super::notification_service::GetNotificationChannelRequest) -> ::grpcio::Result<super::notification::NotificationChannel> {
        self.get_notification_channel_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_notification_channel_async_opt(&self, req: &super::notification_service::GetNotificationChannelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::notification::NotificationChannel>> {
        self.client.unary_call_async(&METHOD_NOTIFICATION_CHANNEL_SERVICE_GET_NOTIFICATION_CHANNEL, req, opt)
    }

    pub fn get_notification_channel_async(&self, req: &super::notification_service::GetNotificationChannelRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::notification::NotificationChannel>> {
        self.get_notification_channel_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_notification_channel_opt(&self, req: &super::notification_service::CreateNotificationChannelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::notification::NotificationChannel> {
        self.client.unary_call(&METHOD_NOTIFICATION_CHANNEL_SERVICE_CREATE_NOTIFICATION_CHANNEL, req, opt)
    }

    pub fn create_notification_channel(&self, req: &super::notification_service::CreateNotificationChannelRequest) -> ::grpcio::Result<super::notification::NotificationChannel> {
        self.create_notification_channel_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_notification_channel_async_opt(&self, req: &super::notification_service::CreateNotificationChannelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::notification::NotificationChannel>> {
        self.client.unary_call_async(&METHOD_NOTIFICATION_CHANNEL_SERVICE_CREATE_NOTIFICATION_CHANNEL, req, opt)
    }

    pub fn create_notification_channel_async(&self, req: &super::notification_service::CreateNotificationChannelRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::notification::NotificationChannel>> {
        self.create_notification_channel_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_notification_channel_opt(&self, req: &super::notification_service::UpdateNotificationChannelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::notification::NotificationChannel> {
        self.client.unary_call(&METHOD_NOTIFICATION_CHANNEL_SERVICE_UPDATE_NOTIFICATION_CHANNEL, req, opt)
    }

    pub fn update_notification_channel(&self, req: &super::notification_service::UpdateNotificationChannelRequest) -> ::grpcio::Result<super::notification::NotificationChannel> {
        self.update_notification_channel_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_notification_channel_async_opt(&self, req: &super::notification_service::UpdateNotificationChannelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::notification::NotificationChannel>> {
        self.client.unary_call_async(&METHOD_NOTIFICATION_CHANNEL_SERVICE_UPDATE_NOTIFICATION_CHANNEL, req, opt)
    }

    pub fn update_notification_channel_async(&self, req: &super::notification_service::UpdateNotificationChannelRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::notification::NotificationChannel>> {
        self.update_notification_channel_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_notification_channel_opt(&self, req: &super::notification_service::DeleteNotificationChannelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_NOTIFICATION_CHANNEL_SERVICE_DELETE_NOTIFICATION_CHANNEL, req, opt)
    }

    pub fn delete_notification_channel(&self, req: &super::notification_service::DeleteNotificationChannelRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.delete_notification_channel_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_notification_channel_async_opt(&self, req: &super::notification_service::DeleteNotificationChannelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_NOTIFICATION_CHANNEL_SERVICE_DELETE_NOTIFICATION_CHANNEL, req, opt)
    }

    pub fn delete_notification_channel_async(&self, req: &super::notification_service::DeleteNotificationChannelRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.delete_notification_channel_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_notification_channel_verification_code_opt(&self, req: &super::notification_service::SendNotificationChannelVerificationCodeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_NOTIFICATION_CHANNEL_SERVICE_SEND_NOTIFICATION_CHANNEL_VERIFICATION_CODE, req, opt)
    }

    pub fn send_notification_channel_verification_code(&self, req: &super::notification_service::SendNotificationChannelVerificationCodeRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.send_notification_channel_verification_code_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_notification_channel_verification_code_async_opt(&self, req: &super::notification_service::SendNotificationChannelVerificationCodeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_NOTIFICATION_CHANNEL_SERVICE_SEND_NOTIFICATION_CHANNEL_VERIFICATION_CODE, req, opt)
    }

    pub fn send_notification_channel_verification_code_async(&self, req: &super::notification_service::SendNotificationChannelVerificationCodeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.send_notification_channel_verification_code_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_notification_channel_verification_code_opt(&self, req: &super::notification_service::GetNotificationChannelVerificationCodeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::notification_service::GetNotificationChannelVerificationCodeResponse> {
        self.client.unary_call(&METHOD_NOTIFICATION_CHANNEL_SERVICE_GET_NOTIFICATION_CHANNEL_VERIFICATION_CODE, req, opt)
    }

    pub fn get_notification_channel_verification_code(&self, req: &super::notification_service::GetNotificationChannelVerificationCodeRequest) -> ::grpcio::Result<super::notification_service::GetNotificationChannelVerificationCodeResponse> {
        self.get_notification_channel_verification_code_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_notification_channel_verification_code_async_opt(&self, req: &super::notification_service::GetNotificationChannelVerificationCodeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::notification_service::GetNotificationChannelVerificationCodeResponse>> {
        self.client.unary_call_async(&METHOD_NOTIFICATION_CHANNEL_SERVICE_GET_NOTIFICATION_CHANNEL_VERIFICATION_CODE, req, opt)
    }

    pub fn get_notification_channel_verification_code_async(&self, req: &super::notification_service::GetNotificationChannelVerificationCodeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::notification_service::GetNotificationChannelVerificationCodeResponse>> {
        self.get_notification_channel_verification_code_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn verify_notification_channel_opt(&self, req: &super::notification_service::VerifyNotificationChannelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::notification::NotificationChannel> {
        self.client.unary_call(&METHOD_NOTIFICATION_CHANNEL_SERVICE_VERIFY_NOTIFICATION_CHANNEL, req, opt)
    }

    pub fn verify_notification_channel(&self, req: &super::notification_service::VerifyNotificationChannelRequest) -> ::grpcio::Result<super::notification::NotificationChannel> {
        self.verify_notification_channel_opt(req, ::grpcio::CallOption::default())
    }

    pub fn verify_notification_channel_async_opt(&self, req: &super::notification_service::VerifyNotificationChannelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::notification::NotificationChannel>> {
        self.client.unary_call_async(&METHOD_NOTIFICATION_CHANNEL_SERVICE_VERIFY_NOTIFICATION_CHANNEL, req, opt)
    }

    pub fn verify_notification_channel_async(&self, req: &super::notification_service::VerifyNotificationChannelRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::notification::NotificationChannel>> {
        self.verify_notification_channel_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait NotificationChannelService {
    fn list_notification_channel_descriptors(&mut self, ctx: ::grpcio::RpcContext, _req: super::notification_service::ListNotificationChannelDescriptorsRequest, sink: ::grpcio::UnarySink<super::notification_service::ListNotificationChannelDescriptorsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_notification_channel_descriptor(&mut self, ctx: ::grpcio::RpcContext, _req: super::notification_service::GetNotificationChannelDescriptorRequest, sink: ::grpcio::UnarySink<super::notification::NotificationChannelDescriptor>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_notification_channels(&mut self, ctx: ::grpcio::RpcContext, _req: super::notification_service::ListNotificationChannelsRequest, sink: ::grpcio::UnarySink<super::notification_service::ListNotificationChannelsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_notification_channel(&mut self, ctx: ::grpcio::RpcContext, _req: super::notification_service::GetNotificationChannelRequest, sink: ::grpcio::UnarySink<super::notification::NotificationChannel>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn create_notification_channel(&mut self, ctx: ::grpcio::RpcContext, _req: super::notification_service::CreateNotificationChannelRequest, sink: ::grpcio::UnarySink<super::notification::NotificationChannel>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_notification_channel(&mut self, ctx: ::grpcio::RpcContext, _req: super::notification_service::UpdateNotificationChannelRequest, sink: ::grpcio::UnarySink<super::notification::NotificationChannel>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_notification_channel(&mut self, ctx: ::grpcio::RpcContext, _req: super::notification_service::DeleteNotificationChannelRequest, sink: ::grpcio::UnarySink<super::empty::Empty>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn send_notification_channel_verification_code(&mut self, ctx: ::grpcio::RpcContext, _req: super::notification_service::SendNotificationChannelVerificationCodeRequest, sink: ::grpcio::UnarySink<super::empty::Empty>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_notification_channel_verification_code(&mut self, ctx: ::grpcio::RpcContext, _req: super::notification_service::GetNotificationChannelVerificationCodeRequest, sink: ::grpcio::UnarySink<super::notification_service::GetNotificationChannelVerificationCodeResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn verify_notification_channel(&mut self, ctx: ::grpcio::RpcContext, _req: super::notification_service::VerifyNotificationChannelRequest, sink: ::grpcio::UnarySink<super::notification::NotificationChannel>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_notification_channel_service<S: NotificationChannelService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_NOTIFICATION_CHANNEL_SERVICE_LIST_NOTIFICATION_CHANNEL_DESCRIPTORS, move |ctx, req, resp| {
        instance.list_notification_channel_descriptors(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_NOTIFICATION_CHANNEL_SERVICE_GET_NOTIFICATION_CHANNEL_DESCRIPTOR, move |ctx, req, resp| {
        instance.get_notification_channel_descriptor(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_NOTIFICATION_CHANNEL_SERVICE_LIST_NOTIFICATION_CHANNELS, move |ctx, req, resp| {
        instance.list_notification_channels(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_NOTIFICATION_CHANNEL_SERVICE_GET_NOTIFICATION_CHANNEL, move |ctx, req, resp| {
        instance.get_notification_channel(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_NOTIFICATION_CHANNEL_SERVICE_CREATE_NOTIFICATION_CHANNEL, move |ctx, req, resp| {
        instance.create_notification_channel(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_NOTIFICATION_CHANNEL_SERVICE_UPDATE_NOTIFICATION_CHANNEL, move |ctx, req, resp| {
        instance.update_notification_channel(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_NOTIFICATION_CHANNEL_SERVICE_DELETE_NOTIFICATION_CHANNEL, move |ctx, req, resp| {
        instance.delete_notification_channel(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_NOTIFICATION_CHANNEL_SERVICE_SEND_NOTIFICATION_CHANNEL_VERIFICATION_CODE, move |ctx, req, resp| {
        instance.send_notification_channel_verification_code(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_NOTIFICATION_CHANNEL_SERVICE_GET_NOTIFICATION_CHANNEL_VERIFICATION_CODE, move |ctx, req, resp| {
        instance.get_notification_channel_verification_code(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_NOTIFICATION_CHANNEL_SERVICE_VERIFY_NOTIFICATION_CHANNEL, move |ctx, req, resp| {
        instance.verify_notification_channel(ctx, req, resp)
    });
    builder.build()
}
