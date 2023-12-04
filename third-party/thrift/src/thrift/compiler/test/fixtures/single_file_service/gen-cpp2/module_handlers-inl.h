/**
 * Autogenerated by Thrift for 
 *
 * DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
 *  @generated @nocommit
 */
#pragma once

#include "thrift/compiler/test/fixtures/single_file_service/gen-cpp2/module_handlers.h"

#include <thrift/lib/cpp2/gen/service_tcc.h>

namespace cpp2 {
typedef apache::thrift::ThriftPresult<false> A_foo_pargs;
typedef apache::thrift::ThriftPresult<true, apache::thrift::FieldData<0, ::apache::thrift::type_class::structure, ::cpp2::Foo*>> A_foo_presult;
template <typename ProtocolIn_, typename ProtocolOut_>
void AAsyncProcessor::setUpAndProcess_foo(apache::thrift::ResponseChannelRequest::UniquePtr req, apache::thrift::SerializedCompressedRequest&& serializedRequest, apache::thrift::Cpp2RequestContext* ctx, folly::EventBase* eb, FOLLY_MAYBE_UNUSED apache::thrift::concurrency::ThreadManager* tm) {
  if (!setUpRequestProcessing(req, ctx, eb, tm, apache::thrift::RpcKind::SINGLE_REQUEST_SINGLE_RESPONSE, iface_)) {
    return;
  }
  auto scope = iface_->getRequestExecutionScope(ctx, apache::thrift::concurrency::NORMAL);
  ctx->setRequestExecutionScope(std::move(scope));
  processInThread(std::move(req), std::move(serializedRequest), ctx, eb, tm, apache::thrift::RpcKind::SINGLE_REQUEST_SINGLE_RESPONSE, &AAsyncProcessor::executeRequest_foo<ProtocolIn_, ProtocolOut_>, this);
}

template <typename ProtocolIn_, typename ProtocolOut_>
void AAsyncProcessor::executeRequest_foo(apache::thrift::ServerRequest&& serverRequest) {
  // make sure getRequestContext is null
  // so async calls don't accidentally use it
  iface_->setRequestContext(nullptr);
  ::cpp2::A_foo_pargs args;
  const auto* server = serverRequest.requestContext()->getConnectionContext()->getWorkerContext()->getServerContext();
  apache::thrift::ContextStack::UniquePtr ctxStack = apache::thrift::ContextStack::create(this->coalesceLegacyEventHandlersWith(server), this->getServiceName(), "A.foo", serverRequest.requestContext());
  try {
    deserializeRequest<ProtocolIn_>(args, "foo", apache::thrift::detail::ServerRequestHelper::compressedRequest(std::move(serverRequest)).uncompress(), ctxStack.get());
  }
  catch (...) {
    folly::exception_wrapper ew(std::current_exception());
    apache::thrift::detail::ap::process_handle_exn_deserialization<ProtocolOut_>(
        ew
        , apache::thrift::detail::ServerRequestHelper::request(std::move(serverRequest))
        , serverRequest.requestContext()
        , apache::thrift::detail::ServerRequestHelper::eventBase(serverRequest)
        , "foo");
    return;
  }
  auto requestPileNotification = apache::thrift::detail::ServerRequestHelper::moveRequestPileNotification(serverRequest);
  auto concurrencyControllerNotification = apache::thrift::detail::ServerRequestHelper::moveConcurrencyControllerNotification(serverRequest);
  auto callback = std::make_unique<apache::thrift::HandlerCallback<std::unique_ptr<::cpp2::Foo>>>(
    apache::thrift::detail::ServerRequestHelper::request(std::move(serverRequest))
    , std::move(ctxStack)
    , return_foo<ProtocolIn_,ProtocolOut_>
    , throw_wrapped_foo<ProtocolIn_, ProtocolOut_>
    , serverRequest.requestContext()->getProtoSeqId()
    , apache::thrift::detail::ServerRequestHelper::eventBase(serverRequest)
    , apache::thrift::detail::ServerRequestHelper::executor(serverRequest)
    , serverRequest.requestContext()
    , requestPileNotification
    , concurrencyControllerNotification, std::move(serverRequest.requestData())
    );
  iface_->async_tm_foo(std::move(callback));
}

template <class ProtocolIn_, class ProtocolOut_>
apache::thrift::SerializedResponse AAsyncProcessor::return_foo(apache::thrift::ContextStack* ctx, ::cpp2::Foo const& _return) {
  ProtocolOut_ prot;
  ::cpp2::A_foo_presult result;
  result.get<0>().value = const_cast<::cpp2::Foo*>(&_return);
  result.setIsSet(0, true);
  return serializeResponse("foo", &prot, ctx, result);
}

template <class ProtocolIn_, class ProtocolOut_>
void AAsyncProcessor::throw_wrapped_foo(apache::thrift::ResponseChannelRequest::UniquePtr req,FOLLY_MAYBE_UNUSED int32_t protoSeqId,apache::thrift::ContextStack* ctx,folly::exception_wrapper ew,apache::thrift::Cpp2RequestContext* reqCtx) {
  if (!ew) {
    return;
  }
  {
    apache::thrift::detail::ap::process_throw_wrapped_handler_error<ProtocolOut_>(
        ew, std::move(req), reqCtx, ctx, "foo");
    return;
  }
}


typedef apache::thrift::ThriftPresult<false> A_I_interact_pargs;
typedef apache::thrift::ThriftPresult<true> A_I_interact_presult;
template <typename ProtocolIn_, typename ProtocolOut_>
void AAsyncProcessor::setUpAndProcess_I_interact(apache::thrift::ResponseChannelRequest::UniquePtr req, apache::thrift::SerializedCompressedRequest&& serializedRequest, apache::thrift::Cpp2RequestContext* ctx, folly::EventBase* eb, FOLLY_MAYBE_UNUSED apache::thrift::concurrency::ThreadManager* tm) {
  if (!setUpRequestProcessing(req, ctx, eb, tm, apache::thrift::RpcKind::SINGLE_REQUEST_SINGLE_RESPONSE, iface_, "I")) {
    return;
  }
  auto scope = iface_->getRequestExecutionScope(ctx, apache::thrift::concurrency::NORMAL);
  ctx->setRequestExecutionScope(std::move(scope));
  processInThread(std::move(req), std::move(serializedRequest), ctx, eb, tm, apache::thrift::RpcKind::SINGLE_REQUEST_SINGLE_RESPONSE, &AAsyncProcessor::executeRequest_I_interact<ProtocolIn_, ProtocolOut_>, this);
}

template <typename ProtocolIn_, typename ProtocolOut_>
void AAsyncProcessor::executeRequest_I_interact(apache::thrift::ServerRequest&& serverRequest) {
  auto tile = serverRequest.requestContext()->releaseTile();
  // make sure getRequestContext is null
  // so async calls don't accidentally use it
  iface_->setRequestContext(nullptr);
  ::cpp2::A_I_interact_pargs args;
  const auto* server = serverRequest.requestContext()->getConnectionContext()->getWorkerContext()->getServerContext();
  apache::thrift::ContextStack::UniquePtr ctxStack = apache::thrift::ContextStack::create(this->coalesceLegacyEventHandlersWith(server), this->getServiceName(), "A.I.interact", serverRequest.requestContext());
  auto& iface = static_cast<apache::thrift::ServiceHandler<A>::IIf&>(*tile);
  try {
    deserializeRequest<ProtocolIn_>(args, "I.interact", apache::thrift::detail::ServerRequestHelper::compressedRequest(std::move(serverRequest)).uncompress(), ctxStack.get());
  }
  catch (...) {
    folly::exception_wrapper ew(std::current_exception());
    apache::thrift::detail::ap::process_handle_exn_deserialization<ProtocolOut_>(
        ew
        , apache::thrift::detail::ServerRequestHelper::request(std::move(serverRequest))
        , serverRequest.requestContext()
        , apache::thrift::detail::ServerRequestHelper::eventBase(serverRequest)
        , "I.interact");
    return;
  }
  auto requestPileNotification = apache::thrift::detail::ServerRequestHelper::moveRequestPileNotification(serverRequest);
  auto concurrencyControllerNotification = apache::thrift::detail::ServerRequestHelper::moveConcurrencyControllerNotification(serverRequest);
  auto callback = std::make_unique<apache::thrift::HandlerCallback<void>>(
    apache::thrift::detail::ServerRequestHelper::request(std::move(serverRequest))
    , std::move(ctxStack)
    , return_I_interact<ProtocolIn_,ProtocolOut_>
    , throw_wrapped_I_interact<ProtocolIn_, ProtocolOut_>
    , serverRequest.requestContext()->getProtoSeqId()
    , apache::thrift::detail::ServerRequestHelper::eventBase(serverRequest)
    , apache::thrift::detail::ServerRequestHelper::executor(serverRequest)
    , serverRequest.requestContext()
    , requestPileNotification
    , concurrencyControllerNotification, std::move(serverRequest.requestData())
    , std::move(tile));
  iface.async_tm_interact(std::move(callback));
}

template <class ProtocolIn_, class ProtocolOut_>
apache::thrift::SerializedResponse AAsyncProcessor::return_I_interact(apache::thrift::ContextStack* ctx) {
  ProtocolOut_ prot;
  ::cpp2::A_I_interact_presult result;
  return serializeResponse("I.interact", &prot, ctx, result);
}

template <class ProtocolIn_, class ProtocolOut_>
void AAsyncProcessor::throw_wrapped_I_interact(apache::thrift::ResponseChannelRequest::UniquePtr req,FOLLY_MAYBE_UNUSED int32_t protoSeqId,apache::thrift::ContextStack* ctx,folly::exception_wrapper ew,apache::thrift::Cpp2RequestContext* reqCtx) {
  if (!ew) {
    return;
  }
  {
    apache::thrift::detail::ap::process_throw_wrapped_handler_error<ProtocolOut_>(
        ew, std::move(req), reqCtx, ctx, "I.interact");
    return;
  }
}

} // cpp2

namespace cpp2 {
typedef apache::thrift::ThriftPresult<false, apache::thrift::FieldData<1, ::apache::thrift::type_class::structure, ::cpp2::Foo*>> B_bar_pargs;
typedef apache::thrift::ThriftPresult<true> B_bar_presult;
typedef apache::thrift::ThriftPresult<false> B_stream_stuff_pargs;
typedef apache::thrift::ThriftPResultStream<
    apache::thrift::ThriftPresult<true>,
    apache::thrift::ThriftPresult<true, apache::thrift::FieldData<0, ::apache::thrift::type_class::integral, ::std::int32_t*>>
    > B_stream_stuff_presult;
typedef apache::thrift::ThriftPresult<false> B_sink_stuff_pargs;
typedef apache::thrift::ThriftPResultSink<
    apache::thrift::ThriftPresult<true>,
    apache::thrift::ThriftPresult<true, apache::thrift::FieldData<0, ::apache::thrift::type_class::integral, ::std::int32_t*>>,
    apache::thrift::ThriftPresult<true, apache::thrift::FieldData<0, ::apache::thrift::type_class::integral, ::std::int32_t*>>
    > B_sink_stuff_presult;
template <typename ProtocolIn_, typename ProtocolOut_>
void BAsyncProcessor::setUpAndProcess_bar(apache::thrift::ResponseChannelRequest::UniquePtr req, apache::thrift::SerializedCompressedRequest&& serializedRequest, apache::thrift::Cpp2RequestContext* ctx, folly::EventBase* eb, FOLLY_MAYBE_UNUSED apache::thrift::concurrency::ThreadManager* tm) {
  if (!setUpRequestProcessing(req, ctx, eb, tm, apache::thrift::RpcKind::SINGLE_REQUEST_SINGLE_RESPONSE, iface_)) {
    return;
  }
  auto scope = iface_->getRequestExecutionScope(ctx, apache::thrift::concurrency::NORMAL);
  ctx->setRequestExecutionScope(std::move(scope));
  processInThread(std::move(req), std::move(serializedRequest), ctx, eb, tm, apache::thrift::RpcKind::SINGLE_REQUEST_SINGLE_RESPONSE, &BAsyncProcessor::executeRequest_bar<ProtocolIn_, ProtocolOut_>, this);
}

template <typename ProtocolIn_, typename ProtocolOut_>
void BAsyncProcessor::executeRequest_bar(apache::thrift::ServerRequest&& serverRequest) {
  // make sure getRequestContext is null
  // so async calls don't accidentally use it
  iface_->setRequestContext(nullptr);
  ::cpp2::B_bar_pargs args;
  auto uarg_foo = std::make_unique<::cpp2::Foo>();
  args.get<0>().value = uarg_foo.get();
  const auto* server = serverRequest.requestContext()->getConnectionContext()->getWorkerContext()->getServerContext();
  apache::thrift::ContextStack::UniquePtr ctxStack = apache::thrift::ContextStack::create(this->coalesceLegacyEventHandlersWith(server), this->getServiceName(), "B.bar", serverRequest.requestContext());
  try {
    deserializeRequest<ProtocolIn_>(args, "bar", apache::thrift::detail::ServerRequestHelper::compressedRequest(std::move(serverRequest)).uncompress(), ctxStack.get());
  }
  catch (...) {
    folly::exception_wrapper ew(std::current_exception());
    apache::thrift::detail::ap::process_handle_exn_deserialization<ProtocolOut_>(
        ew
        , apache::thrift::detail::ServerRequestHelper::request(std::move(serverRequest))
        , serverRequest.requestContext()
        , apache::thrift::detail::ServerRequestHelper::eventBase(serverRequest)
        , "bar");
    return;
  }
  auto requestPileNotification = apache::thrift::detail::ServerRequestHelper::moveRequestPileNotification(serverRequest);
  auto concurrencyControllerNotification = apache::thrift::detail::ServerRequestHelper::moveConcurrencyControllerNotification(serverRequest);
  auto callback = std::make_unique<apache::thrift::HandlerCallback<void>>(
    apache::thrift::detail::ServerRequestHelper::request(std::move(serverRequest))
    , std::move(ctxStack)
    , return_bar<ProtocolIn_,ProtocolOut_>
    , throw_wrapped_bar<ProtocolIn_, ProtocolOut_>
    , serverRequest.requestContext()->getProtoSeqId()
    , apache::thrift::detail::ServerRequestHelper::eventBase(serverRequest)
    , apache::thrift::detail::ServerRequestHelper::executor(serverRequest)
    , serverRequest.requestContext()
    , requestPileNotification
    , concurrencyControllerNotification, std::move(serverRequest.requestData())
    );
  iface_->async_tm_bar(std::move(callback), std::move(uarg_foo));
}

template <class ProtocolIn_, class ProtocolOut_>
apache::thrift::SerializedResponse BAsyncProcessor::return_bar(apache::thrift::ContextStack* ctx) {
  ProtocolOut_ prot;
  ::cpp2::B_bar_presult result;
  return serializeResponse("bar", &prot, ctx, result);
}

template <class ProtocolIn_, class ProtocolOut_>
void BAsyncProcessor::throw_wrapped_bar(apache::thrift::ResponseChannelRequest::UniquePtr req,FOLLY_MAYBE_UNUSED int32_t protoSeqId,apache::thrift::ContextStack* ctx,folly::exception_wrapper ew,apache::thrift::Cpp2RequestContext* reqCtx) {
  if (!ew) {
    return;
  }
  {
    apache::thrift::detail::ap::process_throw_wrapped_handler_error<ProtocolOut_>(
        ew, std::move(req), reqCtx, ctx, "bar");
    return;
  }
}

template <typename ProtocolIn_, typename ProtocolOut_>
void BAsyncProcessor::setUpAndProcess_stream_stuff(apache::thrift::ResponseChannelRequest::UniquePtr req, apache::thrift::SerializedCompressedRequest&& serializedRequest, apache::thrift::Cpp2RequestContext* ctx, folly::EventBase* eb, FOLLY_MAYBE_UNUSED apache::thrift::concurrency::ThreadManager* tm) {
  if (!setUpRequestProcessing(req, ctx, eb, tm, apache::thrift::RpcKind::SINGLE_REQUEST_STREAMING_RESPONSE, iface_)) {
    return;
  }
  auto scope = iface_->getRequestExecutionScope(ctx, apache::thrift::concurrency::NORMAL);
  ctx->setRequestExecutionScope(std::move(scope));
  processInThread(std::move(req), std::move(serializedRequest), ctx, eb, tm, apache::thrift::RpcKind::SINGLE_REQUEST_STREAMING_RESPONSE, &BAsyncProcessor::executeRequest_stream_stuff<ProtocolIn_, ProtocolOut_>, this);
}

template <typename ProtocolIn_, typename ProtocolOut_>
void BAsyncProcessor::executeRequest_stream_stuff(apache::thrift::ServerRequest&& serverRequest) {
  // make sure getRequestContext is null
  // so async calls don't accidentally use it
  iface_->setRequestContext(nullptr);
  ::cpp2::B_stream_stuff_pargs args;
  const auto* server = serverRequest.requestContext()->getConnectionContext()->getWorkerContext()->getServerContext();
  apache::thrift::ContextStack::UniquePtr ctxStack = apache::thrift::ContextStack::create(this->coalesceLegacyEventHandlersWith(server), this->getServiceName(), "B.stream_stuff", serverRequest.requestContext());
  try {
    deserializeRequest<ProtocolIn_>(args, "stream_stuff", apache::thrift::detail::ServerRequestHelper::compressedRequest(std::move(serverRequest)).uncompress(), ctxStack.get());
  }
  catch (...) {
    folly::exception_wrapper ew(std::current_exception());
    apache::thrift::detail::ap::process_handle_exn_deserialization<ProtocolOut_>(
        ew
        , apache::thrift::detail::ServerRequestHelper::request(std::move(serverRequest))
        , serverRequest.requestContext()
        , apache::thrift::detail::ServerRequestHelper::eventBase(serverRequest)
        , "stream_stuff");
    return;
  }
  auto requestPileNotification = apache::thrift::detail::ServerRequestHelper::moveRequestPileNotification(serverRequest);
  auto concurrencyControllerNotification = apache::thrift::detail::ServerRequestHelper::moveConcurrencyControllerNotification(serverRequest);
  auto callback = std::make_unique<apache::thrift::HandlerCallback<::apache::thrift::ServerStream<::std::int32_t>>>(
    apache::thrift::detail::ServerRequestHelper::request(std::move(serverRequest))
    , std::move(ctxStack)
    , return_stream_stuff<ProtocolIn_,ProtocolOut_>
    , throw_wrapped_stream_stuff<ProtocolIn_, ProtocolOut_>
    , serverRequest.requestContext()->getProtoSeqId()
    , apache::thrift::detail::ServerRequestHelper::eventBase(serverRequest)
    , apache::thrift::detail::ServerRequestHelper::executor(serverRequest)
    , serverRequest.requestContext()
    , requestPileNotification
    , concurrencyControllerNotification, std::move(serverRequest.requestData())
    );
  iface_->async_tm_stream_stuff(std::move(callback));
}

template <class ProtocolIn_, class ProtocolOut_>
apache::thrift::ResponseAndServerStreamFactory BAsyncProcessor::return_stream_stuff(apache::thrift::ContextStack* ctx, folly::Executor::KeepAlive<> executor, ::apache::thrift::ServerStream<::std::int32_t>&& _return) {
  ProtocolOut_ prot;
  ::cpp2::B_stream_stuff_presult::FieldsType result;
  using StreamPResultType = ::cpp2::B_stream_stuff_presult::StreamPResultType;
  auto& returnStream = _return;

      using ExMapType = apache::thrift::detail::ap::EmptyExMapType;
  auto encodedStream = apache::thrift::detail::ap::encode_server_stream<ProtocolOut_, StreamPResultType, ExMapType>(std::move(returnStream), std::move(executor));
  return {serializeResponse("stream_stuff", &prot, ctx, result), std::move(encodedStream)};
}

template <class ProtocolIn_, class ProtocolOut_>
void BAsyncProcessor::throw_wrapped_stream_stuff(apache::thrift::ResponseChannelRequest::UniquePtr req,FOLLY_MAYBE_UNUSED int32_t protoSeqId,apache::thrift::ContextStack* ctx,folly::exception_wrapper ew,apache::thrift::Cpp2RequestContext* reqCtx) {
  if (!ew) {
    return;
  }
  {
    apache::thrift::detail::ap::process_throw_wrapped_handler_error<ProtocolOut_>(
        ew, std::move(req), reqCtx, ctx, "stream_stuff");
    return;
  }
}

template <typename ProtocolIn_, typename ProtocolOut_>
void BAsyncProcessor::setUpAndProcess_sink_stuff(apache::thrift::ResponseChannelRequest::UniquePtr req, apache::thrift::SerializedCompressedRequest&& serializedRequest, apache::thrift::Cpp2RequestContext* ctx, folly::EventBase* eb, FOLLY_MAYBE_UNUSED apache::thrift::concurrency::ThreadManager* tm) {
  if (!setUpRequestProcessing(req, ctx, eb, tm, apache::thrift::RpcKind::SINK, iface_)) {
    return;
  }
  auto scope = iface_->getRequestExecutionScope(ctx, apache::thrift::concurrency::NORMAL);
  ctx->setRequestExecutionScope(std::move(scope));
  processInThread(std::move(req), std::move(serializedRequest), ctx, eb, tm, apache::thrift::RpcKind::SINK, &BAsyncProcessor::executeRequest_sink_stuff<ProtocolIn_, ProtocolOut_>, this);
}

template <typename ProtocolIn_, typename ProtocolOut_>
void BAsyncProcessor::executeRequest_sink_stuff(apache::thrift::ServerRequest&& serverRequest) {
  // make sure getRequestContext is null
  // so async calls don't accidentally use it
  iface_->setRequestContext(nullptr);
  ::cpp2::B_sink_stuff_pargs args;
  const auto* server = serverRequest.requestContext()->getConnectionContext()->getWorkerContext()->getServerContext();
  apache::thrift::ContextStack::UniquePtr ctxStack = apache::thrift::ContextStack::create(this->coalesceLegacyEventHandlersWith(server), this->getServiceName(), "B.sink_stuff", serverRequest.requestContext());
  try {
    deserializeRequest<ProtocolIn_>(args, "sink_stuff", apache::thrift::detail::ServerRequestHelper::compressedRequest(std::move(serverRequest)).uncompress(), ctxStack.get());
  }
  catch (...) {
    folly::exception_wrapper ew(std::current_exception());
    apache::thrift::detail::ap::process_handle_exn_deserialization<ProtocolOut_>(
        ew
        , apache::thrift::detail::ServerRequestHelper::request(std::move(serverRequest))
        , serverRequest.requestContext()
        , apache::thrift::detail::ServerRequestHelper::eventBase(serverRequest)
        , "sink_stuff");
    return;
  }
  auto requestPileNotification = apache::thrift::detail::ServerRequestHelper::moveRequestPileNotification(serverRequest);
  auto concurrencyControllerNotification = apache::thrift::detail::ServerRequestHelper::moveConcurrencyControllerNotification(serverRequest);
  auto callback = std::make_unique<apache::thrift::HandlerCallback<::apache::thrift::SinkConsumer<::std::int32_t, ::std::int32_t>>>(
    apache::thrift::detail::ServerRequestHelper::request(std::move(serverRequest))
    , std::move(ctxStack)
    , return_sink_stuff<ProtocolIn_,ProtocolOut_>
    , throw_wrapped_sink_stuff<ProtocolIn_, ProtocolOut_>
    , serverRequest.requestContext()->getProtoSeqId()
    , apache::thrift::detail::ServerRequestHelper::eventBase(serverRequest)
    , apache::thrift::detail::ServerRequestHelper::executor(serverRequest)
    , serverRequest.requestContext()
    , requestPileNotification
    , concurrencyControllerNotification, std::move(serverRequest.requestData())
    );
  iface_->async_tm_sink_stuff(std::move(callback));
}

template <class ProtocolIn_, class ProtocolOut_>
std::pair<apache::thrift::SerializedResponse, apache::thrift::detail::SinkConsumerImpl> BAsyncProcessor::return_sink_stuff(apache::thrift::ContextStack* ctx, ::apache::thrift::SinkConsumer<::std::int32_t, ::std::int32_t>&& _return, folly::Executor::KeepAlive<> executor) {
  ProtocolOut_ prot;
  ::cpp2::B_sink_stuff_presult::FieldsType result;
  using SinkPResultType = ::cpp2::B_sink_stuff_presult::SinkPResultType;
  using FinalResponsePResultType = ::cpp2::B_sink_stuff_presult::FinalResponsePResultType;

  using ExMapType = apache::thrift::detail::ap::EmptyExMapType;

  auto sinkConsumerImpl = apache::thrift::detail::ap::toSinkConsumerImpl<
      ProtocolIn_,
      ProtocolOut_,
      SinkPResultType,
      FinalResponsePResultType,
      ExMapType>(
      std::move(_return),
      std::move(executor));

  return {serializeResponse("sink_stuff", &prot, ctx, result), std::move(sinkConsumerImpl)};
}

template <class ProtocolIn_, class ProtocolOut_>
void BAsyncProcessor::throw_wrapped_sink_stuff(apache::thrift::ResponseChannelRequest::UniquePtr req,FOLLY_MAYBE_UNUSED int32_t protoSeqId,apache::thrift::ContextStack* ctx,folly::exception_wrapper ew,apache::thrift::Cpp2RequestContext* reqCtx) {
  if (!ew) {
    return;
  }
  {
    apache::thrift::detail::ap::process_throw_wrapped_handler_error<ProtocolOut_>(
        ew, std::move(req), reqCtx, ctx, "sink_stuff");
    return;
  }
}


} // cpp2

namespace cpp2 {

typedef apache::thrift::ThriftPresult<false> C_I_interact_pargs;
typedef apache::thrift::ThriftPresult<true> C_I_interact_presult;
template <typename ProtocolIn_, typename ProtocolOut_>
void CAsyncProcessor::setUpAndProcess_I_interact(apache::thrift::ResponseChannelRequest::UniquePtr req, apache::thrift::SerializedCompressedRequest&& serializedRequest, apache::thrift::Cpp2RequestContext* ctx, folly::EventBase* eb, FOLLY_MAYBE_UNUSED apache::thrift::concurrency::ThreadManager* tm) {
  if (!setUpRequestProcessing(req, ctx, eb, tm, apache::thrift::RpcKind::SINGLE_REQUEST_SINGLE_RESPONSE, iface_, "I")) {
    return;
  }
  auto scope = iface_->getRequestExecutionScope(ctx, apache::thrift::concurrency::NORMAL);
  ctx->setRequestExecutionScope(std::move(scope));
  processInThread(std::move(req), std::move(serializedRequest), ctx, eb, tm, apache::thrift::RpcKind::SINGLE_REQUEST_SINGLE_RESPONSE, &CAsyncProcessor::executeRequest_I_interact<ProtocolIn_, ProtocolOut_>, this);
}

template <typename ProtocolIn_, typename ProtocolOut_>
void CAsyncProcessor::executeRequest_I_interact(apache::thrift::ServerRequest&& serverRequest) {
  auto tile = serverRequest.requestContext()->releaseTile();
  // make sure getRequestContext is null
  // so async calls don't accidentally use it
  iface_->setRequestContext(nullptr);
  ::cpp2::C_I_interact_pargs args;
  const auto* server = serverRequest.requestContext()->getConnectionContext()->getWorkerContext()->getServerContext();
  apache::thrift::ContextStack::UniquePtr ctxStack = apache::thrift::ContextStack::create(this->coalesceLegacyEventHandlersWith(server), this->getServiceName(), "C.I.interact", serverRequest.requestContext());
  auto& iface = static_cast<apache::thrift::ServiceHandler<C>::IIf&>(*tile);
  try {
    deserializeRequest<ProtocolIn_>(args, "I.interact", apache::thrift::detail::ServerRequestHelper::compressedRequest(std::move(serverRequest)).uncompress(), ctxStack.get());
  }
  catch (...) {
    folly::exception_wrapper ew(std::current_exception());
    apache::thrift::detail::ap::process_handle_exn_deserialization<ProtocolOut_>(
        ew
        , apache::thrift::detail::ServerRequestHelper::request(std::move(serverRequest))
        , serverRequest.requestContext()
        , apache::thrift::detail::ServerRequestHelper::eventBase(serverRequest)
        , "I.interact");
    return;
  }
  auto requestPileNotification = apache::thrift::detail::ServerRequestHelper::moveRequestPileNotification(serverRequest);
  auto concurrencyControllerNotification = apache::thrift::detail::ServerRequestHelper::moveConcurrencyControllerNotification(serverRequest);
  auto callback = std::make_unique<apache::thrift::HandlerCallback<void>>(
    apache::thrift::detail::ServerRequestHelper::request(std::move(serverRequest))
    , std::move(ctxStack)
    , return_I_interact<ProtocolIn_,ProtocolOut_>
    , throw_wrapped_I_interact<ProtocolIn_, ProtocolOut_>
    , serverRequest.requestContext()->getProtoSeqId()
    , apache::thrift::detail::ServerRequestHelper::eventBase(serverRequest)
    , apache::thrift::detail::ServerRequestHelper::executor(serverRequest)
    , serverRequest.requestContext()
    , requestPileNotification
    , concurrencyControllerNotification, std::move(serverRequest.requestData())
    , std::move(tile));
  iface.async_tm_interact(std::move(callback));
}

template <class ProtocolIn_, class ProtocolOut_>
apache::thrift::SerializedResponse CAsyncProcessor::return_I_interact(apache::thrift::ContextStack* ctx) {
  ProtocolOut_ prot;
  ::cpp2::C_I_interact_presult result;
  return serializeResponse("I.interact", &prot, ctx, result);
}

template <class ProtocolIn_, class ProtocolOut_>
void CAsyncProcessor::throw_wrapped_I_interact(apache::thrift::ResponseChannelRequest::UniquePtr req,FOLLY_MAYBE_UNUSED int32_t protoSeqId,apache::thrift::ContextStack* ctx,folly::exception_wrapper ew,apache::thrift::Cpp2RequestContext* reqCtx) {
  if (!ew) {
    return;
  }
  {
    apache::thrift::detail::ap::process_throw_wrapped_handler_error<ProtocolOut_>(
        ew, std::move(req), reqCtx, ctx, "I.interact");
    return;
  }
}

} // cpp2

