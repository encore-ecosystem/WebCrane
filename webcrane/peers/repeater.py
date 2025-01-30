import asyncio
from multiprocessing import Process

from collections import deque
from multiprocessing import Event
from webcrane.peers.peer import Peer
from webcrane.src.packages import *
from webcrane.src.rooms import *
from webcrane.peers.upnp import get_internal_ip
from webcrane.peers.upnp_websocket import UPnPWebSocket

import websockets


class RepeaterPeer(Peer):
    rooms = Rooms()

    async def run(self, start_bootstrap: Event, port = 8765):
        ip = get_internal_ip()
        async with UPnPWebSocket(self.bootstrap, ip, port):
            print('Bootstrap started')
            start_bootstrap.set()
            await asyncio.get_running_loop().create_future()
            print('Bootstrap stopped')

    async def bootstrap(self, websocket: websockets.WebSocketServerProtocol):
        role = await self.recv(websocket)
        match role.data.get('role', 'none'):
            case 'pull':
                await asyncio.gather(self.handle_pull(websocket), self.keepalive(websocket))

            case 'push':
                await asyncio.gather(self.handle_push(websocket), self.keepalive(websocket))

            case _:
                await websocket.close(reason=f"Error to fetch role {role}")

    @staticmethod
    async def keepalive(websocket: websockets.WebSocketServerProtocol):
        while websocket.open:
            await asyncio.sleep(1)
            await websocket.keepalive_ping()

    async def handle_push(self, websocket: websockets.WebSocketServerProtocol):
        addr = websocket.remote_address[:2]
        # cprint(f"[{addr}]: Connected to the room", color='red')

        # print(f"[PUB {addr}]: Receiving project package")
        project_package = await self.recv(websocket)
        project_name = project_package.data['project_name']

        # print(f"[PUB {addr}]: Creating room")
        self.rooms.remove_room(room_name=project_name)
        self.rooms.create_room(room_name=project_name)

        # print(f"[PUB {addr}]: Entering to console")
        while True:
            package = await self.recv(websocket)
            if isinstance(package, RefreshPackage):
                await self.send(websocket, package_chunk_generator(RefreshPackage(self.rooms.get_addresses(project_name))))
            elif isinstance(package, CompletePackage):
                break
            else:
                print(f'Unexpected package: {package}')

        print(f"[PUB {addr}]: Locking room")
        self.rooms.lock(room_name=project_name)

        print(f"[PUB {addr}]: Transmitting hashes")
        await self.shared_send_from_generator(project_name, self.recv_from_generator(websocket))
        self.rooms.add_send_request(project_name, SendType.CLOSE, [])

        print(f"[PUB {addr}]: Waiting to receive all missed packages")
        while self.rooms.get_status(project_name) != self.rooms.get_num_of_subs(project_name):
            await asyncio.sleep(0.001)

        print(f"[PUB {addr}]: Send missed files")
        missing_files = set()
        for miss in self.rooms.get_missing_files(room_name=project_name):
            missing_files |= miss
        await self.send(websocket, package_chunk_generator(MissingFiles(missing_files)))
        # self.rooms.add_send_request(project_name, SendType.CLOSE, [])

        print(f"[PUB {addr}]: Transmit chunks")
        for _ in range(len(missing_files)):
            await self.shared_send_from_generator(project_name, self.recv_from_generator(websocket))

        self.rooms.add_send_request(project_name, SendType.CLOSE, [])

        while project_name in self.rooms.rooms:
            await asyncio.sleep(0.05)

        await websocket.close()
        print(f"[PUB {addr}]: Closed")

    async def handle_pull(self, websocket: websockets.WebSocketServerProtocol):
        addr = websocket.remote_address[:2]

        print(f"[SUB {addr}]: Receiving project name")
        project_package = await self.recv(websocket)
        project_name = project_package.data['project_name']

        print(f"[SUB {addr}]: Waiting for pusher")
        while project_name not in self.rooms.rooms:
            await asyncio.sleep(0.05)

        print(f"[SUB {addr}]: Append user")
        self.rooms.add_sub(room_name=project_name, sub_address=addr, sub_websocket=websocket)

        print(f"[SUB {addr}]: Transmitting hashes")
        buffer: deque = self.rooms.buffer[project_name]
        while True:
            while len(buffer) == 0:
                await asyncio.sleep(0.05)
            buffer[0][2] -= 1
            if buffer[0][2] == 0:
                sending_request = buffer.popleft()
            else:
                sending_request = buffer[0]
            send_type: SendType = sending_request[0]
            match send_type:
                case SendType.CLOSE:
                    break
                case SendType.PACKAGE:
                    await self.send(websocket, sending_request[1])
                case SendType.GENERATOR:
                    await self.send_from_generator(websocket, sending_request[1])

        print(f"[SUB {addr}]: Receiving missed")
        missing_file_package = await self.recv(websocket)
        missing_files = missing_file_package.data['missing_files']

        self.rooms.increment_status(room_name=project_name)
        self.rooms.add_missed_files(room_name=project_name, sub_address=addr, missed_files=missing_files)

        print(f"[SUB {addr}]: Transmitting chunks")
        while True:
            while len(buffer) == 0:
                await asyncio.sleep(0.05)
            buffer[0][2] -= 1
            if buffer[0][2] == 0:
                sending_request = buffer.popleft()
            else:
                sending_request = buffer[0]
            send_type: SendType = sending_request[0]
            match send_type:
                case SendType.CLOSE:
                    break
                case SendType.PACKAGE:
                    await self.send(websocket, sending_request[1])
                case SendType.GENERATOR:
                    await self.send_from_generator(websocket, sending_request[1])

        print(f"[SUB {addr}]: Close connection")
        await websocket.close()
        self.rooms.remove_sub(room_name=project_name, sub_address=addr)

    async def shared_send(self, room: str, chunk_generator):
        self.rooms.add_send_request(room_name=room, send_type=SendType.PACKAGE, generator=chunk_generator)

    async def shared_send_from_generator(self, room: str, generator):
        self.rooms.add_send_request(room_name=room, send_type=SendType.GENERATOR, generator=generator)


async def async_run_repeater(start_bootstrap, stop_bootstrap):
    asyncio.create_task(RepeaterPeer().run(start_bootstrap))
    while not stop_bootstrap.is_set():
        await asyncio.sleep(0.1)


def run_repeater(start_bootstrap: Event, stop_bootstrap):
    asyncio.run(async_run_repeater(start_bootstrap, stop_bootstrap))


async def push():
    start_bootstrap = Event()
    stop_bootstrap = Event()
    proc = Process(target=run_repeater, args=(start_bootstrap,stop_bootstrap))
    proc.start()

    while not start_bootstrap.is_set():
        await asyncio.sleep(0.1)
    await Peer().push(stop_bootstrap)


__all__ = ['RepeaterPeer', 'push']
