"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const server_1 = require("./api/server");
server_1.createServer()
    .then(server => {
    server.listen(3000, () => {
        console.info(`Listening on http://localhost:3000`);
    });
})
    .catch(err => {
    console.error(`Error: ${err}`);
});
