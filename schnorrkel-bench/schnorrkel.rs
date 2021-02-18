use criterion::{criterion_group, criterion_main, Criterion};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use schnorrkel::{
    context::SigningContext, signing_context, ExpansionMode, MiniSecretKey, PublicKey, SecretKey,
    Signature, PUBLIC_KEY_LENGTH, SECRET_KEY_LENGTH, SIGNATURE_LENGTH,
};
use sha2::{Digest, Sha256};

struct Input {
    context: SigningContext,
    context_bytes: [u8; 32],

    seckey: SecretKey,
    seckey_bytes: [u8; SECRET_KEY_LENGTH],

    pubkey: PublicKey,
    pubkey_bytes: [u8; PUBLIC_KEY_LENGTH],

    messages: Vec<[u8; 32]>,

    signatures: Vec<Signature>,
    signatures_bytes: Vec<[u8; SIGNATURE_LENGTH]>,
}

fn make_hash(bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finalize().into()
}

fn generate_input() -> Vec<Input> {
    let seed = match std::env::var("SEED") {
        Ok(seed) => seed,
        Err(std::env::VarError::NotPresent) => {
            let mut rng = thread_rng();
            std::iter::repeat(())
                .take(32)
                .map(|()| rng.sample(Alphanumeric))
                .map(char::from)
                .collect()
        }
        Err(error) => panic!("Failed to get seed: {}", error),
    };
    println!("Input seed: {}", seed);
    let seed = make_hash(seed.as_bytes());

    std::iter::repeat_with(|| {
        let seed = make_hash(&seed);
        let context_bytes = seed.clone();
        let context = signing_context(&context_bytes);

        let seed = make_hash(&seed);
        let mini_seckey = MiniSecretKey::from_bytes(&seed).unwrap();
        let keypair = mini_seckey.expand_to_keypair(ExpansionMode::Uniform);

        let messages = std::iter::repeat_with(|| {
            let seed = make_hash(&seed);
            seed.clone()
        })
        .take(10)
        .collect::<Vec<[u8; 32]>>();

        let signatures = messages
            .iter()
            .map(|message| keypair.secret.sign(context.bytes(message), &keypair.public))
            .collect::<Vec<Signature>>();
        let signatures_bytes = signatures
            .iter()
            .map(|signature| signature.to_bytes())
            .collect();

        Input {
            context,
            context_bytes,

            seckey: keypair.secret.clone(),
            seckey_bytes: keypair.secret.to_bytes(),

            pubkey: keypair.public.clone(),
            pubkey_bytes: keypair.public.to_bytes(),

            messages,

            signatures,
            signatures_bytes,
        }
    })
    .take(100)
    .collect()
}

fn schnorrkel_benchmark(c: &mut Criterion) {
    let input = generate_input();

    let mut group = c.benchmark_group("steateless");
    group.bench_with_input("sign", &input, |b, input| {
        b.iter(|| {
            for item in input {
                let context = signing_context(&item.context_bytes);
                let seckey = SecretKey::from_bytes(&item.seckey_bytes).unwrap();
                let pubkey = PublicKey::from_bytes(&item.pubkey_bytes).unwrap();
                let _signature = seckey.sign(context.bytes(&item.messages[0]), &pubkey);
            }
        })
    });
    group.bench_with_input("verify", &input, |b, input| {
        b.iter(|| {
            for item in input {
                let context = signing_context(&item.context_bytes);
                let pubkey = PublicKey::from_bytes(&item.pubkey_bytes).unwrap();
                let signature = Signature::from_bytes(&item.signatures_bytes[0]).unwrap();
                pubkey
                    .verify(context.bytes(&item.messages[0]), &signature)
                    .unwrap();
            }
        })
    });
    group.finish();

    let mut group = c.benchmark_group("stateful");
    group.bench_with_input("sign", &input, |b, input| {
        b.iter(|| {
            for item in input.iter().take(10) {
                for message in item.messages.iter() {
                    let _signature = item.seckey.sign(item.context.bytes(message), &item.pubkey);
                }
            }
        })
    });
    group.bench_with_input("verify", &input, |b, input| {
        b.iter(|| {
            for item in input.iter().take(10) {
                for (message, signature) in item.messages.iter().zip(item.signatures.iter()) {
                    item.pubkey
                        .verify(item.context.bytes(message), signature)
                        .unwrap()
                }
            }
        })
    });
    group.finish();
}

criterion_group!(benches, schnorrkel_benchmark);
criterion_main!(benches);
