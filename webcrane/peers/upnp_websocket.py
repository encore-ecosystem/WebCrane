import websockets
from upnpy.ssdp import SSDPDevice

from webcrane.peers.upnp import add_port_mapping, remove_port_mapping


class UPnPWebSocket:
    def __init__(self, ws_handler, host, port, **kwargs):
        self.ws_handler = ws_handler
        self.host = host
        self.port = port
        self.kwargs = kwargs

    async def __aenter__(self):
        self.server = await websockets.serve(
            self.ws_handler, self.host, self.port, **self.kwargs
        )
        self.upnp_device = add_port_mapping(self.port)
        return self.server

    async def __aexit__(self, exc_type, exc_val, exc_tb):
        self.server.close()
        await self.server.wait_closed()
        remove_port_mapping(self.upnp_device, self.port)


__all__ = ['UPnPWebSocket']
