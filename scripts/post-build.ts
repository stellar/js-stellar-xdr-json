import { encodeBase64 } from "jsr:@std/encoding@1.0.5/base64";

// Rename the package to include the @stellar namespace.
{
    const file = "pkg/package.json";
    const input = Deno.readTextFileSync(file);
    const output = input.replaceAll(
        "stellar-xdr-json",
        "@stellar/stellar-xdr-json",
    );
    Deno.writeTextFileSync(file, output);
}

// Embed the wasm file into the js.
{
    const wasm = Deno.readFileSync("pkg/stellar_xdr_json_bg.wasm");
    const wasm_b64 = encodeBase64(wasm);

    const file = "pkg/stellar_xdr_json.js";
    const input = Deno.readTextFileSync(file);
    const output = input.replaceAll(
        "stellar_xdr_json_bg.wasm",
        `data:application/wasm;base64,${wasm_b64}`,
    );
    Deno.writeTextFileSync(file, output);
}

// Unexport the wasm initialization and run it automatically
{
    const file = "pkg/stellar_xdr_json.js";
    const input = Deno.readTextFileSync(file);
    const output = input.replaceAll(
        "export default __wbg_init;",
        "await __wbg_init();",
    );
    Deno.writeTextFileSync(file, output);
}
