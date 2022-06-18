### JSON Resume Renderer [WIP]

This is a binary that takes in a JSON following the [JSON Resume schema](https://jsonresume.org/schema/) and a template and uses [Tera](https://tera.netlify.app/) to generate a document.

This is WIP. Couple of things needed before general usage:

- [x] Be able to take in any template. Currently, my template to generate a LaTex file is hardcoded.
- [x] Include all fields of JSON Resume Schema. Currently, it only takes in a subset of the fields.
- [x] Make args CLI-like.
- [ ] Add template directory support, as opposed to a one-off template.
- [ ] Better error messaging.

#### Getting Started

`jsonresume-renderer` is available via [Cargo](https://crates.io/crates/jsonresume-renderer):

```bash
$ cargo install jsonresume-renderer
$ jsonresume-renderer --help
```

#### Sample Usage

```bash
$ jsonresume-renderer -j my-json-resume.json -t my-template.txt -o output.txt
```

#### Example Template

I use this repo to generate a LaTex document of my resume. You can check out the template file in my [resume repo](https://github.com/gapuchi/resume/blob/main/templates/resume.tex).
