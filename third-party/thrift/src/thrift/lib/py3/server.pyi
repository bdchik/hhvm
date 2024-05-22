#!/usr/bin/env python3
# Copyright (c) Meta Platforms, Inc. and affiliates.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

import ipaddress
import os
import pathlib
from asyncio import Future
from enum import Enum
from types import TracebackType
from typing import (
    Any,
    Callable,
    ClassVar,
    Mapping,
    NamedTuple,
    Optional,
    overload,
    Type,
    TypeVar,
    Union,
)

from thrift.py3.common import Headers, Priority

# pyre-ignore[33]: the callable can be any function
mT = TypeVar("mT", bound=Callable[..., Any])
T = TypeVar("T")
IPAddress = Union[ipaddress.IPv4Address, ipaddress.IPv6Address]
# pyre-fixme[24]: Generic type `os.PathLike` expects 1 type parameter.
Path = Union[str, bytes, os.PathLike]

class SocketAddress(NamedTuple):
    ip: Optional[IPAddress]
    port: Optional[int]
    path: Optional[pathlib.Path]

@overload
def get_context() -> RequestContext: ...
@overload
def get_context(default: T) -> Union[T, RequestContext]: ...

class SSLPolicy(Enum):
    DISABLED: SSLPolicy = ...
    PERMITTED: SSLPolicy = ...
    REQUIRED: SSLPolicy = ...
    @property
    def value(self) -> int: ...

class AsyncProcessorFactory:
    annotations: ClassVar[Mapping[str, str]] = ...
    # pyre-ignore[3]: it can return anything
    async def __aenter__(self) -> Any: ...
    async def __aexit__(
        self,
        exc_type: Optional[Type[BaseException]],
        exc_value: Optional[BaseException],
        traceback: Optional[TracebackType],
    ) -> Optional[bool]: ...
    async def onStartServing(self) -> None: ...
    async def onStopRequested(self) -> None: ...

class ServiceInterface(AsyncProcessorFactory):
    pass

hT = TypeVar("hT", bound=AsyncProcessorFactory)

def getServiceName(svc: hT) -> str: ...

class ThriftServer:
    def __init__(
        self,
        handler: Optional[hT],
        port: int = 0,
        ip: Optional[Union[IPAddress, str]] = None,
        path: Optional[Path] = None,
        socket_fd: Optional[int] = None,
    ) -> None: ...
    async def serve(self) -> None: ...
    def get_ssl_policy(self) -> SSLPolicy: ...
    def set_ssl_policy(self, policy: SSLPolicy) -> None: ...
    def stop(self) -> None: ...
    def stop_listening(self) -> None: ...
    def get_address(self) -> Future[SocketAddress]: ...
    def get_active_requests(self) -> int: ...
    def get_max_requests(self) -> int: ...
    def set_max_requests(self, max_requests: int) -> None: ...
    def get_max_connections(self) -> int: ...
    def set_max_connections(self, max_connections: int) -> None: ...
    def get_listen_backlog(self) -> int: ...
    def set_listen_backlog(self, listen_backlog: int) -> None: ...
    def set_io_worker_threads(self, num: int) -> None: ...
    def get_io_worker_threads(self) -> int: ...
    def get_cpu_worker_threads(self) -> int: ...
    def set_workers_join_timeout(self, seconds: int) -> None: ...
    def set_allow_plaintext_on_loopback(self, enabled: bool) -> None: ...
    def is_plaintext_allowed_on_loopback(self) -> bool: ...
    def set_idle_timeout(self, seconds: float) -> None: ...
    def get_idle_timeout(self) -> float: ...
    def set_queue_timeout(self, seconds: float) -> None: ...
    def get_queue_timeout(self) -> float: ...
    def use_existing_socket(self, socket: int) -> None: ...
    def set_language_framework_name(self, name: str) -> None: ...
    def set_stop_workers_on_stop_listening(self, stop_workers: bool) -> None: ...
    def get_stop_workers_on_stop_listening(self) -> bool: ...
    def set_idle_server_timeout(self, seconds: float) -> None: ...
    def get_quick_exit_on_shutdown_timeout(self) -> bool: ...
    def set_quick_exit_on_shutdown_timeout(
        self, quick_exit_on_shutdown_timeout: bool
    ) -> None: ...
    def disable_info_logging(self) -> None: ...

class ReadHeaders(Headers): ...
class WriteHeaders(Headers): ...

class ConnectionContext:
    local_address: SocketAddress
    peer_address: SocketAddress
    peer_common_name: str
    security_protocol: str
    peer_certificate: bytes
    peer_certificate_identity: str
    client_metadata: ClientMetadata

class RequestContext:
    connection_context: ConnectionContext
    @property
    def priority(self) -> Priority: ...
    @property
    def read_headers(self) -> ReadHeaders: ...
    @property
    def write_headers(self) -> WriteHeaders: ...
    def set_header(self, key: str, value: str) -> None: ...
    method_name: str
    request_id: str
    request_timeout: float

class ClientMetadata:
    agent: str
    hostname: str
    def getMetadataField(self, key: str) -> str: ...
