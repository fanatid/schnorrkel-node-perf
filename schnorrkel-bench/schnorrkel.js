const assert = require("assert");
const crypto = require("crypto");
const { Suite } = require("benchmark");
const napi = require("../schnorrkel-napi");
const wasm = require("../schnorrkel-wasm/pkg");

function make_hash(input) {
  return crypto.createHash("sha256").update(input).digest();
}

function generate_seed() {
  let seed = "";
  while (seed.length < 32) {
    const byte = crypto.randomBytes(1)[0];
    if (
      ("0" >= byte && "9" <= byte) ||
      ("a" >= byte && "z" <= byte) ||
      ("A" >= byte && "Z" <= byte)
    ) {
      seed += String.fromCharCode(byte);
    }
  }
  return seed;
}

function generate_input() {
  let seed = process.env.SEED || generate_seed();
  console.log(`Input seed: ${seed}`);
  seed = make_hash(seed);

  return new Array(100).fill(null).map(() => {
    seed = make_hash(seed);
    const context_bytes = Buffer.from(seed);
    const context = napi.stateful.create_context(context_bytes);

    seed = make_hash(seed);
    const obj = napi.stateful.generate_pair(seed);
    const { seckey, seckey_bytes, pubkey, pubkey_bytes } = obj;

    const messages = new Array(10).fill(null).map(() => {
      seed = make_hash(seed);
      return Buffer.from(seed);
    });

    const signatures = [];
    const signatures_bytes = [];
    for (const message of messages) {
      const obj = napi.stateful.sign(context, seckey, pubkey, message);
      const { signature, signature_bytes } = obj;
      signatures.push(signature);
      signatures_bytes.push(signature_bytes);
    }

    return {
      context,
      context_bytes,

      seckey,
      seckey_bytes,

      pubkey,
      pubkey_bytes,

      messages,

      signatures,
      signatures_bytes,
    };
  });
}

const input = generate_input();
new Suite()
  .add("napi/stateful/sign", () => {
    for (let i = 0; i < 10; ++i) {
      const item = input[i];
      for (const message of item.messages) {
        napi.stateful.sign(item.context, item.seckey, item.pubkey, message);
      }
    }
  })
  .add("napi/stateful/verify", () => {
    for (let i = 0; i < 10; ++i) {
      const item = input[i];
      for (let j = 0; j < item.messages.length; ++j) {
        const message = item.messages[j];
        const signature = item.signatures[j];
        assert(
          napi.stateful.verify(item.context, item.pubkey, message, signature)
        );
      }
    }
  })
  .add("napi/stateless/sign", () => {
    for (const item of input) {
      napi.stateless.sign(
        item.context_bytes,
        item.seckey_bytes,
        item.pubkey_bytes,
        item.messages[0]
      );
    }
  })
  .add("napi/stateless/verify", () => {
    for (const item of input) {
      assert(
        napi.stateless.verify(
          item.context_bytes,
          item.pubkey_bytes,
          item.messages[0],
          item.signatures_bytes[0]
        )
      );
    }
  })
  .add("wasm/stateless/sign", () => {
    for (const item of input) {
      wasm.stateless_sign(
        item.context_bytes,
        item.seckey_bytes,
        item.pubkey_bytes,
        item.messages[0]
      );
    }
  })
  .add("wasm/stateless/verify", () => {
    for (const item of input) {
      assert(
        wasm.stateless_verify(
          item.context_bytes,
          item.pubkey_bytes,
          item.messages[0],
          item.signatures_bytes[0]
        )
      );
    }
  })
  .on("cycle", (event) => console.log(String(event.target)))
  .run();
