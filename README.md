### JSON Resume Renderer [WIP]

This is a binary that takes in a JSON following the [JSON Resume schema](https://jsonresume.org/schema/) and a template and uses [Tera](https://tera.netlify.app/) to generate a document.

This is WIP. Couple of things needed before general usage:

- [ ] Be able to take in any template. Currently my template to generate a LaTex file is hardcoded.
- [ ] Include all fields of JSON Resume Schema. Currently it only takes in the fields needed for the above template.

#### Sample Usage

```bash
$ cargo install jsonresume-renderer
$ jsonresume-renderer my-json-resume.json output.txt
```