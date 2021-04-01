"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.upload = void 0;
function upload(req, res) {
    res.json({
        originalname: req.file.originalname,
        encoding: req.file.encoding,
        mimetype: req.file.mimetype,
        // Buffer of file conents
        buffer: req.file.buffer,
    });
}
exports.upload = upload;
