import ws from 'k6/ws';
import { check } from 'k6';
import { Counter } from 'k6/metrics';

export const options = {
    vus: 100,
    duration: '60s',
};

const streamMsgCounter = new Counter('received_stream_msgs');

export default function () {
    const url = 'ws://localhost:8080/ws';
    const params = { tags: { my_tag: 'my-ws-test' } };

    const res = ws.connect(url, params, function (socket) {
        socket.on('open', () => {
            socket.setInterval(() => {
                socket.send('ping from k6');
            }, 1000);
        });

        socket.on('message', (data) => {
            streamMsgCounter.add(data.length);
        });

        socket.on('close', () => {
        });

        socket.on('error', (e) => {
            console.log('An error occurred: ', e.error());
        });
    });

    check(res, { 'status is 101': (r) => r && r.status === 101 });
}