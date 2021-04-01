"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    Object.defineProperty(o, k2, { enumerable: true, get: function() { return m[k]; } });
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.createServer = void 0;
const express_1 = __importDefault(require("express"));
const path_1 = __importDefault(require("path"));
const OpenApiValidator = __importStar(require("express-openapi-validator"));
// import multer from 'multer'
async function createServer() {
    const server = express_1.default();
    // Placeholder for the form input
    server.get('/', function (_, res) {
        res.send('<html><head></head><body>\
               <form method="POST" enctype="multipart/form-data" action="/upload">\
                <input type="file" name="recfile"><br />\
                <input type="submit">\
              </form>\
            </body></html>');
        res.end();
    });
    const spec = path_1.default.join(__dirname, './../../api-spec/openapi.yaml');
    // TODO: localhost only?
    server.use('/spec', express_1.default.static(spec));
    // TODO: server swagger UI
    // const upload = multer({ dest: 'uploads/' })
    // server.post('/upload', upload.single('recfile'), (req, res, _) => {
    //   const file = req.file
    //   console.log(file)
    //   // if (!file) {
    //   //   const error = new Error('Please upload a file')
    //   //   error.httpStatusCode = 400
    //   //   return next(error)
    //   // }
    //     res.send(file)
    // })
    server.use(OpenApiValidator.middleware({
        apiSpec: spec,
        validateApiSpec: true,
        validateResponses: true,
        validateRequests: true,
        operationHandlers: {
            basePath: path_1.default.join(__dirname, './controllers'),
            resolver: OpenApiValidator.resolvers.modulePathResolver
        },
        fileUploader: { dest: './auto_uploads/' }
    }));
    //   server.use((err: any, req: express.Request, res: express.Response, next: express.NextFunction) => {
    //     res.status(err.status).json({
    //       error: {
    //         type: 'request_validation',
    //         message: err.message,
    //         errors: err.errors
    //       }
    //     })
    //   })
    // server.use((err: any, req: any, res: any, next: any) => {
    //     // format errors
    //     res.status(err.status || 500).json({
    //       message: err.message,
    //       errors: err.errors,
    //     });
    //   });
    return server;
}
exports.createServer = createServer;
