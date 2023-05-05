pub mod posts;

markup::define! {
  TemplateHead<'a>(title: &'a str) {
    head {
      title { "Chris Miller - " @title }
      style {
          "body { background: #fafbfc; }"
          "#main { padding: 2rem; }"
      }
    }
  }
}
