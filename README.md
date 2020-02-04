# Rust

Ce projet permet de traiter une image PPM (P3) qui sera nommée **output.ppm**
Il utilise une version modifiée de ppma_io afin de traiter l'image.

## Commandes pour compiler le Rust

cargo +nightly build

## Commandes pour lancer les benchmark Rust

```bash
cargo +nightly bench
```

## Commandes pour lancer le python (v2.7)

```bash
python runner.py
```

| args | description |
| ----------- | ----------- |
| *none* | apply gray by default |
| gray | turn the ppm file to gray |
| reverted | invert the ppm file |
