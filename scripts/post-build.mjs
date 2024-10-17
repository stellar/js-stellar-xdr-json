import fs from "node:fs";

// Rename the package to include the @stellar namespace.
{
    const file = "pkg/package.json";
    const input = fs.readFileSync(file, "utf8");
    const output = input.replaceAll(
        "stellar-xdr-json",
        "@stellar/stellar-xdr-json",
    );
    fs.writeFileSync(file, output);
}

// Embed the wasm file into the js.
{
    const wasm = fs.readFileSync("pkg/stellar_xdr_json_bg.wasm");
    const wasm_b64 = wasm.toString("base64");

    const file = "pkg/stellar_xdr_json.js";
    const input = fs.readFileSync(file, "utf8");
    const output = input.replaceAll(
        "stellar_xdr_json_bg.wasm",
        `data:application/wasm;base64,${wasm_b64}`,
    );
    fs.writeFileSync(file, output);
}

// Unexport the wasm initialization and run it automatically
{
    const file = "pkg/stellar_xdr_json.js";
    const input = fs.readFileSync(file, "utf8");
    const output = input.replaceAll(
        "export default __wbg_init;",
        "await __wbg_init();",
    );
    fs.writeFileSync(file, output);
}
