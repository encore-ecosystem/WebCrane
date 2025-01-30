from upnpy.exceptions import ActionNotFoundError, SOAPError
from upnpy.ssdp import SSDPDevice

import upnpy
import socket

def get_internal_ip():
    s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    s.connect(("8.8.8.8", 80))
    internal_ip = s.getsockname()[0]
    s.close()

    return internal_ip


def find_device_with_port_mapping(services):
    for service in services:
        try:
            if hasattr(service, 'AddPortMapping') and hasattr(service, 'DeletePortMapping'):
                return service
        except ActionNotFoundError:
            continue
    return None


def add_port_mapping(port: int) -> SSDPDevice:
    internal_ip = get_internal_ip()
    upnp = upnpy.UPnP()
    upnp.discover()
    device = upnp.get_igd()
    print(f"UPnP uses {device}")
    service = find_device_with_port_mapping(device.get_services())
    service.AddPortMapping(
            NewRemoteHost='',
            NewExternalPort=port,
            NewProtocol='TCP',
            NewInternalPort=port,
            NewInternalClient=internal_ip,
            NewEnabled=1,
            NewPortMappingDescription='webcrane',
            NewLeaseDuration=0
    )

    return service


def remove_port_mapping(service: SSDPDevice, port: int) -> None:
    try:
        service.DeletePortMapping(
        NewRemoteHost='',
        NewExternalPort=port,
        NewProtocol='TCP',
    )
    except SOAPError:
        pass