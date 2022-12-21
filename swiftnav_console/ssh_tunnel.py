# Copyright (c) 2022 Swift Navigation
#
# Permission is hereby granted, free of charge, to any person obtaining a copy of
# this software and associated documentation files (the "Software"), to deal in
# the Software without restriction, including without limitation the rights to
# use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
# the Software, and to permit persons to whom the Software is furnished to do so,
# subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
# FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
# COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
# IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
# CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

import argparse
import sshtunnel  # type: ignore

_default_tunnel_port = 22
_default_remote_bind_port = 55555


def validate(args: argparse.Namespace, parser: argparse.ArgumentParser):
    if not (args.ssh_remote_bind_address or args.ssh_tunnel):
        return

    if bool(args.ssh_remote_bind_address) != bool(args.ssh_tunnel):
        parser.error(
            f"""The --ssh-tunnel and --ssh-remote-bind-address options must be used together.
--ssh-tunnel option format:       [user]:[password]@example.com[:port] (default port {_default_tunnel_port})
--ssh-remote-bind-address format: example.com[:port] (default port {_default_remote_bind_port})
"""
        )

    e2_g1_str = "expected 2, got 1"
    try:
        try:
            (user_pw, host_port) = args.ssh_tunnel.split("@")
        except ValueError as e:
            if e2_g1_str not in str(e):
                raise
            (host_port,) = args.ssh_tunnel.split("@")
        try:
            (_, _) = user_pw.split(":")
        except ValueError as e:
            if e2_g1_str not in str(e):
                raise
        except UnboundLocalError:
            pass
        (_, _) = host_port.split(":")
    except ValueError as e:
        if e2_g1_str not in str(e):
            parser.error(
                f"""invalid --ssh-tunnel argument.
Please use format: [user]:[password]@example.com[:port] (default port {_default_tunnel_port})"""
            )

    try:
        (_, _) = args.ssh_remote_bind_address.split(":")
    except ValueError as e:
        if e2_g1_str not in str(e):
            parser.error(
                f"""invalid --ssh-remote-bind-address argument.
Please use format: example.com[:port] (default port {_default_remote_bind_port})"""
            )


def setup(tunnel_address: str, remote_bind_address: str):
    username_password = ""
    try:
        (username_password, host_port) = tunnel_address.split("@")
    except ValueError:
        host_port = tunnel_address
    password = None
    try:
        (username, password) = username_password.split(":")
    except ValueError:
        username = username_password
    port = str(_default_tunnel_port)
    try:
        (host, port) = host_port.split(":")
    except ValueError:
        host = host_port

    remote_bind_port = str(_default_remote_bind_port)
    try:
        (remote_bind_host, remote_bind_port) = remote_bind_address.split(":")
    except ValueError:
        remote_bind_host = remote_bind_address

    global sshtunnel_server  # pylint: disable=global-variable-undefined
    # To debug this, set `logger` parameter to `sshtunnel.create_logger(None, "DEBUG")`
    sshtunnel_server = sshtunnel.SSHTunnelForwarder(  # type: ignore
        (host, int(port)),
        ssh_username=username,
        ssh_password=password,
        local_bind_address=("127.0.0.1", int(remote_bind_port)),
        remote_bind_address=(remote_bind_host, int(remote_bind_port)),
        # logger=sshtunnel.create_logger(None, "DEBUG"),
    )
    sshtunnel_server.start()  # type: ignore
