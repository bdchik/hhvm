/**
 * Autogenerated by Thrift
 *
 * DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
 *  @generated
 */

#include <thrift/compiler/test/fixtures/single_file_service/gen-py3/module/clients_wrapper.h>

namespace cpp2 {


folly::Future<::cpp2::Foo>
AClientWrapper::foo(
    apache::thrift::RpcOptions& rpcOptions) {
  auto* client = static_cast<::cpp2::AAsyncClient*>(async_client_.get());
  folly::Promise<::cpp2::Foo> _promise;
  auto _future = _promise.getFuture();
  auto callback = std::make_unique<::thrift::py3::FutureCallback<::cpp2::Foo>>(
    std::move(_promise), rpcOptions, client->recv_wrapped_foo, channel_);
  try {
    client->foo(
      rpcOptions,
      std::move(callback)
    );
  } catch (...) {
    return folly::makeFuture<::cpp2::Foo>(folly::exception_wrapper(
      std::current_exception()
    ));
  }
  return _future;
}


folly::Future<std::unique_ptr<::thrift::py3::ClientWrapper>>
AClientWrapper::createI() {
  return folly::via(
      channel_->getEventBase(),
      [=]() -> std::unique_ptr<::thrift::py3::ClientWrapper> {
        auto interaction_client = static_cast<std::unique_ptr<apache::thrift::GeneratedAsyncClient>>(std::make_unique<AAsyncClient::I>(((AAsyncClient*)async_client_.get())->createI()));
        return static_cast<std::unique_ptr<::thrift::py3::ClientWrapper>>(std::make_unique<IInteractionWrapper>(std::move(interaction_client), channel_));
      }
  );
}

folly::Future<folly::Unit>
AClientWrapper::IInteractionWrapper::interact(
    apache::thrift::RpcOptions& rpcOptions) {
  auto* client = static_cast<::cpp2::AAsyncClient::I*>(async_client_.get());
  folly::Promise<folly::Unit> _promise;
  auto _future = _promise.getFuture();
  auto callback = std::make_unique<::thrift::py3::FutureCallback<folly::Unit>>(
    std::move(_promise), rpcOptions, client->recv_wrapped_interact, channel_);
  try {
    client->interact(
      rpcOptions,
      std::move(callback)
    );
  } catch (...) {
    return folly::makeFuture<folly::Unit>(folly::exception_wrapper(
      std::current_exception()
    ));
  }
  return _future;
}

folly::Future<folly::Unit>
BClientWrapper::bar(
    apache::thrift::RpcOptions& rpcOptions,
    ::cpp2::Foo arg_foo) {
  auto* client = static_cast<::cpp2::BAsyncClient*>(async_client_.get());
  folly::Promise<folly::Unit> _promise;
  auto _future = _promise.getFuture();
  auto callback = std::make_unique<::thrift::py3::FutureCallback<folly::Unit>>(
    std::move(_promise), rpcOptions, client->recv_wrapped_bar, channel_);
  try {
    client->bar(
      rpcOptions,
      std::move(callback),
      arg_foo
    );
  } catch (...) {
    return folly::makeFuture<folly::Unit>(folly::exception_wrapper(
      std::current_exception()
    ));
  }
  return _future;
}

folly::Future<apache::thrift::ClientBufferedStream<int32_t>>
BClientWrapper::stream_stuff(
    apache::thrift::RpcOptions& rpcOptions) {
  auto* client = static_cast<::cpp2::BAsyncClient*>(async_client_.get());
  folly::Promise<apache::thrift::ClientBufferedStream<int32_t>> _promise;
  auto _future = _promise.getFuture();
  auto callback = std::make_unique<::thrift::py3::FutureCallback<apache::thrift::ClientBufferedStream<int32_t>>>(
    std::move(_promise), rpcOptions, client->recv_wrapped_stream_stuff, channel_);
  try {
    client->stream_stuff(
      rpcOptions,
      std::move(callback)
    );
  } catch (...) {
    return folly::makeFuture<apache::thrift::ClientBufferedStream<int32_t>>(folly::exception_wrapper(
      std::current_exception()
    ));
  }
  return _future;
}


folly::Future<std::unique_ptr<::thrift::py3::ClientWrapper>>
CClientWrapper::createI() {
  return folly::via(
      channel_->getEventBase(),
      [=]() -> std::unique_ptr<::thrift::py3::ClientWrapper> {
        auto interaction_client = static_cast<std::unique_ptr<apache::thrift::GeneratedAsyncClient>>(std::make_unique<CAsyncClient::I>(((CAsyncClient*)async_client_.get())->createI()));
        return static_cast<std::unique_ptr<::thrift::py3::ClientWrapper>>(std::make_unique<IInteractionWrapper>(std::move(interaction_client), channel_));
      }
  );
}

folly::Future<folly::Unit>
CClientWrapper::IInteractionWrapper::interact(
    apache::thrift::RpcOptions& rpcOptions) {
  auto* client = static_cast<::cpp2::CAsyncClient::I*>(async_client_.get());
  folly::Promise<folly::Unit> _promise;
  auto _future = _promise.getFuture();
  auto callback = std::make_unique<::thrift::py3::FutureCallback<folly::Unit>>(
    std::move(_promise), rpcOptions, client->recv_wrapped_interact, channel_);
  try {
    client->interact(
      rpcOptions,
      std::move(callback)
    );
  } catch (...) {
    return folly::makeFuture<folly::Unit>(folly::exception_wrapper(
      std::current_exception()
    ));
  }
  return _future;
}

} // namespace cpp2
