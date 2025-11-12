pub mod deployer;
pub fn node_code_wrapper(user_function: &str, path: &str) -> String {
    format!(
        r#"
const {{ NodeVM }} = require("vm2");

const code = `{user_function}`;

const vm = new NodeVM({{
  console: "inherit",
  sandbox: {{}},
  require: {{
    external: true,   // allow requiring npm modules
    builtin: ["os", "fs", "path"], // allow these builtins
  }},
}});

const handler = vm.run(code, "vm.js");

(async () => {{
  await handler(
    {{ path: "{path}" }},
    {{
      status: (code) => ({{
        succeed: (msg) => console.log("Status", code, ":", msg),
    }}),
    }}
  );
    }})();
"#,
        user_function = user_function,
        path = path
    )
}
