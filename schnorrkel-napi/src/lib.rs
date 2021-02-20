#[macro_use]
extern crate napi_derive;

use napi::{CallContext, Env, JsBoolean, JsBuffer, JsObject, Result as NapiResult};
use schnorrkel::{
    context::SigningContext, signing_context, ExpansionMode, MiniSecretKey, PublicKey, SecretKey,
    Signature,
};

#[module_exports]
fn init(mut exports: JsObject, env: Env) -> NapiResult<()> {
    let mut stateful = env.create_object()?;
    stateful.create_named_method("create_context", create_context)?;
    stateful.create_named_method("generate_pair", generate_pair)?;
    stateful.create_named_method("sign", stateful_sign)?;
    stateful.create_named_method("verify", stateful_verify)?;
    exports.set_named_property("stateful", stateful)?;

    let mut stateless = env.create_object()?;
    stateless.create_named_method("sign", stateless_sign)?;
    stateless.create_named_method("verify", stateless_verify)?;
    exports.set_named_property("stateless", stateless)?;

    Ok(())
}

#[js_function(1)]
fn create_context(ctx: CallContext) -> NapiResult<JsObject> {
    let context_bytes = ctx.get::<JsBuffer>(0)?.into_value()?;
    let context = signing_context(&context_bytes);

    let mut obj = ctx.env.create_object()?;
    ctx.env.wrap(&mut obj, context)?;
    Ok(obj)
}

#[js_function(1)]
fn generate_pair(ctx: CallContext) -> NapiResult<JsObject> {
    let seed = ctx.get::<JsBuffer>(0)?.into_value()?;
    let mini_seckey = MiniSecretKey::from_bytes(&seed).expect("valid seed for MiniSecretKey");
    let keypair = mini_seckey.expand_to_keypair(ExpansionMode::Uniform);
    let secret = keypair.secret.clone();
    let public = keypair.public;

    let mut result = ctx.env.create_object()?;

    let seckey_bytes = ctx.env.create_buffer_copy(secret.to_bytes())?.into_raw();
    result.set_named_property("seckey_bytes", seckey_bytes)?;

    let mut obj = ctx.env.create_object()?;
    ctx.env.wrap(&mut obj, secret)?;
    result.set_named_property("seckey", obj)?;

    let pubkey_bytes = ctx.env.create_buffer_copy(public.to_bytes())?.into_raw();
    result.set_named_property("pubkey_bytes", pubkey_bytes)?;

    let mut obj = ctx.env.create_object()?;
    ctx.env.wrap(&mut obj, public)?;
    result.set_named_property("pubkey", obj)?;

    Ok(result)
}

#[js_function(4)]
fn stateful_sign(ctx: CallContext) -> NapiResult<JsObject> {
    let context = ctx.env.unwrap::<SigningContext>(&ctx.get::<JsObject>(0)?)?;
    let seckey = ctx.env.unwrap::<SecretKey>(&ctx.get::<JsObject>(1)?)?;
    let pubkey = ctx.env.unwrap::<PublicKey>(&ctx.get::<JsObject>(2)?)?;
    let message = ctx.get::<JsBuffer>(3)?.into_value()?;

    let signature = seckey.sign(context.bytes(&message), &pubkey);
    signature2object(ctx.env, signature)
}

#[js_function(4)]
fn stateful_verify(ctx: CallContext) -> NapiResult<JsBoolean> {
    let context = ctx.env.unwrap::<SigningContext>(&ctx.get::<JsObject>(0)?)?;
    let pubkey = ctx.env.unwrap::<PublicKey>(&ctx.get::<JsObject>(1)?)?;
    let message = ctx.get::<JsBuffer>(2)?.into_value()?;
    let signature = ctx.env.unwrap::<Signature>(&ctx.get::<JsObject>(3)?)?;

    let is_valid = pubkey.verify(context.bytes(&message), &signature).is_ok();
    ctx.env.get_boolean(is_valid)
}

#[js_function(4)]
fn stateless_sign(ctx: CallContext) -> NapiResult<JsObject> {
    let context_bytes = ctx.get::<JsBuffer>(0)?.into_value()?;
    let seckey_bytes = ctx.get::<JsBuffer>(1)?.into_value()?;
    let pubkey_bytes = ctx.get::<JsBuffer>(2)?.into_value()?;
    let message = ctx.get::<JsBuffer>(3)?.into_value()?;

    let context = signing_context(&context_bytes);
    let seckey = SecretKey::from_bytes(&seckey_bytes).expect("valid seckey");
    let pubkey = PublicKey::from_bytes(&pubkey_bytes).expect("valid pubkey");

    let signature = seckey.sign(context.bytes(&message), &pubkey);
    signature2object(ctx.env, signature)
}

#[js_function(4)]
fn stateless_verify(ctx: CallContext) -> NapiResult<JsBoolean> {
    let context_bytes = ctx.get::<JsBuffer>(0)?.into_value()?;
    let pubkey_bytes = ctx.get::<JsBuffer>(1)?.into_value()?;
    let message = ctx.get::<JsBuffer>(2)?.into_value()?;
    let signature_bytes = ctx.get::<JsBuffer>(3)?.into_value()?;

    let context = signing_context(&context_bytes);
    let pubkey = PublicKey::from_bytes(&pubkey_bytes).expect("valid pubkey");
    let signature = Signature::from_bytes(&signature_bytes).expect("valid signature");

    let is_valid = pubkey.verify(context.bytes(&message), &signature).is_ok();
    ctx.env.get_boolean(is_valid)
}

fn signature2object(env: &Env, signature: Signature) -> NapiResult<JsObject> {
    let mut result = env.create_object()?;

    let signature_bytes = env.create_buffer_copy(signature.to_bytes())?.into_raw();
    result.set_named_property("signature_bytes", signature_bytes)?;

    let mut obj = env.create_object()?;
    env.wrap(&mut obj, signature)?;
    result.set_named_property("signature", obj)?;

    Ok(result)
}
