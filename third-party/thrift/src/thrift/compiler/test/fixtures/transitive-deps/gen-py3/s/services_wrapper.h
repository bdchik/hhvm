/**
 * Autogenerated by Thrift
 *
 * DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
 *  @generated
 */

#pragma once
#if __has_include(<thrift/compiler/test/fixtures/transitive-deps/gen-cpp2/TestService.h>)
#include <thrift/compiler/test/fixtures/transitive-deps/gen-cpp2/TestService.h>
#else
#include <thrift/compiler/test/fixtures/transitive-deps/gen-cpp2/s_handlers.h>
#endif
#include <folly/python/futures.h>
#include <Python.h>

#include <memory>

namespace cpp2 {

class TestServiceWrapper : virtual public TestServiceSvIf {
  protected:
    PyObject *if_object;
    folly::Executor *executor;
  public:
    explicit TestServiceWrapper(PyObject *if_object, folly::Executor *exc);
    void async_tm_test(std::unique_ptr<apache::thrift::HandlerCallback<void>> callback) override;
folly::SemiFuture<folly::Unit> semifuture_onStartServing() override;
folly::SemiFuture<folly::Unit> semifuture_onStopRequested() override;
};

std::shared_ptr<apache::thrift::ServerInterface> TestServiceInterface(PyObject *if_object, folly::Executor *exc);
} // namespace cpp2
