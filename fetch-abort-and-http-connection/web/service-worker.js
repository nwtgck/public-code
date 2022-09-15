self.addEventListener('activate', (e) => {
  e.waitUntil(self.clients.claim());
});
// self.addEventListener('message', (e) => {
//     if (!e.data) {
//         return;
//     }
//     switch (e.data.type) {
//         case 'skip-waiting':
//             self.skipWaiting();
//             break;
//         case 'enroll-download': {
//             const { headers, readableStream } = e.data;
//             if (!isHeaders(headers)) {
//                 console.error('data.headers is invalid');
//                 return;
//             }
//             if (!(readableStream instanceof ReadableStream)) {
//                 console.error('data.readableStream is not ReadableStream');
//                 return;
//             }
//             const id = generateUniqueSwDownloadId();
//             idToSwDownload.set(id, {
//                 headers,
//                 readableStream,
//             });
//             e.ports[0].postMessage({
//                 swDownloadId: id,
//             });
//             break;
//         }
//         case 'enroll-download-with-channel': {
//             const { headers } = e.data;
//             if (!isHeaders(headers)) {
//                 console.error('data.headers is invalid');
//                 return;
//             }
//             const readableStream = new ReadableStream({
//                 start(ctrl) {
//                     e.ports[0].onmessage = (ev) => {
//                         if (ev.data.done) {
//                             ctrl.close();
//                             return;
//                         }
//                         ctrl.enqueue(ev.data.value);
//                     };
//                 }
//             });
//             const id = generateUniqueSwDownloadId();
//             idToSwDownload.set(id, {
//                 headers,
//                 readableStream,
//             });
//             e.ports[0].postMessage({
//                 swDownloadId: id,
//             });
//             break;
//         }
//         default:
//             break;
//     }
// });
self.addEventListener('fetch', (event) => {
    // const url = new URL(event.request.url);
    // if (url.pathname === '/sw-download-support/v2') {
    //     event.respondWith(new Response(new ReadableStream({
    //         start(controller) {
    //             controller.enqueue(new Uint8Array([79, 75]));
    //             controller.close();
    //         }
    //     })));
    // }
    // else if (url.pathname === '/sw-download/v2') {
    //     const fragmentQuery = new URL(`a://a${url.hash.substring(1)}`).searchParams;
    //     const id = fragmentQuery.get("id");
    //     if (id === null) {
    //         console.error("id not found", url);
    //         return;
    //     }
    //     const swDownload = idToSwDownload.get(id);
    //     if (swDownload === undefined) {
    //         console.error(`download ID ${id} not found`);
    //         return;
    //     }
    //     idToSwDownload.delete(id);
    //     const headers = new Headers(swDownload.headers);
    //     event.respondWith(new Response(swDownload.readableStream, {
    //         headers,
    //     }));
    // }
});
