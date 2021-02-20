use neon::prelude::*;
use neon::register_module;
use schnorrkel::{signing_context, PublicKey, SecretKey, Signature, SIGNATURE_LENGTH};

fn stateless_sign(mut cx: FunctionContext) -> JsResult<JsObject> {
    let context_bytes = cx.argument::<JsBuffer>(0)?;
    let seckey_bytes = cx.argument::<JsBuffer>(1)?;
    let pubkey_bytes = cx.argument::<JsBuffer>(2)?;
    let message = cx.argument::<JsBuffer>(3)?;

    let context = cx.borrow(&context_bytes, |data| signing_context(data.as_slice()));
    let seckey = cx.borrow(&seckey_bytes, |data| {
        SecretKey::from_bytes(data.as_slice()).expect("valid seckey")
    });
    let pubkey = cx.borrow(&pubkey_bytes, |data| {
        PublicKey::from_bytes(data.as_slice()).expect("valid pubkey")
    });

    let signature = cx.borrow(&message, |data| {
        seckey.sign(context.bytes(data.as_slice()), &pubkey)
    });
    let result = JsObject::new(&mut cx);

    let mut signature_bytes = cx.buffer(SIGNATURE_LENGTH as u32)?;
    cx.borrow_mut(&mut signature_bytes, |data| {
        data.as_mut_slice().copy_from_slice(&signature.to_bytes());
    });
    result.set(&mut cx, "signature_bytes", signature_bytes)?;

    Ok(result)
}

fn stateless_verify(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let context_bytes = cx.argument::<JsBuffer>(0)?;
    let pubkey_bytes = cx.argument::<JsBuffer>(1)?;
    let message = cx.argument::<JsBuffer>(2)?;
    let signature_bytes = cx.argument::<JsBuffer>(3)?;

    let context = cx.borrow(&context_bytes, |data| signing_context(data.as_slice()));
    let pubkey = cx.borrow(&pubkey_bytes, |data| {
        PublicKey::from_bytes(data.as_slice()).expect("valid pubkey")
    });
    let signature = cx.borrow(&signature_bytes, |data| {
        Signature::from_bytes(data.as_slice()).expect("valid signature")
    });

    let is_valid = cx.borrow(&message, |data| {
        pubkey
            .verify(context.bytes(data.as_slice()), &signature)
            .is_ok()
    });
    Ok(cx.boolean(is_valid))
}

register_module!(mut m, {
    macro_rules! define_function {
        ($object:ident, $fn_name:expr, $fn:ident) => {
            let js_fn = JsFunction::new(&mut m, $fn)?;
            $object.set(&mut m, $fn_name, js_fn)?;
        };
    }

    // let stateful = JsObject::new(&mut m);
    // define_function!(stateful, "create_context", stateful_create_context);
    // define_function!(stateful, "create_seckey", stateful_create_seckey);
    // define_function!(stateful, "create_pubkey", stateful_create_pubkey);
    // define_function!(stateful, "create_signature", stateful_create_signature);
    // define_function!(stateful, "sign", stateful_sign);
    // define_function!(stateful, "verify", stateful_verify);
    // m.export_value("stateful", stateful)?;

    let stateless = JsObject::new(&mut m);
    define_function!(stateless, "sign", stateless_sign);
    define_function!(stateless, "verify", stateless_verify);
    m.export_value("stateless", stateless)?;

    Ok(())
});
