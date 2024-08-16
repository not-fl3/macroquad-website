# Macroquad Website

https://macroquad.rs/

The Macroquad website is built with the Zola static site generator and uses a heavily modified version of the AkiDocs theme ([which is MIT licensed](https://github.com/aaranxu/adidoks/blob/main/LICENSE)). The main difference in the Macroquad website theme from AkiDocs is that semantic CSS is used instead of Bootstrap.

## Developing

1. Fork & clone the repository
2. Download [Zola](https://github.com/getzola/zola/releases), using the version specified in [`.github/worksflows/ci.yaml`](https://github.com/not-fl3/macroquad-website/blob/source/.github/workflows/ci.yaml)
3. Run `zola serve` to start up a local web server for the site
4. Make changes develop locally
