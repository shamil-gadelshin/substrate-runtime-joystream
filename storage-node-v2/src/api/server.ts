import express from 'express'
import path from 'path'
import {Express} from 'express-serve-static-core'
// import * as OpenApiValidator from 'express-openapi-validator'
import multer from 'multer'

export async function createServer(): Promise<Express> {
  const server = express()

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

  const spec = path.join(__dirname, './../../api-spec/openapi.yaml');
  // TODO: localhost only?
  server.use('/spec', express.static(spec));
  // TODO: server swagger UI

  const upload = multer({ dest: 'uploads/' })

  server.post('/upload', upload.single('recfile'), (req, res, _) => {
    const file = req.file
    console.log(file)
    // if (!file) {
    //   const error = new Error('Please upload a file')
    //   error.httpStatusCode = 400
    //   return next(error)
    // }
      res.send(file)
  })

  // server.use(
  //   OpenApiValidator.middleware({
  //     apiSpec: spec,
  //     validateApiSpec: true,
  //     validateResponses: true,
  //     validateRequests: true,
  //     operationHandlers: {
  //         basePath: path.join(__dirname, './controllers'),
  //         resolver: OpenApiValidator.resolvers.modulePathResolver
  //     }
  //   }),
  // );

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

  return server
}
