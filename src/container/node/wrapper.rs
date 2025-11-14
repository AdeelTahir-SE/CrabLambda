pub fn node_code_wrapper(user_function: &str, port: &str) -> String {
    format!(
r#"
const http = require("http");
const {{ NodeVM }} = require("vm2");

const code = `{user_function}`;

const vm = new NodeVM({{
  console: "inherit",
  sandbox: {{}},
  require: {{
    external: true,
    builtin: ["os", "fs", "path", "http"],
  }},
}});

let userHandler;
try {{
  userHandler = vm.run(code, "vm.js");
  if (typeof userHandler !== "function") {{
    console.error("Error: module.exports must export a function");
    process.exit(1);
  }}
}} catch (err) {{
  console.error("VM init failed:", err);
  process.exit(1);
}}

const server = http.createServer(async (req, res) => {{
  let body = "";
  req.on("data", chunk => body += chunk);
  req.on("end", async () => {{
    const event = {{
      method: req.method,
      url: req.url,
      headers: req.headers,
      body: body
    }};
    const context = {{
      status: (code) => ({{
        succeed: (msg) => {{
          res.writeHead(code, {{ 'Content-Type': 'application/json' }});
          res.end(JSON.stringify({{ success: true, data: msg }}));
        }},
        fail: (err) => {{
          res.writeHead(code, {{ 'Content-Type': 'application/json' }});
          res.end(JSON.stringify({{ success: false, error: err.toString() }}));
        }}
      }})
    }};

    try {{
      const result = await userHandler(event, context);
      if (!res.writableEnded && result !== undefined) {{
        res.writeHead(200, {{ 'Content-Type': 'application/json' }});
        res.end(JSON.stringify({{ success: true, data: result }}));
      }}
    }} catch (err) {{
      if (!res.writableEnded) {{
        res.writeHead(500, {{ 'Content-Type': 'application/json' }});
        res.end(JSON.stringify({{ success: false, error: err.toString() }}));
      }}
    }}
  }});
}});

server.listen({port}, () => console.log("Listening on port {port}"));
"#,
        user_function = user_function,
        port = port
    )
}
